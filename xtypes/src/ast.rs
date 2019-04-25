use std::collections::BTreeMap;

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
#[derive(Clone, Debug, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AttributeList(BTreeMap<String, Attribute>);

impl AttributeList {
    pub fn new() -> Self {
        AttributeList(BTreeMap::new())
    }

    pub fn add(&mut self, attr: Attribute) {
        self.0.insert(attr.name.clone(), attr);
    }

    pub fn get<T: AsRef<str> + Sized>(&self, key: T) -> Option<String> {
        self.0.get(key.as_ref()).map(|a| a.value.clone())
    }
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
#[derive(Clone, Debug, PartialEq)]
pub struct Tuple(pub Vec<String>);

#[derive(Clone, Debug, PartialEq)]
pub struct EnumMessage {
    pub variants: Vec<EnumVariant>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnumVariant {
    pub name: String,
    pub attrs: AttributeList,
    pub content: Option<Tuple>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StructField {
    pub name: String,
    pub type_name: TypeName,
    pub is_optional: bool,
    pub attrs: AttributeList,
}
#[derive(Clone, Debug, PartialEq)]
pub struct StructMessage {
    pub fields: Vec<StructField>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MessageType {
    Enum(EnumMessage),
    Struct(StructMessage),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SymbolType {
    Message(MessageType),
    Primitive,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SymbolDefinition {
    pub name: TypeName,
    pub attrs: AttributeList,
    pub value: SymbolType,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleInfo {
    pub name: String,
    pub attrs: AttributeList,
}

/// A portion of a [DottedIdent](xtypes::ast::DottedIdent).
#[derive(Clone, Debug, PartialEq)]
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
#[derive(Clone, Debug, PartialEq)]
pub struct DottedIdent {
    pub parts: Vec<DottedIdentPart>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum IdentOrWildcard {
    Ident(String),
    Wildcard,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleUse {
    pub attrs: AttributeList,
    pub filename: String,
    pub ident: IdentOrWildcard,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeName {
    Concrete(String),
    Generic(String, Box<TypeName>),
}
impl Into<String> for &TypeName {
    fn into(self) -> String {
        match &self {
            TypeName::Concrete(s) => s.clone(),
            TypeName::Generic(s, _) => s.clone(),
        }
    }
}
impl Into<String> for TypeName {
    fn into(self) -> String {
        (&self).into()
    }
}
impl TypeName {
    pub fn identifier(&self) -> String {
        match self {
            TypeName::Concrete(s) => s.clone(),
            TypeName::Generic(s, _) => s.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct XtFile {
    pub module_info: ModuleInfo,
    pub use_imports: Vec<ModuleUse>,
    pub symbols: Vec<SymbolDefinition>,
}
