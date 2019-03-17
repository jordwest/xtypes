use crate::ast::*;
use crate::parser::parse;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub enum SymbolDefinition {
    /// A primitive is a symbol that is implicitly understood by the code generator.
    ///
    /// For example, a string might be defined like:
    /// ```xt
    /// type string;
    /// ```
    Primitive,

    // A message is a symbol whose interface is defined inside an xt file
    Message(Message),
}

#[derive(Debug)]
pub struct Symbol {
    pub namespace: String,
    pub name: String,
    pub symbol_definition: SymbolDefinition,
}

pub struct Namespace {
    pub symbols: HashMap<String, Symbol>,
}

impl Namespace {
    fn new() -> Self {
        Namespace {
            symbols: HashMap::new(),
        }
    }

    fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols.insert(symbol.name.clone(), symbol);
    }

    fn get_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }
}

pub struct SymbolMap {
    pub namespaces: HashMap<String, Namespace>,
}

impl SymbolMap {
    fn new() -> Self {
        SymbolMap {
            namespaces: HashMap::new(),
        }
    }

    fn add_symbol(&mut self, symbol: Symbol) {
        let namespace = self
            .namespaces
            .entry(symbol.namespace.clone())
            .or_insert(Namespace::new());

        namespace.add_symbol(symbol);
    }

    fn get_symbol(&self, namespace: &str, name: &str) -> Option<&Symbol> {
        let namespace = self.namespaces.get(namespace)?;

        namespace.get_symbol(name)
    }
}

#[test]
fn test_symbol_map() {
    use insta::assert_debug_snapshot_matches;

    let symbol = Symbol {
        namespace: String::from("App.Namespace"),
        name: String::from("Message"),
        symbol_definition: SymbolDefinition::Primitive,
    };

    let symbol2 = Symbol {
        namespace: String::from("App.Other"),
        name: String::from("Primitive"),
        symbol_definition: SymbolDefinition::Primitive,
    };

    let mut map = SymbolMap::new();
    map.add_symbol(symbol);
    map.add_symbol(symbol2);
    assert_debug_snapshot_matches!(
        "test_symbol_map finds symbol",
        map.get_symbol("App.Namespace", "Message")
    );
    assert_debug_snapshot_matches!(
        "test_symbol_map finds symbol2",
        map.get_symbol("App.Other", "Primitive")
    );
    assert_debug_snapshot_matches!(
        "test_symbol_map does not find symbol in non existent namespace",
        map.get_symbol("MissingNamespace", "Message")
    );
}

pub struct ModuleResult {
    /// Symbols defined in this module and all imported modules
    pub symbol_lookup: SymbolMap,

    pub entry_file: XtFile,
}

pub fn load_file_and_imports(absolute_filename: PathBuf, files: &mut HashMap<PathBuf, XtFile>) {
    println!("Reading file {:?}", absolute_filename);
    let src = fs::read_to_string(&absolute_filename).unwrap();
    let file = parse(&src);
    for import in &file.imports {
        let import_abs_path = (&absolute_filename)
            .parent() // Get the directory of the current file
            .unwrap()
            .join(&import.path)
            .canonicalize()
            .unwrap();
        load_file_and_imports(import_abs_path, files);
    }
    files.insert(absolute_filename.clone(), file);
}

#[test]
fn test_load_file_and_imports() {
    use insta::assert_debug_snapshot_matches;
    use std::collections::BTreeMap;
    let file_path = PathBuf::from("./src/sample.xt").canonicalize().unwrap();
    let mut files = HashMap::new();
    let _ = load_file_and_imports(file_path, &mut files);

    // Keys need to be sorted so snapshot matches every time
    let ordered: BTreeMap<_, _> = files.iter().collect();
    assert_debug_snapshot_matches!("load_file_and_imports", ordered);
}

/// This is the main function used to load and parse `xt` files and their imports.
pub fn load_module<'a>(filename: String) -> ModuleResult {
    let entry_file_src = fs::read_to_string(&filename).unwrap();
    let entry_file = parse(&entry_file_src);
    // let imports: Vec<XtFile> = vec![];
    ModuleResult {
        symbol_lookup: SymbolMap::new(),
        entry_file,
    }
}
