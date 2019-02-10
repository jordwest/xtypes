#[test]
fn test_parse_output_typescript_defs() {
    use insta::assert_snapshot_matches;
    use xtypes::parser::parse;

    let file = parse(include_str!("../src/sample.xt"));

    let defs = xtypes::writers::typescript::write_defs(file);
    assert_snapshot_matches!("sample.xt.d.ts", defs);
}