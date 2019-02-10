fn main() {
    println!("Hello, world!");
}

mod xt {
    use pest::iterators::Pair;
    use pest::Parser;
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "grammar.pest"]
    pub struct XtParser;

    #[derive(Debug, PartialEq)]
    pub struct Attribute {
        pub name: String,
        pub value: String,
    }
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

    #[derive(Debug, PartialEq)]
    pub struct EnumVariant {
        pub name: String,
        pub attrs: Vec<Attribute>,
    }
    impl From<Pair<'_, Rule>> for EnumVariant {
        fn from(pair: Pair<'_, Rule>) -> EnumVariant {
            match pair.as_rule() {
                Rule::variant => {
                    let mut name = String::new();
                    let mut attrs = Vec::new();
                    for pair in pair.into_inner() {
                        match pair.as_rule() {
                            Rule::ident => name = pair.as_str().into(),
                            Rule::attribute => attrs.push(pair.into()),
                            _ => panic!(),
                        }
                    }
                    EnumVariant { name, attrs }
                }
                _ => panic!(),
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct EnumMessage {
        pub variants: Vec<EnumVariant>,
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

    #[derive(Debug, PartialEq)]
    pub struct StructField {
        pub name: String,
        pub type_name: String,
        pub is_optional: bool,
        pub attrs: Vec<Attribute>,
    }
    impl From<Pair<'_, Rule>> for StructField {
        fn from(pair: Pair<'_, Rule>) -> StructField {
            match pair.as_rule() {
                Rule::struct_field => {
                    let mut name = None;
                    let mut type_name = None;
                    let mut is_optional = false;
                    let mut attrs = Vec::new();
                    for pair in pair.into_inner() {
                        match pair.as_rule() {
                            Rule::field_name => name = Some(pair.as_str().into()),
                            Rule::field_type => type_name = Some(pair.as_str().into()),
                            Rule::attribute => attrs.push(pair.into()),
                            Rule::optional => is_optional = true,
                            rule => panic!("Unexpected rule {}", pair.as_str()),
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
    #[derive(Debug, PartialEq)]
    pub struct StructMessage {
        pub fields: Vec<StructField>,
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

    #[derive(Debug, PartialEq)]
    pub enum MessageType {
        Enum(EnumMessage),
        Struct(StructMessage),
    }
    impl From<Pair<'_, Rule>> for MessageType {
        fn from(pair: Pair<'_, Rule>) -> MessageType {
            match pair.as_rule() {
                Rule::enum_message => MessageType::Enum(pair.into()),
                Rule::struct_message => MessageType::Struct(pair.into()),
                _ => panic!("Unexpected message type"),
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct Message {
        pub name: String,
        pub value: MessageType,
    }
    impl From<Pair<'_, Rule>> for Message {
        fn from(pair: Pair<'_, Rule>) -> Message {
            match pair.as_rule() {
                Rule::message => {
                    let mut inner_pairs = pair.clone().into_inner();
                    let ident = inner_pairs.next().unwrap();
                    let name = match ident.as_rule() {
                        Rule::ident => ident.as_str(),
                        _ => panic!("Expected message name, found {}", pair.clone()),
                    };
                    let value = inner_pairs.next().unwrap();
                    Message {
                        name: name.into(),
                        value: value.into(),
                    }
                }
                unknown => panic!("Unexpected rule '{:?}' found ", unknown),
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct ModuleInfo {
        pub name: String,
        pub attrs: Vec<Attribute>,
    }
    impl From<Pair<'_, Rule>> for ModuleInfo {
        fn from(pair: Pair<'_, Rule>) -> ModuleInfo {
            match pair.as_rule() {
                Rule::module_decl => {
                    let mut attrs = vec![];
                    let mut name = "";
                    for pair in pair.into_inner() {
                        match pair.as_rule() {
                            Rule::dotted_ident => name = pair.as_str(),
                            Rule::attribute => attrs.push(pair.into()),
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

    #[derive(Debug, PartialEq)]
    pub struct File {
        pub module_info: ModuleInfo,
        pub messages: Vec<Message>,
    }
    impl From<Pair<'_, Rule>> for File {
        fn from(pair: Pair<'_, Rule>) -> File {
            match pair.as_rule() {
                Rule::file => {
                    let mut messages = vec![];
                    let mut module_info = None;
                    for pair in pair.into_inner() {
                        match pair.as_rule() {
                            Rule::module_decl => module_info = Some(pair.into()),
                            Rule::message => messages.push(pair.into()),
                            Rule::EOI => (),
                            _ => panic!("Unexpected '{:?}'", pair),
                        }
                    }
                    File {
                        module_info: module_info.unwrap(),
                        messages: messages,
                    }
                }
                _ => panic!(),
            }
        }
    }

    pub fn parse(t: &str) -> File {
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
}
