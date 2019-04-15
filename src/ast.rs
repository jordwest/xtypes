/// An attribute is a special flag that can be attached to:
///  - A message
///  - A field
///  - An enum variant
///  - A module
///  - A use statement
///
/// Attributes can be used by the code generator in any way
/// they're needed.
///
/// For example, these attributes might tell a code
/// generator to name the fields differently when
/// doing JSON serialization:
///
/// ```xt
/// message Book = {
///   #[json.key="book_name"]
///   name: string,
///
///   #[json.key="author_name"]
///   author: string,
/// }
/// ```
#[derive(Debug, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

/// A tuple can contain multiple types in a sequence.
///
/// For example:
///
/// ```xt
/// (string, float32)
/// ```
///
/// This is useful in the context of an enum:
///
/// ```xt
/// message OneOrTwoNumbers =
///     | One(float32)
///     | Two(float32, float32)
/// ;
/// ```
#[derive(Debug, PartialEq)]
pub struct Tuple(pub Vec<String>);

#[derive(Debug, PartialEq)]
pub struct EnumMessage {
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, PartialEq)]
pub struct EnumVariant {
    pub name: String,
    pub attrs: Vec<Attribute>,
    pub content: Option<Tuple>,
}

#[derive(Debug, PartialEq)]
pub struct StructField {
    pub name: String,
    pub type_name: TypeName,
    pub is_optional: bool,
    pub attrs: Vec<Attribute>,
}
#[derive(Debug, PartialEq)]
pub struct StructMessage {
    pub fields: Vec<StructField>,
}

#[derive(Debug, PartialEq)]
pub enum MessageType {
    Enum(EnumMessage),
    Struct(StructMessage),
}

#[derive(Debug, PartialEq)]
pub struct Message {
    pub name: String,
    pub attrs: Vec<Attribute>,
    pub value: MessageType,
}

#[derive(Debug, PartialEq)]
pub struct ModuleInfo {
    pub name: String,
    pub attrs: Vec<Attribute>,
}

/// A portion of a [DottedIdent](xtypes::ast::DottedIdent).
#[derive(Debug, PartialEq)]
pub enum DottedIdentPart {
    Ident(String),
    Wildcard,
}

/// A DottedIdent refers to a dotted variable name. For example:
///
/// ```xt
/// use SomeModule.SubModule.*;
/// //  ^^^^^^^^^^^^^^^^^^^^^^
/// ```
///
/// The above statement would contain 3 "parts":
///   - Ident("SomeModule")
///   - Ident("SubModule")
///   - Wildcard
#[derive(Debug, PartialEq)]
pub struct DottedIdent {
    pub parts: Vec<DottedIdentPart>,
}

#[derive(Debug, PartialEq)]
pub enum IdentOrWildcard {
    Ident(String),
    Wildcard,
}

#[derive(Debug, PartialEq)]
pub struct ModuleUse {
    pub attrs: Vec<Attribute>,
    pub filename: String,
    pub ident: IdentOrWildcard,
}

#[derive(Debug, PartialEq)]
pub enum TypeName {
    Concrete(String),
    Generic(String, Box<TypeName>),
}

#[derive(Debug, PartialEq)]
pub struct XtFile {
    pub module_info: ModuleInfo,
    pub use_imports: Vec<ModuleUse>,
    pub messages: Vec<Message>,
}
