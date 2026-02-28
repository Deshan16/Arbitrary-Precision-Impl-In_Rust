use super::super::{BigFloat, BigFloatError, BigInt};

impl BigFloat {
    pub fn div(&self, other: &Self) -> Result<Self, BigFloatError> {
        if other.sign == 0 || other.mantissa.is_zero() {
            return Err(BigFloatError::ZeroDivisionError);
        }
        
        if self.sign == 0 || self.mantissa.is_zero() {
            return Ok(BigFloat::new(0, BigInt::zero(), 0, self.precision.max(other.precision)));
        }
        
        let prec = self.precision.max(other.precision);
        let guard = self.guard_digits_for_precision(prec);
        let scale = prec + guard;
        
        let scaled = self.mantissa.mul_pow10(scale);
        
        let (q, r) = scaled.div_mod(&other.mantissa)?;
        
        let q = BigFloat::round_from_remainder(q, r, &other.mantissa);
        
        let sign = self.sign * other.sign;
        let exp10 = self.exp10 - other.exp10 - scale as i64;
        
        let mut res = BigFloat::new(sign, q, exp10, prec);
        res.trim_to_prec();
        Ok(res)
    }
    
    pub fn div_u32(&self, d: u32) -> Result<Self, BigFloatError> {
        if d == 0 {
            return Err(BigFloatError::ZeroDivisionError);
        }
        
        if self.is_zero() {
            return Ok(Self::zero(self.precision));
        }
        
        let scale = self.precision + self.guard_digits_for_precision(self.precision);
        
        let scaled = self.mantissa.mul_pow10(scale);
        let (q, r) = scaled.div_mod_u32(d)?;
        let q = BigFloat::round_from_remainder(q, BigInt::from(r), &BigInt::from(d));
        
        let mut res = BigFloat::new(self.sign, q, self.exp10 - scale as i64, self.precision);
        res.trim_to_prec();
        Ok(res)
    }
    
    pub fn div_u64(&self, d: u64) -> Result<Self, BigFloatError> {
        if d == 0 {
            return Err(BigFloatError::ZeroDivisionError);
        }
        
        if self.is_zero() {
            return Ok(Self::zero(self.precision));
        }
        
        let scale = self.precision + self.guard_digits_for_precision(self.precision);
        
        let scaled = self.mantissa.mul_pow10(scale);
        let (q, r) = scaled.div_mod_u64(d)?;
        let q = BigFloat::round_from_remainder(q, BigInt::from(r), &BigInt::from(d));
        
        let mut res = BigFloat::new(self.sign, q, self.exp10 - scale as i64, self.precision);
        res.trim_to_prec();
        Ok(res)
    }
}