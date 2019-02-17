use crate::parser::{EnumMessage, Message, MessageType, StructMessage, Tuple, TypeName, XtFile};
use jens::Block;

use jens_derive::Template;

#[derive(Template)]
#[filename = "writers/typescript.jens"]
struct Template {}

pub fn make_tuple_type(v: Tuple) -> Block {
    Block::from(format!("[{}]", v.0.join(", ")))
}

pub fn make_variants(v: EnumMessage) -> Block {
    Block::join_map(v.variants, |variant, _| match variant.content {
        None => Template::variant(variant.name),
        Some(content) => Template::variant_with_content(variant.name, make_tuple_type(content)),
    })
}

pub fn make_type_name(v: &TypeName) -> Block {
    match v {
        TypeName::Concrete(s) => Block::from(s.clone()),
        TypeName::Generic(s, g) => Template::generic(s.clone(), make_type_name(g)),
    }
}

pub fn make_fields(v: StructMessage) -> Block {
    Block::join_map(v.fields, |field, _| {
        Block::from(format!(
            "{}{}: {}",
            field.name,
            if field.is_optional { "?" } else { "" },
            make_type_name(&field.type_name)
        ))
    })
}

fn make_doc_block(msg: &Message) -> Block {
    match msg.attr("doc") {
        None => Block::empty(),
        Some(v) => Template::docblock(v),
    }
}

pub fn write_defs(file: XtFile) -> String {
    let output = Template::main(Block::join_map(file.messages, |m, _| {
        Template::namespace(
            make_doc_block(&m),
            m.name,
            match m.value {
                MessageType::Enum(v) => Template::decl_tagged_union("T", make_variants(v)),
                MessageType::Struct(v) => Template::decl_struct("T", make_fields(v)),
            },
        )
    }));
    format!("{}", output)
}
