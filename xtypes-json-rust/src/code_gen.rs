use xtypes::ast::{MessageType, SymbolType};
use xtypes::module_loader::ModuleScope;
use jens::Block;

use jens_derive::Template;

#[derive(Template)]
#[filename = "rust.jens"]
struct Template {}

mod gen {
    use super::Template;
    use xtypes::ast::{EnumVariant, StructField, SymbolDefinition, Tuple, TypeName};
    use jens::Block;

    pub fn tuple_types(v: &Tuple) -> Block {
        Block::from(v.0.join(","))
    }
    pub fn variant(v: &EnumVariant) -> Block {
        match &v.content {
            None => Template::variant(v.name.clone()),
            Some(content) => Template::variant_with_content(v.name.clone(), tuple_types(&content)),
        }
    }

    pub fn type_name(t: &TypeName) -> Block {
        match t {
            TypeName::Concrete(s) => Block::from(s.clone()),
            TypeName::Generic(s, g) => Template::generic(Block::from(s.clone()), type_name(g)),
        }
    }

    pub fn struct_field(field: &StructField) -> Block {
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

    pub fn docblock(msg: &SymbolDefinition) -> Block {
        match msg.attrs.get("doc") {
            None => Block::empty(),
            Some(v) => Template::docblock(v),
        }
    }
}

pub fn write_defs(scope: ModuleScope) -> String {
    let output = Template::main(Block::join_map(scope.module.symbols, |m, _| match &m.value {
        SymbolType::Primitive => Block::empty(),
        SymbolType::Message(MessageType::Enum(v)) => Template::decl_tagged_union(
            gen::docblock(&m),
            Block::from(m.name),
            Block::join_map(&v.variants, |v, _| gen::variant(v)),
        ),
        SymbolType::Message(MessageType::Struct(s)) => Template::decl_struct(
            gen::docblock(&m),
            Block::from(m.name),
            Block::join_map(&s.fields, |f, _| gen::struct_field(f)),
        ),
    }));
    format!("{}", output)
}
