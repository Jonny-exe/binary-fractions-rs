[![Documentation](https://docs.rs/binary-fractions/badge.svg)](https://docs.rs/binary-fractions/) 
[![crates.io - Version](https://img.shields.io/crates/v/binary-fractions)](https://crates.io/crates/binary-fractions)
[![crates.io - Downloads](https://img.shields.io/crates/d/binary-fractions)](https://crates.io/crates/binary-fractions)

# Floating-point Binary Fractions: Do math in base 2!

![logo](https://raw.githubusercontent.com/Jonny-exe/binary-fractions/master/binary-fractions.svg)

```
 ████       ███
░░███      ░░░
 ░███████  ████  ████████    ██████   ████████  █████ ████
 ░███░░███░░███ ░░███░░███  ░░░░░███ ░░███░░███░░███ ░███
 ░███ ░███ ░███  ░███ ░███   ███████  ░███ ░░░  ░███ ░███
 ░███ ░███ ░███  ░███ ░███  ███░░███  ░███      ░███ ░███
 ████████  █████ ████ █████░░████████ █████     ░░███████
░░░░░░░░  ░░░░░ ░░░░ ░░░░░  ░░░░░░░░ ░░░░░       ░░░░░███
                                                 ███ ░███
                                                ░░██████
                                                 ░░░░░░

    ██████                                 ███      ███
   ███░░███                               ░███     ░░░
  ░███ ░░░  ████████   ██████    ██████  ███████   ████   ██████  ████████    █████
 ███████   ░░███░░███ ░░░░░███  ███░░███░░░███░   ░░███  ███░░███░░███░░███  ███░░
░░░███░     ░███ ░░░   ███████ ░███ ░░░   ░███     ░███ ░███ ░███ ░███ ░███ ░░█████
  ░███      ░███      ███░░███ ░███  ███  ░███ ███ ░███ ░███ ░███ ░███ ░███  ░░░░███
  █████     █████    ░░████████░░██████   ░░█████  █████░░██████  ████ █████ ██████
 ░░░░░     ░░░░░      ░░░░░░░░  ░░░░░░     ░░░░░  ░░░░░  ░░░░░░  ░░░░ ░░░░░ ░░░░░░
```

This is an implementation of a floating-point binary fractions crate
in Rust. This enables you to work with binary fractions and binary floats
with ease! This is a port from the corresponding Python verion of
[binary-franctions](https://github.com/Jonny-exe/binary-fractions).

This module allows one to represent integers, floats and fractions as
binary strings.
- e.g. the integer 3 will be represented as string '0b11'.
- e.g. the float -3.75 will be represented as string '-0b11.11'.
- e.g. the fraction 1/2 will be represented as string '0b0.1'
- Exponential representation is also possible:
'-0b0.01111e3', '-0b11.1e1' or '-0b1110e-2' all represent float -3.75.
- two's complement representation is possible too:
'11.11' for -1.25 in decimal, or '-0b1.01' in binary fraction.

Many operations and transformations are offered.
You can sum, subtract, multiply, and divide long floating-point binary
fractions. You can compute power of them, shift them left, shift them right,
etc.

Basic representation of binary fractions and binary floats:
A binary fraction is a subset of binary floats. Basically, a binary fraction
is a binary float without an exponent (e.g. '-0b101.0101').
Let's have a look at an example binary float value to see how it is represented.

```
     prefix '0b' to indicate "binary" or "base 2"
     ||
     ||   decimal point
     ||   |
     ||   |    exponent separator
     ||   |    |
     ||   |    | exponent in base 10 (not in base 2!)
     ||   |    | ||
    -0b101.0101e-34  <-- example floating-point binary fraction
    |  ||| |||| |
 sign  ||| |||| exponent sign
       ||| ||||
       ||| fraction bits in base 2
       |||
       integer bits in base 2
```

If you are curious about floating point binary fractions, have a look at:
- https://en.wikipedia.org/wiki/Computer_number_format#Representing_fractions_in_binary
- https://www.electronics-tutorials.ws/binary/binary-fractions.html
- https://ryanstutorials.net/binary-tutorial/binary-floating-point.php
- https://planetcalc.com/862/

If you are curious about Two's complement:
- https://janmr.com/blog/2010/07/bitwise-operators-and-negative-numbers/
- https://en.wikipedia.org/wiki/Two%27s_complement

## License:
- GPL v3 or later

## Features:
- Rust
- constructors for various types: int, float, Fraction, str, TwosComplement, Binary
- supports many operators: +, -, *, /, //, %, **, <<, >>, ~, &, ...
- supports many methods: not, abs, round, floor, ceil, ...
- internally the value is kept as a Fraction and most operations are done
	in Fractions. This results in better performance and infinite precision.
	Only a few limited operations such as 'and', 'or', 'xor', and 'invert'
	are done on strings.
- very high precision
- many operations are lossless, i.e. with no rounding errors or loss of precision
- supports very long binary fractions
- supports exponential representations
- well documented
    - Please read the documentation at
        ([binary-fractions](https://docs.rs/fraction/latest/binary-fractions/)).
- well tested
    - over 1600 test cases


## Sample usage, Example calls:

Please have a look at the short example program that uses the
`Binary` crate. See `examples` directory in the repo.

## Requirements:
- Rust
- Fraction crate

## Installation:
- see [https://crates.io/crates/binary-fractions/](https://crates.io/crates/binary-fractions/)

## Testing, Maturity
- run example(s) in `examples` directory to execute simple sample program(s)
- run test cases in `tests` directory to execute all unit tests
- `Binary` is relatively mature, more than 1600 test cases have been written and all
    passed.

## Contributions:
- PRs are welcome and very much appreciated! :+1:
- Please run and pass all existing 1600+ test cases before issuing a PR.
- File Format: linted/beautified with `rustfmt`

Enjoy :heart: !
