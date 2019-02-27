use crate::parser::{MessageType, XtFile};
use jens::Block;

use jens_derive::Template;

#[derive(Template)]
#[filename = "writers/rust.jens"]
struct Template {}

mod gen {
    use super::Template;
    use crate::parser::{EnumVariant, Message, StructField, Tuple, TypeName};
    use jens::Block;

    pub fn tuple_types(v: Tuple) -> Block {
        Block::from(v.0.join(","))
    }
    pub fn variant(v: EnumVariant) -> Block {
        match v.content {
            None => Template::variant(v.name),
            Some(content) => Template::variant_with_content(v.name, tuple_types(content)),
        }
    }

    pub fn type_name(t: &TypeName) -> Block {
        match t {
            TypeName::Concrete(s) => Block::from(s.clone()),
            TypeName::Generic(s, g) => Template::generic(Block::from(s.clone()), type_name(g)),
        }
    }

    pub fn struct_field(field: StructField) -> Block {
        Block::from(if field.is_optional {
            format!(
                "pub {}: Option<{}>,",
                field.name,
                type_name(&field.type_name)
            )
        } else {
            format!("pub {}: {},", field.name, type_name(&field.type_name))
        })
    }

    pub fn docblock(msg: &Message) -> Block {
        match msg.attr("doc") {
            None => Block::empty(),
            Some(v) => Template::docblock(v),
        }
    }
}

pub fn write_defs(file: XtFile) -> String {
    let output = Template::main(Block::join_map(file.messages, |m, _| match m.value {
        MessageType::Enum(v) => {
            Template::decl_tagged_union(m.name, Block::join_map(v.variants, |v, _| gen::variant(v)))
        }
        MessageType::Struct(s) => {
            Template::decl_struct("T", Block::join_map(s.fields, |f, _| gen::struct_field(f)))
        }
    }));
    format!("{}", output)
}
