extern crate jep106;

#[test]
fn print_unknown_test() {
    let none = jep106::JEP106Code::new(0x08, 0xFF).get();
    assert_eq!(None, none);
}