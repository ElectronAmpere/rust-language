---
id: lj9ghosrypy2zjr146udv8h
title: Basics
desc: ''
updated: 1703851811657
created: 1703740104473
---
- [Reference links](#reference-links)
- [Keywords](#keywords)
- [Datatypes](#datatypes)
  - [Scalar Types](#scalar-types)
    - [Integers](#integers)
      - [Numerical Literals](#numerical-literals)
    - [Floating-Points](#floating-points)
    - [Booleans](#booleans)
    - [Characters](#characters)

## Reference links

[Getting Started](https://www.rust-lang.org/learn/get-started)

## Keywords

For more Keywords please refer [Appendix A](<../../reference/books/Steve Klabnik_ Carol Nichols - The Rust Programming Language, 2nd Edition-No Starch Press (2022).pdf>)

| Keywords | Description                                           | Usage                                     |
| :------- | :---------------------------------------------------- | :---------------------------------------- |
| let      | Creates a immutable variable                          | let variable = 0;                         |
| mut      | Creates a mutable variable                            | let mut variable = 0;                     |
| fn       | Creates a function                                    | fn main(){ ... }                          |
| loop     | Iterates a section of code indefinitely               | loop { ... }                              |
| break    | Breaks out of the current scope of execution          | loop { break; }                           |
| use      | Imports the invoked crates on to the current program  | use std::io                               |
| continue | Skips current execution after the statement, proceeds | loop { continue; }                        |
| match    | Matches the results to the conditions inside it       | match Result { state1 => statement, ... } |
| const    | Creates a const variable, evaluated at compile time   | const HOURS_IN_SECONDS: u32 = 3600;       |

## Datatypes

### Scalar Types

The scalar types represent a single value. There are four primary types in rust,

1. Integers
2. Floating-Points
3. Booleans
4. Characters

#### Integers

An integer is the number without the fractional component.

| Length  | Signed | Unsigned |
| :------ | :----- | :------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

Signed numbers are stored as 2's complement. Signed variants can store numbers from $-(2^n-1) \space to \space 2^{n-1}-1$.
While the unsigned variants can store numbers from $0 \space to \space 2^n-1$
Say, for i8 the value ranges from $-(2^7) \space to \space 2^7-1$ which euals -128 to 127. And for u8 the value ranges from
$0 \space to \space 2^8-1$ which equals 0 to 255.

Also, the `isize` and `usize` are architechture dependent. Based on the architechture they vary from 32-bit to 64-bit.
By default the integer values in rust are set to `i32`.

##### Numerical Literals

The numerical literals can be multiple numeric types and allow a type suffix for example, `57u8`. They can also have a visual seperator
say `1_000` is the same as `1000`.

| Number Literals  | Example     |
| :--------------- | :---------- |
| Decimal          | 98_222      |
| Hex              | 0xff        |
| Octal            | 0o77        |
| Binary           | 0b1111_0000 |
| Byte (`u8` only) | b'A'        |

#### Floating-Points

#### Booleans

#### Characters
