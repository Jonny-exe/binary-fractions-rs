// To build and run cd to the subdirectory and run std cargo commands
// E.g. pushd examples/sample && cargo build && cargo run && popd

extern crate binary_fractions;
use binary_fractions::binary::*;
use binary_fractions::binary::twos_complement::TwosComplement;
//use fraction::Fraction;

fn main() {
  // something
  println!("A sample app in a crate using binary-fractions.");
  println!("Version number is {}.", _BINARY_VERSION);
  let s = TwosComplement::new();
  assert_eq!(s.get(), 0);
  let u = TwosComplement::from(1);
  assert_eq!(u.get(), 1);
  let _v = TwosComplement::from(1);
  // the == and != are not yet implemented for TwosComplement
  // assert_ne!(u, _v);
  let w = Binary::from(1, None, None);
  assert!(w.isi32());
  println!("Bye.");
}
