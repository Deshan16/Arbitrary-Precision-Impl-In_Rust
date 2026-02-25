use std::cmp::Ordering;

use super::{BigFloat, BigInt, super::bigint::BASE};

impl BigFloat {
    pub fn abs(&self) -> Self {
        if self.is_zero() { self.clone() }
        else {
            let mut res = self.clone();
            res.sign = 1;
            res
        }
    }
}

impl BigFloat {
    pub(in crate::bigfloat) fn normalize(&mut self) {
        while !self.mantissa.is_zero() && self.mantissa.rem_u32(10) == 0 {
            self.mantissa = self.mantissa.div_mod_u32(10).expect("div by 10 should not fail").0;
            self.exp10 += 1;
        }
        
        if self.mantissa.is_zero() {
            self.sign = 0;
            self.exp10 = 0;
        }
    }
    
    pub(in crate::bigfloat) fn neg(&self) -> Self {
        if self.sign == 0 { Self::zero(self.precision) }
        else {
            let mut res = self.clone();
            res.sign *= -1;
            res
        }
    }
    
    pub fn trim_to_prec(&mut self) {
        if self.sign == 0 { return; }
        
        let s = self.mantissa.to_string();
        let len = s.len();
        if len <= self.precision { return; }
        
        let cut = len - self.precision;
        
        let mut q = self.mantissa.clone();
        let mut remaining = cut.saturating_sub(1);
        
        while remaining >= 9 {
            q = q.div_mod_u32(BASE).unwrap().0;
            remaining -= 9;
        }
        
        if remaining > 0 {
            let mut p = 1u32;
            for _ in 0..remaining {
                p *= 10;
            }
            q = q.div_mod_u32(p).unwrap().0;
        }
        
        let (mut q2, round_digit) = q.div_mod_u32(10).unwrap();
        if round_digit >= 5 {
            q2 = q2.add(&BigInt::one());
        }
        
        self.mantissa = q2;
        self.exp10 += cut as i64;
        self.normalize();
    }
    
    pub(in crate::bigfloat) fn round_from_remainder(q: BigInt, r: BigInt, den: &BigInt) -> BigInt {
        if r.is_zero() { return q; }
        
        let twice_r = r.add(&r);
        
        if twice_r.comp_abs(den) != Ordering::Less {
            q.add(&BigInt::one())
        } else {
            q
        }
    }
    
    pub(in crate::bigfloat) fn guard_digits_for_precision(&self) -> usize {
        if self.precision <= 1 {
            3
        } else {
            let mut p = self.precision;
            let mut digits = 0usize;
            while p > 0 {
                p /= 5;
                digits += 1;
            }
            (digits + 2).max(3)
        }
    }
    
    pub(in crate::bigfloat) fn with_precision(&self, new_prec: usize) -> Self {
        let mut v = self.clone();
        v.precision = new_prec;
        v.trim_to_prec();
        v
    }
    
    pub(in crate::bigfloat) fn round_to_bigint(&self) -> BigInt {
        if self.exp10 >= 0 {
            return self.mantissa.mul_pow10(self.exp10 as usize);
        }
        
        let shift = (-self.exp10) as usize;
        let divsor = BigInt::one().mul_pow10(shift);
        let (q, r) = self.mantissa.div_mod(&divsor).expect("Divisor is never zero");
        let r2 = &r + &r;
        
        if r2.comp_abs(&divsor).is_ge() {
            if self.mantissa.is_negative() {
                q.sub(&BigInt::one())
            } else {
                q.add(&BigInt::one())
            }
        } else {
            q
        }
    }
}