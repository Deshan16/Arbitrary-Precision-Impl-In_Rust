mod utils;
mod parse;
mod error;
mod comp;
mod traits;
mod math_op;
mod math;
pub use error::BigFloatError;

use super::bigint::BigInt;

#[derive(Debug, Clone)]
pub struct BigFloat {
    pub(crate) sign: i8,
    pub(crate) mantissa: BigInt,
    pub(crate) exp10: i64,
    pub(crate) precision: usize
}

impl BigFloat {
    pub fn new(sign: i8, mut mant: BigInt, exp10: i64, prec: usize) -> Self {
        if mant.is_zero() || sign == 0 {
            return Self { sign: 0, mantissa: BigInt::zero(), exp10: 0, precision: prec };
        }
        
        mant = mant.abs();
        
        let mut bf = Self{ sign: if sign < 0 {-1} else { 1 }, mantissa: mant, exp10, precision: prec };
        bf.normalize();
        bf
    }
    
    pub fn zero(prec: usize) -> Self {
        Self { sign: 0, mantissa: BigInt::zero(), exp10: 0, precision: prec }
    }
    
    pub fn one(prec: usize) -> Self {
        Self::new(1, BigInt::one(), 0, prec)
    }
}