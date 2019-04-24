use crate::ast::{IdentOrWildcard, ModuleUse, SymbolDefinition, XtFile};
use crate::parser;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct ScopeItem {
    pub symbol: SymbolDefinition,
    // containing_module: &'a XtFile,
    pub fully_qualified_name: String,
    pub use_statement: Option<ModuleUse>,
}

/// Keeps track of symbols in scope
#[derive(Debug)]
pub struct ModuleScope {
    pub symbol_map: BTreeMap<String, ScopeItem>,
    pub module: XtFile,
    pub modules: Vec<XtFile>,
}

impl ModuleScope {
    fn add_symbols_from_module(&mut self, module: &XtFile, use_statement: Option<&ModuleUse>) {
        // Prefix the symbol with the import name. For example `Recipe.` in the following import:
        // ```xt
        // use "recipe.xt" as Recipe
        // ```
        let prefix = match use_statement {
            Some(ModuleUse {
                ident: IdentOrWildcard::Ident(v),
                ..
            }) => format!("{}.", v),
            _ => format!(""),
        };

        for symbol in &module.symbols {
            let fully_qualified_name = format!("{}{}", prefix, symbol.name.identifier());

            self.symbol_map.insert(
                fully_qualified_name.clone(),
                ScopeItem {
                    symbol: symbol.clone(),
                    use_statement: use_statement.map(|v| v.clone()),
                    fully_qualified_name,
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

        instance.add_symbols_from_module(&module, None);

        for use_statement in &module.use_imports {
            let module = loader.load_module(&use_statement.filename);
            instance.add_symbols_from_module(&module, Some(use_statement));
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

pub trait ModuleLoader {
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

    pub fn add_path(&mut self, path: PathBuf) {
        self.search_paths.push(path);
    }
}

impl ModuleLoader for FileModuleLoader {
    fn load_module<T: AsRef<str> + Sized>(&self, name: T) -> XtFile {
        let module_path = Path::new(name.as_ref());

        if self.search_paths.len() == 0 {
            panic!("FileModuleLoader needs at least one path to search for modules, none were provided. Check the add_path function.");
        }

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
    use insta::assert_debug_snapshot_matches;

    let src_dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/src"));
    let mut file_loader = FileModuleLoader::new();
    file_loader.search_paths.push(src_dir);
    let module = file_loader.load_module("sample.xt");

    assert_debug_snapshot_matches!("FileModuleLoader::load_module", module);
}
