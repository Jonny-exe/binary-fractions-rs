//! An example program for crate "binary-fractions"

// Can be run via: cargo run --example example

extern crate binary_fractions;
use binary_fractions::*;
use binary_fractions::binary::_BINARY_VERSION;
use binary_fractions::binary::Binary;
use crate::binary::twos_complement::TwosComplement;
//use fraction::Fraction;

fn main() {
    println!("An example.");
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
