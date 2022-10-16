// Integration tests
extern crate binary_fractions;
use binary_fractions::*;
use crate::binary::*;
use crate::twos_complement::*;
// use fraction::Fraction;

#[test]
fn test_versions() {
    let sub = &_BINARY_VERSION[..2];
    assert_eq!(sub,"20");
}

#[test]
fn test_binary_constructor() {
    let b = Binary::from(5, None, None);
    assert_eq!(b.to_string(),"");
}

#[test]
fn test_twoscomplement_constructor() {
    let b = TwosComplement::from(5);
    assert_eq!(b.get(),5);
}

// // This works but is not ideal.
// #[path = "../src/binary.rs"]
// mod binary;
//
// // All the *integration* tests go here.
// // The *unit* tests are inside the corresponding files.
// mod tests {
//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     #[test]
//     fn test_Binary() {
//         assert!(true);
//     }
// }
