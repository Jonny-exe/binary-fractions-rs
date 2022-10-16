// Integration tests
extern crate binary_fractions;
use binary_fractions::*;
// use fraction::Fraction;
use crate::binary::_BINARY_VERSION;

#[test]
fn test_versions_const() {
    let sub = &_BINARY_VERSION[..2];
    assert_eq!(sub,"20");
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
