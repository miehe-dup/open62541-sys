# open62541-sys
open62541 Rust library crate

## Goals
bring the library [open62541](https://open62541.org/) to life in [Rust](https://www.rust-lang.org/) programs!

## What works
compiling the library

## What doesn't work
using the library

## Known issues
- open62541 git submodule must be cloned
- `bindgen` step parses libc parts, which seems unnecessary (check the generated `bindgen.rs`)

## Useful pointers
- Rust FFI: https://doc.rust-lang.org/nomicon/ffi.html
- Rust FFI for CMake projects: https://github.com/alexcrichton/rust-ffi-examples/tree/master/rust-to-cmake
- Rust bindgen: https://rust-lang.github.io/rust-bindgen/
