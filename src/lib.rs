#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
//! Fermion
//!
//! A super-compact binary encoding format ideal for constrained no_std environments.

use serde::{Deserialize, Serialize};

mod de;
mod ser;

#[cfg(test)]
mod pathological;

/// Errors that might occur during serialization/deserialization
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Insufficient space in buffer
    OutOfSpace,
    /// Invalid byte encoding
    InvalidRepresentation,
    /// Enum with more than
    TooManyVariants,
    /// Data type not supported
    NotSupported,
    /// A byte slice or a `str` exceeded maximum length
    LengthExceeded,
    /// Custom error
    Custom,
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, _f: &mut core::fmt::Formatter) -> core::fmt::Result {
        Ok(())
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(_msg: T) -> Error {
        Error::Custom
    }
}

impl serde::de::Error for Error {
    fn custom<T>(_msg: T) -> Error {
        Error::Custom
    }
}

/// Encodes a value into provided buffer
pub fn encode<T: Serialize>(value: &T, buf: &mut [u8]) -> Result<(), Error> {
    let mut serializer = ser::Serializer::new(buf);
    value.serialize(&mut serializer)
}

/// Decodes a value from provided buffer
pub fn decode<'de, T: Deserialize<'de>>(buf: &'de [u8]) -> Result<T, Error> {
    let mut deserializer = de::Deserializer::new(buf);
    T::deserialize(&mut deserializer)
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::pathological;

    #[test]
    fn test_u8() {
        let orig: u8 = 42;

        let mut buf = [0u8];
        encode(&orig, &mut buf).unwrap();

        let decoded: u8 = decode(&buf).unwrap();

        assert_eq!(orig, decoded);
    }

    #[test]
    fn test_multiple() {
        #[derive(PartialEq, Debug, Serialize, Deserialize)]
        struct Test {
            a: u8,
            b: u16,
        }

        let orig = Test { a: 1, b: 2 };

        let mut buf = [0u8; 32];
        encode(&orig, &mut buf).unwrap();

        let decoded = decode(&buf).unwrap();
        assert_eq!(orig, decoded);
    }

    #[test]
    fn test_char() {
        let orig = '⚑';

        let mut buf = [0u8; 4];
        encode(&orig, &mut buf).unwrap();

        let decoded: char = decode(&buf).unwrap();
        assert_eq!(orig, decoded);
    }

    #[test]
    fn test_borrow_bytes() {
        #[derive(PartialEq, Debug, Serialize, Deserialize)]
        struct Test<'a>(&'a [u8]);
        let orig = Test(&[1, 2, 3]);

        let mut buf = [0u8; 5];
        encode(&orig, &mut buf).unwrap();

        let decoded = decode(&buf).unwrap();
        assert_eq!(orig, decoded);
    }

    #[test]
    fn test_borrow_str() {
        #[derive(PartialEq, Debug, Serialize, Deserialize)]
        struct Test<'a>(&'a str);
        let orig = Test("abc");

        let mut buf = [0u8; 5];
        encode(&orig, &mut buf).unwrap();

        let decoded = decode(&buf).unwrap();
        assert_eq!(orig, decoded);
    }

    #[test]
    fn test_insufficient_buffer_write() {
        #[derive(PartialEq, Debug, Serialize, Deserialize)]
        struct Test<'a> {
            byte: u8,
            slice: &'a [u8],
            number: u16,
        }

        let orig = Test {
            byte: 42,
            slice: &[1, 2, 3],
            number: 3,
        };

        let mut buf = [0u8; 7];
        assert_eq!(encode(&orig, &mut buf), Err(Error::OutOfSpace))
    }

    #[test]
    fn test_insufficient_buffer_read() {
        let buf: [u8; 0] = Default::default();

        assert_eq!(decode::<u16>(&buf), Err(Error::OutOfSpace))
    }

    #[test]
    fn too_many_variants() {
        let orig = pathological::TooMany::A256;

        let mut buf = [0u8; 32];

        assert_eq!(encode(&orig, &mut buf), Err(Error::TooManyVariants))
    }

    #[test]
    fn too_long_bytestring() {
        #[derive(PartialEq, Debug, Serialize, Deserialize)]

        struct Test<'a>(&'a [u8]);
        let orig = Test(&pathological::LONG_BYTESTRING);

        let mut buf = [0u8; core::u16::MAX as usize + 2];

        encode(&orig, &mut buf).unwrap();

        let decoded = decode(&buf).unwrap();
        assert_eq!(orig, decoded);

        let bork = Test(&pathological::TOO_LONG_BYTESTRING);

        assert_eq!(encode(&bork, &mut buf), Err(Error::LengthExceeded))
    }

    #[test]
    fn monster_struct() {
        #[derive(PartialEq, Debug, Serialize, Deserialize)]
        struct A {
            a: ((u32, u16), usize),
            b: Option<C>,
        }

        #[derive(PartialEq, Debug, Serialize, Deserialize)]
        struct C(());

        #[derive(PartialEq, Debug, Serialize, Deserialize)]
        enum D<'a> {
            A(u32),
            B(&'a [u8]),
        }

        #[derive(PartialEq, Debug, Serialize, Deserialize)]
        enum V {
            A,
            B,
        }

        #[derive(PartialEq, Debug, Serialize, Deserialize)]
        struct X(u128, u32, u8);

        #[derive(PartialEq, Debug, Serialize, Deserialize)]
        enum E {
            S { r: u8, g: u8, b: u8 },
        }

        #[derive(PartialEq, Debug, Serialize, Deserialize)]
        struct B<'a> {
            a: A,
            b: &'a [u8],
            c: [u32; 5],
            d: [A; 2],
            e: (u8, u16, u32, u64, u128),
            e2: (i8, i16, i32, i64, i128),
            f: D<'a>,
            g: (bool, bool),
            h: char,
            i: &'a str,
            j: V,
            k: X,
            l: E,
            m: (),
        }

        let orig = B {
            a: A {
                a: ((288, 328), 3280),
                b: Some(C(())),
            },
            b: &[0, 2, 3],
            c: [0, 1, 2, 3, 4],
            d: [
                A {
                    a: ((2, 3), 8),
                    b: None,
                },
                A {
                    a: ((1, 2), 7),
                    b: Some(C(())),
                },
            ],
            e: (
                core::u8::MAX,
                core::u16::MAX,
                core::u32::MAX,
                core::u64::MAX,
                core::u128::MAX,
            ),
            e2: (
                core::i8::MIN,
                core::i16::MIN,
                core::i32::MIN,
                core::i64::MIN,
                core::i128::MIN,
            ),
            f: D::B(&[0, 1, 89]),
            g: (true, false),
            h: '⚑',
            i: "hello world",
            j: V::A,
            k: X(0, 1, 2),
            l: E::S { r: 255, g: 0, b: 0 },
            m: (),
        };

        let mut buf = [0u8; 256];
        encode(&orig, &mut buf).unwrap();

        let decoded = decode(&buf).unwrap();
        assert_eq!(orig, decoded);
    }
}
