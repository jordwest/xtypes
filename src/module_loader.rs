use crate::ast::{SymbolDefinition, XtFile};
use crate::parser;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct ScopeItem {
    symbol: SymbolDefinition,
    // containing_module: &'a XtFile,
    fully_qualified_name: String,
    // use_statement: &'a ModuleUse,
}

/// Keeps track of symbols in scope
#[derive(Debug)]
struct ModuleScope {
    symbol_map: BTreeMap<String, ScopeItem>,
    module: XtFile,
    modules: Vec<XtFile>,
}

impl ModuleScope {
    fn add_symbols_from_import(&mut self, module: &XtFile) {
        for symbol in &module.symbols {
            self.symbol_map.insert(
                symbol.name.identifier(),
                ScopeItem {
                    symbol: symbol.clone(),
                    fully_qualified_name: symbol.name.identifier(),
                },
            );
        }
    }

    pub fn load_module_and_imports<T: AsRef<str> + Sized>(
        loader: impl ModuleLoader,
        module_location: T,
    ) -> ModuleScope {
        let module = loader.load_module(module_location);

        let mut instance = ModuleScope {
            symbol_map: BTreeMap::new(),
            module: module.clone(),
            modules: Vec::with_capacity(module.use_imports.len()),
        };

        instance.add_symbols_from_import(&module);

        for use_statement in &module.use_imports {
            let module = loader.load_module(&use_statement.filename);
            instance.add_symbols_from_import(&module);
            instance.modules.push(module);
        }

        instance
    }
}

#[test]
fn test_load_module_and_imports() {
    use insta::assert_debug_snapshot_matches;
    let src_dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/src"));
    let mut file_loader = FileModuleLoader::new();
    file_loader.search_paths.push(src_dir);

    let scope = ModuleScope::load_module_and_imports(file_loader, "sample.xt");
    assert_debug_snapshot_matches!("ModuleScope::load_module_and_imports", scope);
}

trait ModuleLoader {
    fn load_module<T: AsRef<str> + Sized>(&self, name: T) -> XtFile;
}

pub struct FileModuleLoader {
    search_paths: Vec<PathBuf>,
}

impl FileModuleLoader {
    pub fn new() -> Self {
        FileModuleLoader {
            search_paths: Vec::new(),
        }
    }
}

impl ModuleLoader for FileModuleLoader {
    fn load_module<T: AsRef<str> + Sized>(&self, name: T) -> XtFile {
        let module_path = Path::new(name.as_ref());
        for search_path in &self.search_paths {
            let mut filename = search_path.to_owned();
            println!("searching {:?} for {:?}", search_path, module_path);
            filename.push(module_path);
            if filename.is_file() {
                let src = fs::read_to_string(filename).unwrap();
                return parser::parse(&src);
            }
        }

        panic!("Module {} not found", name.as_ref());
    }
}

#[test]
fn test_load_module() {
    let src_dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/src"));
    let mut file_loader = FileModuleLoader::new();
    file_loader.search_paths.push(src_dir);
    let module = file_loader.load_module("sample.xt");
    assert_eq!(module.module_info.name, "Sample.Test");

    assert_eq!(module.use_imports.len(), 1);
    for import in module.use_imports {
        let imported_module = file_loader.load_module(import.filename);
        assert_eq!(imported_module.module_info.name, "XTypes.Prelude");
    }
}
