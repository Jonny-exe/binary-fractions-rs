// Selected function signatures with this regex: fn <A-Za-z\-\_>*\(<\s\S\n\t\rz>*?\)\s?(->\s)?<A-Za-z\_\<\>\, >*? {}
#![allow(dead_code,unused_imports)]

use regex::Regex;
use fraction::{Fraction};

const _BINARY_WARNED_ABOUT_FLOAT: bool = false;  // type: bool
const _BINARY_RELATIVE_TOLERANCE: f64 = -7.281718172; // 1e-10 type: float  number of binary digits to the right of decimal point
const _BINARY_PRECISION: i32 = 128;  // type: int
const _PREFIX: &str = "0b";  // type: &str
const _EXP: &str = "e";  // type: &str
const _NAN: &str = "NaN";  // type: &str
const _INF: &str = "Inf";  // type: &str
const _NINF: &str = "-Inf"; // type: &str
const _BINARY_VERSION: &str = "20210721-160328";  // type: &str // format: date +%Y%m%d-%H%M%S
const _BINARY_TOTAL_TESTS: i32 = 1646;  // type: int // number of asserts in .py file

struct TwosComplement {
    value: i32
}

impl TwosComplement {
    fn from (value: i32) -> TwosComplement {
        TwosComplement{
            value: value
        } 
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
struct Binary<'a> {
    fraction: Fraction,
    string: &'a str,
    sign: i32,
    is_special: bool,
    warn_on_float: bool,
    is_lossless: bool,

}

impl<'a> Binary<'a> {
    pub fn from<T>(
            // cls: Binary,
            value: T, // int, float, string, fraction, twoscomplement, binary   default = 0
            simplify: Option<bool>, //default true
            warn_on_float: Option<bool>, //default false
        ) -> Binary<'a> {
            // TODO: global _BINARY_WARNED_ABOUT_FLOAT 
        // self = super(Binary, cls).__new__(cls);
        let simplify = simplify.unwrap_or(true);
        let warn_on_float = warn_on_float.unwrap_or(false);

        let parser = Regex::new(r" 
            \s*
            (?P<sign>[-+])?              // an optional sign, followed by either...
            (
                (?=\d|\.[01])              // ...a number (with at least one digit)
                (?P<int>[01]*)             // having a (possibly empty) integer part
                (\.(?P<frac>[01]*))?       // followed by an optional fractional part
                (E(?P<exp>[-+]?\d+))?    // followed by an optional exponent, or...
            |
                Inf(inity)?              // ...an infinity, or...
            |
                (?P<signal>s)?           // ...an (optionally signaling)
                NaN                      // NaN
                (?P<diag>\d*)            // with (possibly empty) diagnostic info.
            )
            \s*
            \Z"
        ).unwrap();

        let self_ = Binary {
            fraction: Fraction::from(0),
            string: "",
            sign: 0,
            is_special: false,
            warn_on_float: warn_on_float,
            is_lossless: false,
        };
        // self._fraction = Fraction()
        // self._string = ""
        // self._sign = 0  // 0 indicates positive, 1 indicates negative sign
        // self._is_special = False
        // self._warn_on_float = warn_on_float
        return self_
    }
    
