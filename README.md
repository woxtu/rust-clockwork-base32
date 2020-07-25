# rust-clockwork-base32

Rust implementation of [Clockwork Base32](https://gist.github.com/szktty/228f85794e4187882a77734c89c384a8) which is a Base32 variant inspired by Crockford's Base32.

## Install

Adding the following to the Cargo.toml in your project:

```toml
[dependencies]
clockwork-base32 = { git = "https://github.com/woxtu/rust-clockwork-base32", tag = "0.1.0" }
```

## Usage

```rust
extern crate clockwork_base32 as base32;

assert_eq!(base32::encode("Hello, World!"), "91JPRV3F5GG5EVVJDHJ22");
assert_eq!(base32::decode("91JPRV3F5GG5EVVJDHJ22").unwrap(), "Hello, World!".as_bytes());
```

## License

Copyright (c) 2020 woxtu

Licensed under the MIT license.
