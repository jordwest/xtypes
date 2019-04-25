
#[test]
fn test_parse_output_rust_defs() {
    use insta::assert_snapshot_matches;
    use std::path::PathBuf;
    use xtypes_json_rust;
    use xtypes::module_loader::{FileModuleLoader, ModuleScope};

    let mut file_loader = FileModuleLoader::new();
    file_loader.add_path(PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../xtypes/src")));
    file_loader.add_path(PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../bookstore-example")));

    let scope = ModuleScope::load_module_and_imports(&file_loader, "data.xt");
    let defs = xtypes_json_rust::code_gen::write_defs(scope);
    assert_snapshot_matches!("bookstore.data.xt.rs", defs);

    let scope = ModuleScope::load_module_and_imports(&file_loader, "api.xt");
    let defs = xtypes_json_rust::code_gen::write_defs(scope);
    assert_snapshot_matches!("bookstore.api.xt.rs", defs);
}
