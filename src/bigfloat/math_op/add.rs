use super::super::BigFloat;

impl BigFloat {
    pub fn add(&self, other: &Self) -> Self {
        if self.sign == 0 { return other.clone(); }
        if other.sign == 0 { return self.clone(); }
        
        let e = self.exp10.min(other.exp10);
        
        let prec = self.precision.max(other.precision);
        
        let a_exp = self.sci_exp10_abs();
        let b_exp = other.sci_exp10_abs();
        let cutoff = prec as i64 + 2;
        
        if a_exp >= b_exp + cutoff {
            let mut out = self.clone();
            out.precision = prec;
            out.trim_to_prec();
            return out;
        }
        
        if b_exp >= a_exp + cutoff {
            let mut out = other.clone();
            out.precision = prec;
            out.trim_to_prec();
            return out;
        }
        
        let a_scale = (self.exp10 - e) as usize;
        let b_scale = (other.exp10 - e) as usize;
        
        let mut a = self.mantissa.mul_pow10(a_scale);
        let mut b = other.mantissa.mul_pow10(b_scale);
        
        if self.sign < 0 { a = a.neg(); }
        if other.sign < 0 { b = b.neg(); }
        
        let sum = a.add(&b);
        
        let sign = if sum.is_zero() { 0 } else if sum.is_negative() { -1 } else { 1 };
        
        let mut res = BigFloat::new(sign, sum.abs(), e, prec);
        res.trim_to_prec();
        res
    }
}