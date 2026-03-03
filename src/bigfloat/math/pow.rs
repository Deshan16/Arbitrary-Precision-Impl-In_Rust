use super::super::{BigFloat, BigFloatError};

impl BigFloat {
    pub fn pow_u64(&self, mut n: u64) -> Self {
        let out_prec = self.precision;
        let guard = BigFloat::guard_digits_for_precision(out_prec);
        let work_prec = out_prec + guard;
        
        if n == 0 {
            return BigFloat::one(out_prec);
        }
        
        if self.is_zero() {
            return BigFloat::zero(out_prec);
        }
        
        let mut base = self.with_precision(work_prec);
        base.trim_to_prec();
        
        let mut result = BigFloat::from_with_prec(1u32, work_prec);
        
        while n > 0 {
            if (n & 1) == 1 {
                result = &result * &base;
                result.trim_to_prec();
            }
            
            n >>= 1;
            if n == 0 { break; }
            
            base = &base * &base;
            base.trim_to_prec();
        }
        result.precision = out_prec;
        result.trim_to_prec();
        result
    }
    
    pub fn pow_i32(&self, n: i32) -> Result<BigFloat, BigFloatError> {
        if n == 0 {
            return Ok(BigFloat::one(self.precision));
        }
        
        if n > 0 {
            return Ok(self.pow_u64(n as u64));
        }
        
        if self.is_zero() {
            return Err(BigFloatError::ZeroDivisionError);
        }
        
        let nn: i64 = -(n as i64);
        
        let pos = self.pow_u64(nn as u64);
        BigFloat::from_with_prec(1, self.precision).div(&pos)
    }
}