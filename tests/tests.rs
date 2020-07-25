extern crate clockwork_base32 as base32;

use base32::DecodeError;

#[test]
fn encode_empty() {
  assert_eq!(base32::encode(""), "");
}

#[test]
fn encode_string() {
  assert_eq!(base32::encode("Hello, world!"), "91JPRV3F5GG7EVVJDHJ22");
}

#[test]
fn decode_empty() {
  assert_eq!(base32::decode("").unwrap(), "".as_bytes());
}

#[test]
fn decode_string() {
  assert_eq!(base32::decode("91JPRV3F5GG7EVVJDHJ22").unwrap(), "Hello, world!".as_bytes());
  assert_eq!(base32::decode("9ljprv3f5gg7evvjdhj22").unwrap(), "Hello, world!".as_bytes());
}

#[test]
fn decode_invalid_length() {
  assert_eq!(base32::decode("0").unwrap_err(), DecodeError::InvalidLength);
}

#[test]
fn decode_invalid_symbol() {
  assert_eq!(base32::decode("uU*=").unwrap_err(), DecodeError::InvalidSymbol(b'u', 0));
}
