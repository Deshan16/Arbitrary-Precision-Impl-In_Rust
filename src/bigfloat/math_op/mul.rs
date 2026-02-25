use super::super::{BigFloat, BigInt};

impl BigFloat {
    pub fn mul(&self, other: &Self) -> Self {
        if self.sign == 0 || other.sign == 0 {
            return Self { sign: 0, mantissa: BigInt::zero(), exp10: 0, precision: self.precision.max(other.precision) };
        }
        
        let sign = self.sign * other.sign;
        let mant = self.mantissa.mul(&other.mantissa);
        let exp10 = self.exp10 + other.exp10;
        
        let mut res = BigFloat::new(sign, mant, exp10, self.precision.max(other.precision));
        res.trim_to_prec();
        res
    }
    
    pub(crate) fn mul_u32(&self, m: u32) -> Self {
        if m == 0 || self.is_zero() {
            return Self::zero(self.precision);
        }
        
        let mut res = BigFloat::new(self.sign, self.mantissa.mul_u32(m), self.exp10, self.precision);
        res.normalize();
        res
    }
    
    pub(crate) fn mul_u64(&self, m: u64) -> Self {
        if m == 0 || self.is_zero() {
            return Self::zero(self.precision);
        }
        
        let mut res = BigFloat::new(self.sign, self.mantissa.mul_u64(m), self.exp10, self.precision);
        res.normalize();
        res
    }
}