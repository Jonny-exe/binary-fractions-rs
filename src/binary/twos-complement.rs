//! Class TwosComplement description

/// Structure for TwosComplement
// The struct is public so that BinaryFractions can use it as a type.
// The fields are private to force user to use a contructor (associated fn).
pub struct TwosComplement {
    value: i32
}

impl TwosComplement {
    pub fn new () -> TwosComplement {
        TwosComplement{
            value: 0
        }
    }
    pub fn from (value: i32) -> TwosComplement {
        TwosComplement{
            value: value
        }
    }
    pub fn get (&self) -> i32 {
        self.value
    }
}
// impl TwosComplement {
//     fn new(
//         cls: TwosComplement,
//         value: tuple<i32, f64, Fraction, str> = 0,
//         length: i32 = -1,
//         rel_tol: f64 = _BINARY_RELATIVE_TOLERANCE,
//         ndigits: i32 = _BINARY_PRECISION,
//         simplify: bool = true,
//         warn_on_f64: bool = false,
//     ) -> TwosComplement {}

//     fn istwoscomplement(value: &'a str) -> bool {}
//     fn components(
//             self_value: tuple<str, TwosComplement>, simplify: bool = true
//         ) -> tuple<i32, str, str, i32> {}
//     fn simplify(self_value: tuple<str, TwosComplement>) -> tuple<str, TwosComplement> {}
//     fn to_fraction(self_value: tuple<str, TwosComplement>) -> Fraction {}
//     fn to_f64(self_value: tuple<str, TwosComplement>) -> f64 {}
//     fn to_no_mantissa(
//             self_value: tuple<str, TwosComplement>, length: i32 = -1
//         ) -> tuple<str, TwosComplement> {}
//     fn to_no_exponent(
//             self_value: tuple<str, TwosComplement>, length: i32 = -1, simplify: bool = true
//         ) -> tuple<str, TwosComplement> {}
//     fn invert(
//             self_value: tuple<str, TwosComplement>, simplify: bool = true
//         ) -> tuple<str, TwosComplement> {}
// }


// Unit tests
// Tests that cover exclusively this module

#[cfg(test)]
mod tests {
    use crate::binary::TwosComplement;

    #[test]
    fn test_contructor() {
        // This not working yet, requires == to be implemented via `PartialEq<_>`
        // assert_eq!(TwosComplement::new(),TwosComplement{ value: 0 })
        let n = TwosComplement::new();
        assert_eq!(n.value,0);
        let m = TwosComplement::from(1);
        assert_eq!(m.get(),1);
        let o = TwosComplement{ value: 2 };
        assert_eq!(o.get(),2);
        let p = TwosComplement{ value: 3 };
        assert_eq!(p.get(),3);
    }
}
