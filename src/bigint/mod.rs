mod parse;
mod utils;
mod comp;
mod traits;
mod error;
mod math_op;
pub use error::BigIntError;

pub(crate) const BASE: u32 = 1_000_000_000;
pub(crate) const BASE_DIGITS: usize = 9;

#[derive(Debug, Clone)]
pub struct BigInt {
    pub(in crate::bigint) sign: i8,
    pub(in crate::bigint) digits: Vec<u32>
}

impl BigInt {
    pub fn zero() -> Self {
        Self { sign: 0, digits: Vec::new() }
    }
    
    pub fn one() -> Self {
        Self { sign: 1, digits: vec![1] }
    }
    
    pub fn from_str(s: &str) -> Result<Self, BigIntError> {
        parse::from_str(s)
    }
}