# BRA

 [![Latest Version](https://img.shields.io/crates/v/bra.svg)](https://crates.io/crates/bra) [![Build Status](https://travis-ci.org/Enet4/bra-rs.svg?branch=master)](https://travis-ci.org/Enet4/bra-rs) [![dependency status](https://deps.rs/repo/github/Enet4/bra-rs/status.svg)](https://deps.rs/repo/github/Enet4/bra-rs) ![Minimum Rust Version 1.31](https://img.shields.io/badge/Minimum%20Rust%20Version-1.31-green.svg)

Buffered Random Access (BRA) provides easy random memory access to a sequential source of data in Rust. This is achieved by eagerly retaining all memory read from a given source.

## Example


```rust
use std::io::Read;
use bra::EagerBufRead;

let reader = get_reader();
let mut reader = EagerBufRead::new(reader);

// random access to bytes!
let k: u8 = reader.get(12)?;
// random slicing!
let s: &[u8] = reader.slice(20..48)?;
assert_eq!(s.len(), 28);
// also functions as a buffered reader
let mut chunk = [0; 20];
reader.read_exact(&mut chunk)?;
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
