Seshat
=========
[![crates.io](https://img.shields.io/crates/v/seshat-unicode.svg)](https://crates.io/crates/seshat-unicode)

![logo](https://raw.githubusercontent.com/hardboiled65/Seshat/master/docs/seshat-logo.png)

A Unicode Library for Rust.

Demo
-----
! _This demo is using C++ version of Seshat._
[Seshat Web Demo](https://seshat-demo.herokuapp.com)

Introduction
---------
Seshat (pronounce as Sehs-hat) is a Unicode library that written in Rust.
It provides many of Unicode character data and standard algorithms.
The goal of this project is to provide a ICU-like library in Rust.

Usage
---------
```toml
[dependencies]
seshat-unicode = "0.0.7"
```

```rust
use seshat::unicode::Ucd;

fn main() {
    println!("🦀 is {}!", '🦀'.na());
}
```

Features
---------
Add later.

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

Contribute
---------
Add later.

License
---------
All logo images are copyright [Frybits Inc.](https://github.com/frybitsinc)
and should not be used out of this project without permission.

Seshat is developed under MIT License. For the detail, see the LICENSE file.
