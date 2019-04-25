use xtypes::ast::*;
use xtypes::module_loader::ModuleScope;
use jens::Block;
use jens_derive::Template;

#[derive(Template)]
#[filename = "typescript.jens"]
struct Template {}

mod gen {
    use super::Template;
    use xtypes::ast::*;
    use xtypes::module_loader::ModuleScope;
    use jens::Block;

    pub fn tuple_type(v: &Tuple) -> Block {
        Block::from(format!("[{}]", v.0.join(", ")))
    }

    pub fn variant(variant: &EnumVariant) -> Block {
        match &variant.content {
            None => Template::variant(variant.name.clone()),
            Some(content) => {
                Template::variant_with_content(variant.name.clone(), tuple_type(content))
            }
        }
    }

    pub fn struct_field(scope: &ModuleScope, field: &StructField) -> Block {
        Block::from(format!(
            "{}{}: {}",
            field.name,
            if field.is_optional { "?" } else { "" },
            type_name(scope, &field.type_name)
        ))
    }

    pub fn type_name(scope: &ModuleScope, v: &TypeName) -> Block {
        match v {
            TypeName::Concrete(s) => {
                let scope_item = scope.symbol_map.get(&v.identifier()).expect(s);
                match scope_item.symbol.value {
                    SymbolType::Primitive => {
                        Block::from(scope_item.symbol.attrs.get("js.type").unwrap())
                    }
                    SymbolType::Message(_) => Block::from(format!("{}.T", s)),
                }
                // Template::dot_t(Block::from(s.clone())),
            }
            TypeName::Generic(s, g) => match s {
                s if s == &String::from("Array") => Template::array_type(type_name(scope, g)),
                s => Template::generic(Template::dot_t(s.clone()), type_name(scope, g)),
            },
        }
    }

    pub fn docblock(msg: &SymbolDefinition) -> Block {
        match msg.attrs.get("doc") {
            None => Block::empty(),
            Some(v) => Template::docblock(v),
        }
    }

    pub fn import(import: &ModuleUse) -> Block {
        match &import.ident {
            IdentOrWildcard::Wildcard => {
                Block::from(format!("import * from \"{}.ts\"", import.filename))
            }
            IdentOrWildcard::Ident(s) => {
                Block::from(format!("import * as {} from \"{}.ts\"", s, import.filename))
            }
        }
    }
}

pub fn write_defs(scope: ModuleScope) -> String {
    let output = Template::main(
        Block::join_map(&scope.module.use_imports, |i, _| gen::import(&i)),
        Block::join_map(&scope.module.symbols, |m, _| {
            Template::namespace(
                gen::docblock(&m),
                m.name.clone(),
                match &m.value {
                    SymbolType::Primitive => Block::empty(),
                    SymbolType::Message(MessageType::Enum(ref v)) => Template::decl_tagged_union(
                        "T",
                        Block::join_map(&v.variants, |v, _| gen::variant(v)),
                    ),
                    SymbolType::Message(MessageType::Struct(s)) => Template::decl_struct(
                        "T",
                        Block::join_map(&s.fields, |f, _| gen::struct_field(&scope, f)),
                    ),
                },
            )
        }),
    );
    format!("{}", output)
}
