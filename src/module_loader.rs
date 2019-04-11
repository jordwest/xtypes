use crate::ast::{Message, ModuleUse, XtFile};
use std::collections::BTreeMap;
use std::path::PathBuf;

struct ScopeItem<'a> {
    symbol: &'a Message,
    containing_module: &'a XtFile,
    fully_qualified_name: String,
    use_statement: &'a ModuleUse,
}

/// Keeps track of symbols in scope
struct Scope<'a> {
    symbol_map: BTreeMap<String, ScopeItem<'a>>,
}

trait ModuleLoader {
    fn load_module(&mut self, name: AsRef<str>) -> XtFile;
    fn load_imports(&mut self, module: &XtFile) -> BTreeMap<String, XtFile>;
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
// impl ModuleLoader for FileModuleLoader {
//     fn load_module(&self, name: AsRef<str>) -> XtFile {
//         self
//     }
// }
