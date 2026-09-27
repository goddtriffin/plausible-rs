use serde::{Deserialize, Serialize};

/// Custom properties only accepts scalar values such as strings, numbers and booleans.
/// Data structures such as objects, arrays etc. aren't accepted.
// Implementation on how to constrain types easily from: https://stackoverflow.com/a/52582432/11767294
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropValue {
    // string
    String(String),

    // bool
    Bool(bool),

    // numbers
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),

    F32(f32),
    F64(f64),
}

impl From<String> for PropValue {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<bool> for PropValue {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}

impl From<u8> for PropValue {
    fn from(u: u8) -> Self {
        Self::U8(u)
    }
}

impl From<u16> for PropValue {
    fn from(u: u16) -> Self {
        Self::U16(u)
    }
}

impl From<u32> for PropValue {
    fn from(u: u32) -> Self {
        Self::U32(u)
    }
}

impl From<u64> for PropValue {
    fn from(u: u64) -> Self {
        Self::U64(u)
    }
}

impl From<u128> for PropValue {
    fn from(u: u128) -> Self {
        Self::U128(u)
    }
}

impl From<usize> for PropValue {
    fn from(u: usize) -> Self {
        Self::Usize(u)
    }
}

impl From<i8> for PropValue {
    fn from(i: i8) -> Self {
        Self::I8(i)
    }
}

impl From<i16> for PropValue {
    fn from(i: i16) -> Self {
        Self::I16(i)
    }
}

impl From<i32> for PropValue {
    fn from(i: i32) -> Self {
        Self::I32(i)
    }
}

impl From<i64> for PropValue {
    fn from(i: i64) -> Self {
        Self::I64(i)
    }
}

impl From<i128> for PropValue {
    fn from(i: i128) -> Self {
        Self::I128(i)
    }
}

impl From<isize> for PropValue {
    fn from(i: isize) -> Self {
        Self::Isize(i)
    }
}

impl From<f32> for PropValue {
    fn from(f: f32) -> Self {
        Self::F32(f)
    }
}

impl From<f64> for PropValue {
    fn from(f: f64) -> Self {
        Self::F64(f)
    }
}
