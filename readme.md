# obfnstr-rs

This is a simple rust string obfuscation library that aims to obfuscate as many strings as possible in fn.

Why as much as possible? Because macro expressions are difficult to handle.

## Example

When a fn is marked with `#[obfnstr]`, the string expression appearing in the fn body will be obfuscated.

```rust
#[obfnstr]
fn main() {
    test();
    test1("Hello".to_string());
}

#[obfnstr]
fn test() {
    println!("Hello, world!");

    let _a = "String A";
    let _b = "String B".to_string();
    let _c = 123;
    let _arr = ["Hello", "Ok"];

    let name = "Tom";
    println!("Welcome {}", name);
}

#[obfnstr]
fn test1(str: String) -> (String, String) {
    (str, "Test1")
}
```

The following is the result after `cargo expand`.

```rust
#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use obfnstr_macros::obfnstr;
fn main() {
    test();
    test1(obfnstr::obfuscate(&[20u8, 127u8, 83u8, 238u8, 242u8]).to_string());
}
fn test() {
    {
        ::std::io::_print(format_args!("Hello, world!\n"));
    };
    let _a = obfnstr::obfuscate(&[20u8, 127u8, 83u8, 238u8, 242u8]);
    let _b = obfnstr::obfuscate(&[62u8, 43u8]).to_string();
    let _c = 123;
    let _arr = [
        obfnstr::obfuscate(&[52u8, 127u8, 83u8, 238u8, 242u8]),
        obfnstr::obfuscate(&[51u8, 113u8]),
    ];
    let name = obfnstr::obfuscate(&[8u8, 117u8, 82u8]);
    {
        ::std::io::_print(format_args!("Welcome {0}\n", name));
    };
}
fn test1(str: String) -> (String, String) {
    (str, obfnstr::obfuscate(&[8u8, 127u8, 76u8, 246u8, 172u8]))
}
```

> Note: That since string retrieval is always done by calling `obfnstr::obfuscate`, if you consider performance, perhaps you should consider other solutions.
