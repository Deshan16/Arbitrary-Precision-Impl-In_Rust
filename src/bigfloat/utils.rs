use std::cmp::Ordering;

use super::{BigFloat, BigInt, super::bigint::BASE};

#[derive(Clone, Copy, Debug)]
pub enum RoundingMode {
    TowardZero,
    HalfUp,
    HalfEven,
    TowardPosInf,
    TowardNegInf,
}

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
        self.trim_to_prec_with(RoundingMode::HalfEven);
    }
    
    pub(crate) fn trim_work(&mut self) {
        self.trim_to_prec_with(RoundingMode::TowardZero);
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
    
    pub(in crate::bigfloat) fn guard_digits_for_precision(prec: usize) -> usize {
        if prec <= 8 {
            16
        } else if prec <= 32 {
            24
        } else if prec <= 128 {
            48
        } else if prec <= 512 {
            64
        } else {
            let mut p = prec;
            let mut dec_digits = 1usize;
            while p >= 10 {
                p /= 10;
                dec_digits += 1;
            }
            80 + dec_digits * 6
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
    
    pub fn trim_to_prec_with(&mut self, mode: RoundingMode) {
        if self.sign == 0 { return; }

        let len = self.mantissa.decimal_len();
        if len <= self.precision { return; }

        let cut = len - self.precision;
        let den = BigInt::one().mul_pow10(cut); // 10^cut

        let (q, r) = self.mantissa.div_mod(&den).expect("10^cut != 0");

        let q = Self::round_from_remainder_mode(q, r, &den, self.sign, mode);

        self.mantissa = q;
        self.exp10 += cut as i64;
        self.normalize();
    }
    
    pub(in crate::bigfloat) fn round_from_remainder_mode(q: BigInt, r: BigInt, den: &BigInt, sign: i8, mode: RoundingMode) -> BigInt {
        if r.is_zero() { return q; }
        
        match mode {
            RoundingMode::TowardZero => q,
            RoundingMode::HalfUp => {
                let twice_r = r.add(&r);
                if twice_r.comp_abs(den) != Ordering::Less { q.add(&BigInt::one()) } else { q }
            },
            RoundingMode::HalfEven => {
                let twice_r = r.add(&r);
                match twice_r.comp_abs(den) {
                    Ordering::Greater => q.add(&BigInt::one()),
                    Ordering::Less => q,
                    Ordering::Equal => {
                        // tie: round so last digit becomes even
                        if q.rem_u32(2) == 1 { q.add(&BigInt::one()) } else { q }
                    }
                }
            }
    
            RoundingMode::TowardPosInf => {
                if sign > 0 { q.add(&BigInt::one()) } else { q }
            }
    
            RoundingMode::TowardNegInf => {
                if sign < 0 { q.add(&BigInt::one()) } else { q }
            }
        }
    }
}
