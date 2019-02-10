use crate::parser::{EnumMessage, MessageType, StructMessage, TypeName, XtFile};
use jens::{Block, File};

pub fn make_variants(t: &File, v: EnumMessage) -> Block {
    Block::join_map(v.variants, |variant, _| {
        t.template("variant").set("name", variant.name)
    })
}

pub fn make_type_name(v: &TypeName) -> Block {
    match v {
        TypeName::Concrete(s) => Block::from(s.clone()),
        TypeName::Generic(s, g) => Block::from(format!("{}<{}>", s, make_type_name(g))),
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

pub fn write_defs(file: XtFile) -> String {
    let t = File::parse(include_str!("./typescript.jens")).unwrap();

    let output = t.template("main").set(
        "messages",
        Block::join_map(file.messages, |m, _| {
            t.template("namespace").set("name", m.name).set(
                "content",
                match m.value {
                    MessageType::Enum(v) => t
                        .template("tagged_union")
                        .set("name", "T")
                        .set("variants", make_variants(&t, v)),
                    MessageType::Struct(v) => t
                        .template("struct")
                        .set("name", "T")
                        .set("fields", make_fields(v)),
                },
            )
        }),
    );
    format!("{}", output)
}
