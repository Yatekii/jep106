extern crate jep106;

#[test]
fn print_version_test() {
    let version = jep106::version();
    assert_eq!("BI", version);
}
