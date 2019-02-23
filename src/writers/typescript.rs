use crate::ast::*;
use jens::Block;

use jens_derive::Template;

#[derive(Template)]
#[filename = "writers/typescript.jens"]
struct Template {}

mod gen {
    use super::Template;
    use crate::ast::*;
    use jens::Block;

    pub fn tuple_type(v: Tuple) -> Block {
        Block::from(format!("[{}]", v.0.join(", ")))
    }

    pub fn variant(variant: EnumVariant) -> Block {
        match variant.content {
            None => Template::variant(variant.name),
            Some(content) => Template::variant_with_content(variant.name, tuple_type(content)),
        }
    }

    pub fn struct_field(field: StructField) -> Block {
        Block::from(format!(
            "{}{}: {}",
            field.name,
            if field.is_optional { "?" } else { "" },
            type_name(&field.type_name)
        ))
    }

    pub fn type_name(v: &TypeName) -> Block {
        match v {
            TypeName::Concrete(s) => Template::dot_t(Block::from(s.clone())),
            TypeName::Generic(s, g) => match s {
                s if s == &String::from("Array") => Template::array_type(type_name(g)),
                s => Template::generic(Template::dot_t(s.clone()), type_name(g)),
            },
        }
    }

    pub fn docblock(msg: &Message) -> Block {
        match msg.attr("doc") {
            None => Block::empty(),
            Some(v) => Template::docblock(v),
        }
    }
}

pub fn write_defs(file: XtFile) -> String {
    let output = Template::main(Block::join_map(file.messages, |m, _| {
        Template::namespace(
            gen::docblock(&m),
            m.name,
            match m.value {
                MessageType::Enum(v) => Template::decl_tagged_union(
                    "T",
                    Block::join_map(v.variants, |v, _| gen::variant(v)),
                ),
                MessageType::Struct(s) => Template::decl_struct(
                    "T",
                    Block::join_map(s.fields, |f, _| gen::struct_field(f)),
                ),
            },
        )
    }));
    format!("{}", output)
}
