extern crate jep106;

#[test]
fn print_nordic_test() {
    let nordic = jep106::JEP106Code::new(0x02, 0x44);
    assert_eq!(Some("Nordic VLSI ASA"), nordic.get());
    assert_eq!("JEP106Code({ cc: 0x02, id: 0x44 } => Some(\"Nordic VLSI ASA\"))", format!("{:?}", nordic));
    assert_eq!("Nordic VLSI ASA", format!("{}", nordic));
}