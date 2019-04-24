#[test]
fn test_parse_output_typescript_defs() {
    use insta::assert_snapshot_matches;
    use std::path::PathBuf;
    use xtypes::module_loader::{FileModuleLoader, ModuleScope};

    let mut file_loader = FileModuleLoader::new();
    file_loader.add_path(PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/src")));
    file_loader.add_path(PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests")));

    let scope = ModuleScope::load_module_and_imports(file_loader, "bookstore/data.xt");

    let defs = xtypes::writers::typescript::write_defs(scope);
    assert_snapshot_matches!("bookstore.data.xt.d.ts", defs);
}

#[test]
fn test_parse_output_rust_defs() {
    use insta::assert_snapshot_matches;
    use xtypes::parser::parse;

    let file = parse(include_str!("./bookstore/data.xt"));

    let defs = xtypes::writers::rust::write_defs(file);
    assert_snapshot_matches!("bookstore.data.xt.rs", defs);
}
