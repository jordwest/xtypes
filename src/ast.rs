/// An attribute is a special flag that can be attached to:
///  - A message
///  - A field
///  - An enum variant
///  - A module
///  - An import
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

#[derive(Debug, PartialEq)]
pub struct ModuleUse {
    pub attrs: Vec<Attribute>,
    pub path: String,
}

#[derive(Debug, PartialEq)]
pub struct ModuleImport {
    pub path: String,
    pub attrs: Vec<Attribute>,
}

#[derive(Debug, PartialEq)]
pub enum TypeName {
    Concrete(String),
    Generic(String, Box<TypeName>),
}

#[derive(Debug, PartialEq)]
pub struct XtFile {
    pub module_info: ModuleInfo,
    pub imports: Vec<ModuleImport>,
    pub use_imports: Vec<ModuleUse>,
    pub messages: Vec<Message>,
}
