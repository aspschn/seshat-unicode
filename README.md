# Seshat 𓋇𓏏𓁐

[![crates.io](https://img.shields.io/crates/v/seshat-unicode.svg)](https://crates.io/crates/seshat-unicode)

![logo](https://raw.githubusercontent.com/hardboiled65/Seshat/master/docs/seshat-logo.png)

A Unicode Library for Rust.

<!--
Demo
-----
[Seshat Web Demo](https://seshat-demo.herokuapp.com)
-->

## Introduction

Seshat (pronounce as Sehs-hat) is a Unicode library that written in Rust.
It provides many of Unicode character data and standard algorithms.
The goal of this project is to provide a ICU-like library in Rust.

## Version

Seshat follows the latest version of Unicode. Currently using version 16.0.0.

## Usage

```toml
[dependencies]
seshat-unicode = "0.3.1"
```

```rust
use seshat::unicode::Ucd;

fn main() {
    println!("🦀 is {}!", '🦀'.na());
}
```

### Check the Unicode Version
```rust
use seshat::unicode::UNICODE_VERSION;

fn main() {
    println!("{}", UNICODE_VERSION.to_string());
}
```

## Features


### Grapheme cluster break

```rust
use seshat::unicode::Segmentation;

fn main() {
    let s = "Hi, 👨🏾‍🤝‍👨🏿";
    for seg in s.break_graphemes() {
        println!("{}", seg);
    }
}
```

This will prints
```sh
$ cargo run
H
i
,
 
👨🏾‍🤝‍👨🏿
```

### Normalization

```rust
use seshat::unicode::Normalization;

fn main() {
    let s1 = "Å";
    println!("{:?}", s1.to_nfd()); // Will prints "A\u{30a}"

    let s2 = "㌀";
    println!("{}", s2.to_nfkd()); // Will prints アパート

    let s3 = "e\u{0301}";
    println!("{}", s3.to_nfc()); // Will prints é

    let s4 = "ｱｲｳｴｵ";
    assert_eq!("アイウエオ", s4.to_nfkc());
}
```

### Properties

```rust
use seshat::unicode::Ucd;

fn main() {
    let c = 'ʹ';    // U+0374 GREEK NUMERAL SIGN
    assert_eq!(c.xids(), true); // XID_Start property of the character.
}
```

For enumeration property,

```rust
use seshat::unicode::Ucd;
use seshat::unicode::props::Gc;

fn main() {
    assert_eq!('A'.gc(), Gc::Lu);
    assert_eq!('a'.gc(), Gc::Ll);
}
```


## Patches

0.2.1 - Exclude the `tools/` directory which should not be included when
publishing.


## Contribute

Add later.

## License

All logo images have copyright owned by their creators and should not be used
out of this project without permission.

The drawing part (writing goddess) by [Frybits Inc.](https://github.com/frybitsinc).

Seshat is developed under MIT License. For the detail, see the LICENSE file.
