---
id: lj9ghosrypy2zjr146udv8h
title: Basics
desc: ''
updated: 1703938681342
created: 1703740104473
---
- [Reference links](#reference-links)
- [Keywords](#keywords)
- [Datatypes](#datatypes)
  - [Scalar Types](#scalar-types)
    - [Integers](#integers)
      - [Numerical Literals](#numerical-literals)
      - [Integer Overflow](#integer-overflow)
      - [Floating-Points](#floating-points)
      - [Booleans](#booleans)
      - [Characters](#characters)
    - [Numeric Operations](#numeric-operations)
  - [Compound Types](#compound-types)
    - [Tuples](#tuples)
    - [Array](#array)

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
Say, for `i8` the value ranges from $-(2^7) \space to \space 2^7-1$ which euals -128 to 127. And for `u8` the value ranges from
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

##### Integer Overflow

To explicitly handle the possibility of overflow, you can use these families of methods provided by the standard library for primitive
numeric types:

- Wrap in all modes with the `wrapping_*` methods, such as `wrapping_add`.
- Return the `None` value if there is overflow with the `checked_*` methods.
- Return the value and a boolean indicating whether there was overflow with the `overflowing_*` methods.
- Saturate at the value's minimum or maximum values with the `saturating_*` methods.

##### Floating-Points

There are only two primitive types they are

- `f32` - 32-bit size
- `f64` - 64-bit size

by default on modern CPUs is `f64`, has roughly the same speed as `f32` but with more precision.
Floating-point numbers are represented according to [IEEE-754](../../reference/standards/ieee/IEEE754.PDF) standard.

##### Booleans

For booleans there are only two type of values,

- `true`
- `false`

Most often they are used in `if` statements.

##### Characters

The character type `char` represents the alphabetic type values `a-z, A-Z` and more.
A `char` type is ''four bytes'' in length. Unicode scalar values range from
`U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF` inclusive.

#### Numeric Operations

Rust supports basic mathematical operations for the number types,

- addition
- subtraction
- multiplication
- division
- remainder

> Integer division truncates towards zero to the nearest integer.
> For more information refer [Appendix B](<../../reference/books/Steve Klabnik_ Carol Nichols - The Rust Programming Language, 2nd Edition-No Starch Press (2022).pdf>)

### Compound Types

Compound types can group multiple values into one type. There are two compound types,

- tuples
- arrays

#### Tuples

A tuple is a general way of grouping together a number of values
with a variety of types into one compound type. Tuples have a fixed
length: once declared, they cannot grow or shrink in size.

#### Array

Another way to have a collection of multiple values is with an array.
Unlike a tuple, every element of an array must have the same type.
Unlike arrays in some other languages, arrays in Rust have a fixed
length.

> The data is allocated on the stack rather than the heap.

