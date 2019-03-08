[![Build Status](https://travis-ci.com/nervosnetwork/blake2b-rs.svg)](https://travis-ci.com/nervosnetwork/blake2b-rs)
[![Crate](https://img.shields.io/crates/v/blake2b-rs.svg)](https://crates.io/crates/blake2b-rs)
[![License]](#license)

[license]: https://img.shields.io/badge/License-MIT-green.svg

# blake2b-rs

The [blake2b-rs](https://crates.io/crates/blake2b-rs) crate is a safe Rust binding to [blake2](https://github.com/BLAKE2/BLAKE2).

## Building from Source

```bash
git clone https://github.com/nervosnetwork/blake2b-rs.git
cd blake2b-rs
cargo build
```

## Features
* SIMD optimization .
* Personal and Salt support.

## License

Licensed under the [LICENSE-MIT](http://opensource.org/licenses/MIT), as described in the [LICENSE](LICENSE) file.

### Third-party software

This crate includes copies and modifications of software developed by third parties:

* [BLAKE2](BLAKE2) is based on [BLAKE2](https://github.com/BLAKE2/BLAKE2), triple-licensed under the CC0, the OpenSSL Licence, or the Apache Public License 2.0. Choosing the Apache Public License 2.0.
* [Test Vectors](fixtures) is taken from [emilbayes/blake2b](https://github.com/emilbayes/blake2b), licensed under the [ISC](https://github.com/emilbayes/blake2b/blob/master/LICENSE)
