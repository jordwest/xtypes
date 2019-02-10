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
    pub enum MessageType {
        Enum(EnumMessage),
        // Struct,
    }
    impl From<Pair<'_, Rule>> for MessageType {
        fn from(pair: Pair<'_, Rule>) -> MessageType {
            match pair.as_rule() {
                Rule::enum_message => MessageType::Enum(pair.into()),
                // Rule::struct_message => MessageType::Struct(pair.into())
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
    pub struct File {
        pub messages: Vec<Message>,
    }

    pub fn parse(t: &str) -> File {
        match XtParser::parse(Rule::message, t) {
            Err(e) => panic!("{}", e),
            Ok(v) => File {
                messages: v.into_iter().map(Message::from).collect(),
            },
        }
    }

    #[test]
    fn test_parse() {
        let file =
            parse("message EnumSample = \n\t| #[attr.one = \"A\"] One \n\t| Two \n\t| Three;");
        let expected = File {
            messages: vec![Message {
                name: "EnumSample".into(),
                value: MessageType::Enum(EnumMessage {
                    variants: vec![
                        EnumVariant {
                            name: "One".into(),
                            attrs: vec![Attribute {
                                name: "attr.one".into(),
                                value: "A".into(),
                            }],
                        },
                        EnumVariant {
                            name: "Two".into(),
                            attrs: vec![],
                        },
                        EnumVariant {
                            name: "Three".into(),
                            attrs: vec![],
                        },
                    ],
                }),
            }],
        };
        assert_eq!(expected, file);
    }

    // named!(parse_enum_variant<&str, EnumVariant>,
    //     do_parse!(
    //         name: alphanumeric >>
    //         (EnumVariant {name: name.into()})
    //     )
    // );

    // named!(pub parse_enum<&str, MessageType>,
    //     do_parse!(
    //         opt!(complete!(tag!("|"))) >>
    //         first: parse_enum_variant >>
    //         remaining: many0!(preceded!(tag!("|"), parse_enum_variant)) >>
    //         (MessageType::Enum(EnumMessage {variants: remaining}))
    //     )
    // );

    // named!(parse_message<&str, Message>,
    //     do_parse!(
    //         tag!("message") >>
    //         space1 >>
    //         message_name: alphanumeric >>
    //         tag!("=") >>
    //         message_value: alt!(parse_enum) >>
    //         tag!(";") >>
    //         (Message {name: message_name.into(), value: message_value})
    //     )
    // );
    // named!(pub parse_xt<&str, File>,
    //     do_parse!(
    //         messages: many1!(ws!(parse_message)) >>
    //         (File {messages: messages})
    //     )
    // );
}

// #[test]
// fn test_enum_value() {
//     let (_, result) = xt::parse_enum("|One|Two|Three;").unwrap();

//     use xt::*;
//     let expected = MessageType::Enum(EnumMessage {
//         variants: vec![
//             EnumVariant { name: "One".into() },
//             EnumVariant { name: "Two".into() },
//             EnumVariant {
//                 name: "Three".into(),
//             },
//         ],
//     });

//     assert_eq!(expected, result);
// }

// #[test]
// fn test_enum() {
//     let (_, result) = xt::parse_xt("message EnumSample=One|Two|Three;").unwrap();

//     use xt::*;
//     let expected = File {
//         messages: vec![Message {
//             name: "EnumSample".into(),
//             value: MessageType::Enum(EnumMessage {
//                 variants: vec![
//                     EnumVariant { name: "One".into() },
//                     EnumVariant { name: "Two".into() },
//                     EnumVariant {
//                         name: "Three".into(),
//                     },
//                 ],
//             }),
//         }],
//     };

//     assert_eq!(expected, result);
// }
