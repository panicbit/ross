extern crate ross;

#[test]
fn simple_tri() {
    let code = include_str!("simple_tri.ross");
    ross::parse_module(code).unwrap_or_else(|e| panic!("{}", e));
}
