use crate::ast::*;
use crate::parser::parse;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

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

pub struct Symbol {
    pub namespace: String,
    pub name: String,
    pub symbol_definition: SymbolDefinition,
}

pub struct SymbolStore {}

pub struct ModuleResult {
    /// Symbols defined in this module and all imported modules
    pub symbol_lookup: SymbolStore,

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
    let file_path = PathBuf::from("./src/sample.xt").canonicalize().unwrap();
    let mut files = HashMap::new();
    let modules = load_file_and_imports(file_path, &mut files);
    assert_debug_snapshot_matches!("load_file_and_imports", files);
}

/// This is the main function used to load and parse `xt` files and their imports.
pub fn load_module(filename: String) -> ModuleResult {
    let entry_file_src = fs::read_to_string(&filename).unwrap();
    let entry_file = parse(&entry_file_src);
    // let imports: Vec<XtFile> = vec![];
    ModuleResult {
        symbol_lookup: SymbolStore {},
        entry_file,
    }
}
