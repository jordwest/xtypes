use crate::ast::*;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

/// The parser takes a `.xt` file and parses it into an AST

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct XtParser;

impl From<Pair<'_, Rule>> for Attribute {
    fn from(pair: Pair<'_, Rule>) -> Attribute {
        match pair.as_rule() {
            Rule::attribute => {
                let mut iter = pair.into_inner().into_iter();
                Attribute {
                    name: iter.next().unwrap().as_str().into(),
                    value: iter.next().unwrap().as_str().into(),
                }
            }
            _ => panic!(),
        }
    }
}

impl From<Pair<'_, Rule>> for Tuple {
    fn from(pair: Pair<'_, Rule>) -> Tuple {
        let mut types = vec![];
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::dotted_ident => types.push(pair.as_str().into()),
                _ => panic!(),
            }
        }
        Tuple(types)
    }
}
impl From<Pair<'_, Rule>> for EnumVariant {
    fn from(pair: Pair<'_, Rule>) -> EnumVariant {
        match pair.as_rule() {
            Rule::variant => {
                let mut name = String::new();
                let mut attrs = AttributeList::new();
                let mut content = None;
                for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::ident => name = pair.as_str().into(),
                        Rule::attribute => attrs.add(pair.into()),
                        Rule::tuple => content = Some(pair.into()),
                        _ => panic!(),
                    }
                }
                EnumVariant {
                    name,
                    attrs,
                    content,
                }
            }
            _ => panic!(),
        }
    }
}

impl From<Pair<'_, Rule>> for EnumMessage {
    fn from(pair: Pair<'_, Rule>) -> EnumMessage {
        match pair.as_rule() {
            Rule::enum_message => EnumMessage {
                variants: pair.into_inner().into_iter().map(|i| i.into()).collect(),
            },
            _ => panic!(),
        }
    }
}

impl From<Pair<'_, Rule>> for TypeName {
    fn from(pair: Pair<'_, Rule>) -> TypeName {
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::dotted_ident => TypeName::Concrete(pair.as_str().into()),
            Rule::generic_type => {
                let mut outside_type = None;
                let mut inside_type = None;
                for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::ident => outside_type = Some(pair.as_str().into()),
                        Rule::type_name => inside_type = Some(pair.into()),
                        _ => panic!(),
                    }
                }
                TypeName::Generic(outside_type.unwrap(), Box::new(inside_type.unwrap()))
            }
            _ => panic!(),
        }
    }
}
impl From<Pair<'_, Rule>> for StructField {
    fn from(pair: Pair<'_, Rule>) -> StructField {
        match pair.as_rule() {
            Rule::struct_field => {
                let mut name = None;
                let mut type_name = None;
                let mut is_optional = false;
                let mut attrs = AttributeList::new();
                for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::field_name => name = Some(pair.as_str().into()),
                        Rule::type_name => type_name = Some(pair.into()),
                        Rule::attribute => attrs.add(pair.into()),
                        Rule::optional => is_optional = true,
                        _ => panic!("Unexpected rule {}", pair.as_str()),
                    }
                }
                StructField {
                    name: name.unwrap(),
                    type_name: type_name.unwrap(),
                    is_optional,
                    attrs,
                }
            }
            _ => panic!(),
        }
    }
}

impl From<Pair<'_, Rule>> for StructMessage {
    fn from(pair: Pair<'_, Rule>) -> StructMessage {
        match pair.as_rule() {
            Rule::struct_message => StructMessage {
                fields: pair.into_inner().into_iter().map(|i| i.into()).collect(),
            },
            _ => panic!(),
        }
    }
}
impl From<Pair<'_, Rule>> for MessageType {
    fn from(pair: Pair<'_, Rule>) -> MessageType {
        match pair.as_rule() {
            Rule::enum_message => MessageType::Enum(pair.into()),
            Rule::struct_message => MessageType::Struct(pair.into()),
            r => panic!("Unexpected message type {:?}", r),
        }
    }
}

