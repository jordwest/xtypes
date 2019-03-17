#[test]
fn test_parse_output_typescript_defs() {
    use insta::assert_snapshot_matches;
    use xtypes::parser::parse;

    let file = parse(include_str!("./bookstore/data.xt"));

    let defs = xtypes::writers::typescript::write_defs(file);
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
