use bit_vec::BitVec;
use std::{error, fmt};

fn encode_value(value: usize) -> u8 {
  b"0123456789ABCDEFGHJKMNPQRSTVWXYZ"[value]
}

pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
  let mut bits = BitVec::from_bytes(input.as_ref());
  // add padding bits
  bits.grow((5 - bits.len() % 5) % 5, false);

  let buffer = (0..bits.len())
    .step_by(5)
    .map(|i| (i..i + 5).fold(0, |n, i| (n << 1) + bits[i] as usize))
    .map(encode_value)
    .collect::<Vec<_>>();

  String::from_utf8(buffer).expect("Invalid UTF-8 byte sequence")
}

#[derive(Debug, PartialEq, Eq)]
pub enum DecodeError {
  InvalidLength,
  InvalidSymbol(u8, usize),
}

impl fmt::Display for DecodeError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      DecodeError::InvalidLength => write!(f, "Invalid length byte sequence"),
      DecodeError::InvalidSymbol(byte, index) => write!(f, "Invalid symbol {} at offset {}", byte, index),
    }
  }
}

impl error::Error for DecodeError {}

fn decode_symbol(symbol: u8) -> Option<usize> {
  match symbol {
    b'0' | b'O' | b'o' => Some(0),
    b'1' | b'I' | b'i' | b'L' | b'l' => Some(1),
    b'2' => Some(2),
    b'3' => Some(3),
    b'4' => Some(4),
    b'5' => Some(5),
    b'6' => Some(6),
    b'7' => Some(7),
    b'8' => Some(8),
    b'9' => Some(9),
    b'A' | b'a' => Some(10),
    b'B' | b'b' => Some(11),
    b'C' | b'c' => Some(12),
    b'D' | b'd' => Some(13),
    b'E' | b'e' => Some(14),
    b'F' | b'f' => Some(15),
    b'G' | b'g' => Some(16),
    b'H' | b'h' => Some(17),
    b'J' | b'j' => Some(18),
    b'K' | b'k' => Some(19),
    b'M' | b'm' => Some(20),
    b'N' | b'n' => Some(21),
    b'P' | b'p' => Some(22),
    b'Q' | b'q' => Some(23),
    b'R' | b'r' => Some(24),
    b'S' | b's' => Some(25),
    b'T' | b't' => Some(26),
    b'V' | b'v' => Some(27),
    b'W' | b'w' => Some(28),
    b'X' | b'x' => Some(29),
    b'Y' | b'y' => Some(30),
    b'Z' | b'z' => Some(31),
    _ => None,
  }
}

pub fn decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, DecodeError> {
  let bytes = input.as_ref();
  if bytes.len() == 1 {
    return Err(DecodeError::InvalidLength);
  }

  let mut buffer = BitVec::with_capacity(bytes.len() * 5);
  for (index, symbol) in bytes.into_iter().copied().enumerate() {
    let value = decode_symbol(symbol).ok_or(DecodeError::InvalidSymbol(symbol, index))?;
    for i in (0..5).rev() {
      buffer.push(value >> i & 0x1 != 0);
    }
  }
  // omit padding bits
  buffer.truncate(buffer.len() / 8 * 8);

  Ok(buffer.to_bytes())
}