impl From<Pair<'_, Rule>> for SymbolDefinition {
    fn from(pair: Pair<'_, Rule>) -> SymbolDefinition {
        let mut name: Option<TypeName> = None;
        let mut value: Option<MessageType> = None;
        let mut attrs = AttributeList::new();
        let rule = pair.as_rule();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::type_name => name = Some(pair.into()),
                Rule::attribute => attrs.add(pair.into()),
                _ => value = Some(pair.into()),
            }
        }
        match rule {
            Rule::typedef => SymbolDefinition {
                name: name.unwrap().into(),
                attrs,
                value: SymbolType::Primitive,
            },
            Rule::message => SymbolDefinition {
                name: name.unwrap().into(),
                attrs,
                value: SymbolType::Message(value.unwrap()),
            },
            unknown => panic!("Unexpected rule '{:?}' found ", unknown),
        }
    }
}
impl From<Pair<'_, Rule>> for ModuleInfo {
    fn from(pair: Pair<'_, Rule>) -> ModuleInfo {
        match pair.as_rule() {
            Rule::module_decl => {
                let mut attrs = AttributeList::new();
                let mut name = "";
                for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::dotted_ident => name = pair.as_str(),
                        Rule::attribute => attrs.add(pair.into()),
                        _ => panic!(),
                    }
                }
                ModuleInfo {
                    name: name.into(),
                    attrs,
                }
            }
            _ => panic!(),
        }
    }
}

impl From<Pair<'_, Rule>> for DottedIdent {
    fn from(pair: Pair<'_, Rule>) -> DottedIdent {
        match pair.as_rule() {
            Rule::dotted_ident => {
                let mut parts = vec![];
                println!("Outer: {}", pair.as_str());
                for pair in pair.into_inner() {
                    println!("Pair: {}", pair.as_str());
                    match pair.as_rule() {
                        Rule::ident => parts.push(DottedIdentPart::Ident(pair.as_str().into())),
                        Rule::wildcard => parts.push(DottedIdentPart::Wildcard),
                        _ => panic!("Unexpected rule"),
                    }
                }
                DottedIdent { parts }
            }
            unknown => panic!("Unexpected rule '{:?}' found ", unknown),
        }
    }
}

impl From<Pair<'_, Rule>> for IdentOrWildcard {
    fn from(pair: Pair<'_, Rule>) -> IdentOrWildcard {
        match pair.as_rule() {
            Rule::ident => IdentOrWildcard::Ident(pair.as_str().to_owned()),
            Rule::wildcard => IdentOrWildcard::Wildcard,
            r => panic!("Unexpected rule {:?}", r),
        }
    }
}

impl From<Pair<'_, Rule>> for ModuleUse {
    fn from(pair: Pair<'_, Rule>) -> ModuleUse {
        match pair.as_rule() {
            Rule::use_statement => {
                let mut ident = None;
                let mut filename = None;
                let mut attrs = AttributeList::new();
                for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::ident => {
                            ident = Some(IdentOrWildcard::Ident(pair.as_str().to_owned()))
                        }
                        Rule::wildcard => ident = Some(IdentOrWildcard::Wildcard),
                        Rule::filename => filename = Some(pair.as_str()),
                        Rule::attribute => attrs.add(pair.into()),
                        r => panic!("Unexpected rule {:?}", r),
                    }
                }
                ModuleUse {
                    attrs,
                    ident: ident.unwrap(),
                    filename: filename.unwrap().to_owned(),
                }
            }
            unknown => panic!("Unexpected rule '{:?}' found ", unknown),
        }
    }
}

impl From<Pair<'_, Rule>> for XtFile {
    fn from(pair: Pair<'_, Rule>) -> XtFile {
        match pair.as_rule() {
            Rule::file => {
                let mut symbols = vec![];
                let mut use_imports = vec![];
                let mut module_info = None;
                for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::module_decl => module_info = Some(pair.into()),
                        Rule::message => symbols.push(pair.into()),
                        Rule::use_statement => use_imports.push(pair.into()),
                        Rule::typedef => symbols.push(pair.into()),
                        Rule::EOI => (),
                        _ => panic!("Unexpected '{:?}'", pair),
                    }
                }
                XtFile {
                    module_info: module_info.unwrap(),
                    symbols,
                    use_imports,
                }
            }
            _ => panic!(),
        }
    }
}

pub fn parse(t: &str) -> XtFile {
    match XtParser::parse(Rule::file, t) {
        Err(e) => panic!("{}", e),
        Ok(v) => v.into_iter().next().unwrap().into(),
    }
}

#[test]
fn test_parse_sample_1() {
    use insta::assert_debug_snapshot_matches;

    let file = parse(include_str!("./sample.xt"));
    assert_debug_snapshot_matches!("sample.xt", file);
}