    fn to_f64(value: &str) -> f64 {
        return 0.0;
    }
    fn from_f64(value: f64, rel_tol: Option<f64>) -> &'a str {
        rel_tol.unwrap_or(_BINARY_RELATIVE_TOLERANCE);
        // TODO
        return "";
    }
    fn to_no_exponent<T>(
            self_value: T, // Binary or str
            length: Option<i32>,
            simplify: Option<bool>,
            add_prefix: Option<bool>,
        ) -> T { // tuple
        length.unwrap_or(-1);
        simplify.unwrap_or(true);
        add_prefix.unwrap_or(false);

        //TODO
        return self_value;
    }
    fn to_no_mantissa(self: Binary<'a>) -> Binary<'a> {
        // TODO
        return self;
    }
    fn to_exponent(self: Binary<'a>, exp: Option<i32>) -> Binary<'a> {
        // TODO
        exp.unwrap_or(0);
        return self;
    }
    fn to_sci_exponent(self: Binary<'a>) -> Binary<'a> {
        // TODO
        return self;
    }
    fn to_eng_exponent(self: Binary<'a>) -> Binary<'a> {
        //TODO
        return self;
    }
    fn to_fraction<T>(self_value: T) -> T { //str or binary
        //TODO
        return self_value;
    }
    fn to_fraction_alternative_implementation(value: &'a str) -> Fraction {
        //TODO
        return Fraction::from(0);
    }
    fn to_twoscomplement(self: Binary<'a>, length: Option<i32>) -> TwosComplement {
        // TODO
        length.unwrap_or(-1);
        return TwosComplement::from(0);
    }

    fn from_twoscomplement(value: TwosComplement, simplify: Option<bool>) -> Binary<'a> {
        // ERROR in oriignal, return type 
        // TODO
        simplify.unwrap_or(true);
        return Binary::from(0, None, None);
    }
    fn __f64__(self: Binary<'a>) -> f64 { // originally it was UNION[f64, i32]
        // TODO
        return 0.0;
    }
    fn __i32__(self: Binary<'a>) -> i32 {
        // TODO
        return 0;
    }
    fn __str__(self: Binary<'a>) -> &'a str {
        // TODO
        return "";
    }
    fn compare_representation<T>(self: Binary<'a>, other: T) -> bool { // str or biary
        // TODO
        return true;
    }
    fn __repr__(self: Binary<'a>) -> &'a str {
        // TODO
        return "";
    }
    fn no_prefix<T>(self_value: T) -> &'a str { // str or binary
        // TODO 
        return "";
    }
    fn np<T>(self_value: T) -> &'a str { // str or binary
        // TODO
        return "";
    }
    fn version() -> &'a str {
        // TODO
        return "";
    }
    fn simplify(value: &'a str, add_prefix: Option<bool>) -> &'a str {
        add_prefix.unwrap_or(false);
        // TODO
        return "";
    }
    fn __round__(self: Binary<'a>, ndigits: Option<i32>) -> Binary<'a> {
        ndigits.unwrap_or(0);
        // TODO
        return Binary::from(0, None, None)
    }
    fn round(self: Binary<'a>, ndigits: Option<i32>, simplify: Option<bool>) -> Binary<'a> {
        ndigits.unwrap_or(0);
        simplify.unwrap_or(true);
        // TODO
        return Binary::from(0, None, None);
    }
    fn round_to(value: &'a str, ndigits: Option<i32>, simplify: Option<bool>) -> &'a str {
        ndigits.unwrap_or(0);
        simplify.unwrap_or(true);
        // TODO
        return "";
    }
    fn lfill(self: Binary<'a>, ndigits: Option<i32>, strict: Option<bool>) {
        ndigits.unwrap_or(0);
        strict.unwrap_or(false);
        // TODO
    }
    fn lfill_to(value: &'a str, ndigits: Option<i32>, strict: Option<bool>) -> &'a str {
        ndigits.unwrap_or(0);
        strict.unwrap_or(false);
        // TODO
        return "";
    }
    fn rfill(self: Binary<'a>, ndigits: Option<i32>, strict: Option<bool>) {
        ndigits.unwrap_or(0);
        strict.unwrap_or(false);
        // TODO
    }
    fn rfill_to(value: &'a str, ndigits: Option<i32>, strict: Option<bool>) -> &'a str {
        ndigits.unwrap_or(0);
        strict.unwrap_or(false);
        // TODO
        return "";
    }
    fn get_components(value: &'a str, simplify: Option<bool>) -> (i32, &'a str, &'a str, i32) {
        simplify.unwrap_or(true);
        // TODO
        return (0, "", "", 0);
    }
    fn components(self: Binary<'a>, simplify: Option<bool>) -> (i32, &'a str, &'a str, i32) {
        simplify.unwrap_or(true);
        // TODO
        return (0, "", "", 0)
    }
    fn isinfinity(self: Binary<'a>) -> bool {
        // TODO
        return true;
    }
    fn isnegativeinfinity(self: Binary<'a>) -> bool {
        // TODO
        return true;
    }
    fn ispositiveinfinity(self: Binary<'a>) -> bool {
        // TODO
        return true;
    }
    fn isnan(self: Binary<'a>) -> bool {
        // TODO
        return true;
    }
    fn isi32(self: Binary<'a>) -> bool {
        // TODO
        return true;
    }
    fn _adjusted(self: Binary<'a>) -> i32 {
        return 0;
        // TODO
    }
    fn fraction(self: Binary<'a>) -> Fraction {
        // TODO
        return Fraction::from(0);
    }
    fn string(self: Binary<'a>) -> &'a str {
        // TODO
        return "";
    }
    fn sign(self: Binary<'a>) -> i32 {
        // TODO
        return 0;
    }
    fn isspecial(self: Binary<'a>) -> bool {
        // TODO
        return true;
    }
    fn warnonf64(self: Binary<'a>) -> bool {
        // TODO
        return true;
    }
    fn islossless(self: Binary<'a>) -> bool {
        // TODO
        return true;
    }
    fn fraction_to_string<T>(
            number: T,  // i32, f64, Fraction
            ndigits: Option<i32>,
            simplify: Option<bool>
        ) -> &'a str {
        ndigits.unwrap_or(_BINARY_PRECISION);
        simplify.unwrap_or(true);
        return "";
    }
    fn isclose<T>(
            self: Binary<'a>, other: T, rel_tol: Option<f64>
        ) -> bool {
        // TODO
        rel_tol.unwrap_or(_BINARY_RELATIVE_TOLERANCE);
        return true;

    }
    fn _cmp<T>(self: Binary<'a>, other: T) -> i32 {
        return 0;
    }
    fn compare<T>(self: Binary<'a>, other: T) -> Binary<'a> {
        return Binary::from(0, None, None);
    }
    fn __lt__<T>(self: Binary<'a>, other: T) -> bool {
        // TODO
        return true;
    }
    fn __gt__<T>(self: Binary<'a>, other: T) -> bool {
        // TODO
        return true;
    }
    fn __le__<T>(self: Binary<'a>, other: T) -> bool {
        // TODO
        return true;
    }
    fn __eq__<T>(self: Binary<'a>, other: T) -> bool {
        // TODO
        return true;
    }
    fn __ge__<T>(self: Binary<'a>, other: T) -> bool {
        // TODO
        return true;
    }
    fn __add__<T>(self: Binary<'a>, other: T) -> Binary<'a> {
        // TODO
        return Binary::from(0, None, None);
    }
    fn __sub__<T>(self: Binary<'a>, other: T) -> Binary<'a> {
        // TODO
        return Binary::from(0, None, None);
    }
    fn __mul__<T>(self: Binary<'a>, other: T) -> Binary<'a> {
        // TODO
        return Binary::from(0, None, None);
    }
    fn __truediv__<T>(self: Binary<'a>, other: T) -> Binary<'a> {
        // TODO
        return Binary::from(0, None, None);
    }
    fn __floordiv__<T>(self: Binary<'a>, other: T) -> Binary<'a> {
        // TODO
        return Binary::from(0, None, None);
    }
    fn __mod__<T>(self: Binary<'a>, other: T) -> Binary<'a> {
        // TODO
        return Binary::from(0, None, None);
    }
    fn __pow__<T>(self: Binary<'a>, other: T) -> Binary<'a> {
        // TODO
        return Binary::from(0, None, None);
    }
    fn __abs__(self: Binary<'a>) -> Binary<'a> {
        // TODO
        return Binary::from(0, None, None);
    }
    fn __ceil__(self: Binary<'a>) -> i32 {
        // TODO
        return 0;
    }
    fn ceil(self: Binary<'a>) -> Binary<'a> {
        return Binary::from(0, None, None);
    }
    fn __floor__(self: Binary<'a>) -> i32 {
        return 0;
    }
    fn floor(self: Binary<'a>) -> Binary<'a> {
        // TODO
        return Binary::from(0, None, None);
    }
    fn __rshift__(self: Binary<'a>, ndigits: i32) -> Binary<'a> {
        // TODO
        return Binary::from(0, None, None);
    }
    fn __lshift__(self: Binary<'a>, ndigits: i32) -> Binary<'a> {
        return Binary::from(0, None, None);
    }
    fn __bool__(self: Binary<'a>) -> bool {
        return false;
    }
    fn __not__(self: Binary<'a>) -> bool {
        return false;
    }
    fn __and__<T>(self: Binary<'a>, other: T) -> Binary<'a> {
        return Binary::from(0, None, None);
    }
    fn __or__<T>(self: Binary<'a>, other: T) -> Binary<'a> {
        return Binary::from(0, None, None);
    }
    fn __xor__<T>(self: Binary<'a>, other: T) -> Binary<'a> {
        return Binary::from(0, None, None);
    }
    fn _and_or_xor(this: Binary, other: Binary, which: &'a str) -> Binary<'a> {
        // TODO
        return Binary::from(0, None, None);
    }
    // fn __and(ab) {

    // }
    // fn __or(ab) {}
    // fn __xor(ab) {}
    // fn negative(number) {}
    fn __invert__(self: Binary<'a>) -> Binary<'a> {
        return Binary::from(0, None, None);
    }


    // fn selftest(self) -> bool {}
    // fn test___new__(self) {}
    // fn test_istwoscomplement(self) {}
    // fn test_components(self) {}
    // fn test_simplify(self) {}
    // fn test_to_fraction(self) {}
    // fn test_to_f64(self) {}
    // fn test_to_no_mantissa(self) {}
    // fn test_to_no_exponent(self) {}
    // fn test_invert(self) {}
    // fn selftest(self) -> bool {}
    // fn test___new__(self) {}
    // fn test_version(self) {}
    // fn test_to_f64(self) {}
    // fn test_from_f64(self) {}
    // fn test_to_no_exponent(self) {}
    // fn test___f64__(self) {}
    // fn test___i32__(self) {}
    // fn test___str__(self) {}
    // fn test_compare_representation(self) {}
    // fn test_no_prefix(self) {}
    // fn test_np(self) {}
    // fn test_simplify(self) {}
    // fn test_to_fraction(self) {}
    // fn test___round__(self) {}
    // fn test_round(self) {}
    // fn test_round_to(self) {}
    // fn test_lfill(self) {}
    // fn test_lfill_to(self) {}
    // fn test_rfill(self) {}
    // fn test_rfill_to(self) {}
    // fn test_to_no_mantissa(self) {}
    // fn test_to_exponent(self) {}
    // fn test_to_sci_exponent(self) {}
    // fn test_to_eng_exponent(self) {}
    // fn test_get_components(self) {}
    // fn test_components(self) {}
    // fn test_isinfinity(self) {}
    // fn test_isnegativeinfinity(self) {}
    // fn test_ispositiveinfinity(self) {}
    // fn test_isnan(self) {}
    // fn test_isi32(self) {}
    // fn test_fraction(self) {}
    // fn test_string(self) {}
    // fn test_fraction_to_string(self) {}
    // fn test_isclose(self) {}
    // fn test___eq__(self) {}
    // fn test___lt__(self) {}
    // fn test___gt__(self) {}
    // fn test___le__(self) {}
    // fn test___ge__(self) {}
    // fn test___add__(self) {}
    // fn test___sub__(self) {}
    // fn test___mul__(self) {}
    // fn test___truediv__(self) {}
    // fn test___floordiv__(self) {}
    // fn test___mod__(self) {}
    // fn test___pow__(self) {}
    // fn test___abs__(self) {}
    // fn test___ceil__(self) {}
    // fn test_ceil(self) {}
    // fn test___floor__(self) {}
    // fn test_floor(self) {}
    // fn test___rshift__(self) {}
    // fn test___lshift__(self) {}
    // fn test___bool__(self) {}
    // fn test___not__(self) {}
    // fn test___and__(self) {}
    // fn test___or__(self) {}
    // fn test___xor__(self) {}
    // fn test___invert__(self) {}
    // fn test_to_twoscomplement(self) {}
    // fn test_from_twoscomplement(self) {}
}

