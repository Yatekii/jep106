extern crate jep106;

#[test]
fn print_unknown_test() {
    let none = jep106::JEP106Code::new(0x08, 0xFF);
    assert_eq!(None, none.get());
    assert_eq!("JEP106Code({ cc: 0x08, id: 0xff } => None)", format!("{:?}", none));
    assert_eq!("Unknown Manufacturer Code", format!("{}", none));
}