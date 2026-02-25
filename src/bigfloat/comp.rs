use std::cmp::Ordering;

use  super::BigFloat;

impl BigFloat {
    pub fn is_zero(&self) -> bool {
        self.sign == 0 || self.mantissa.is_zero()
    }
    
    pub fn is_negative(&self) -> bool {
        self.sign < 0 && !self.is_zero()
    }
    
    pub fn comp(&self, other: &Self) -> Ordering {
        if self.is_zero() && other.is_zero() { return Ordering::Equal; }
        if self.is_zero() { return if other.is_negative() { Ordering::Greater } else { Ordering::Less }; }
        if other.is_zero() { return if self.is_negative() { Ordering::Less } else { Ordering::Greater }; }
        
        if self.sign != other.sign {
            return self.sign.cmp(&other.sign);
        }
        
        let ord = self.comp_abs(other);
        if self.sign < 0 { ord.reverse() } else { ord }
    }
}

impl BigFloat {
    pub(crate) fn sci_exp10_abs(&self) -> i64 {
        if self.is_zero() { return i64::MIN; }
        let len = self.mantissa.decimal_len() as i64;
        self.exp10 + (len - 1)
    }
    
    pub(crate) fn comp_abs(&self, other: &Self) -> Ordering {
        if self.is_zero() && other.is_zero() { return Ordering::Equal; }
        if self.is_zero() { return Ordering::Less; }
        if other.is_zero() { return Ordering::Greater; }
        
        let a_exp = self.sci_exp10_abs();
        let b_exp = other.sci_exp10_abs();
        if a_exp != b_exp { return a_exp.cmp(&b_exp); }
        
        let a = self.mantissa.to_string();
        let b = other.mantissa.to_string();
        cmp_scaled_decimal_strings(&a, b.len() - 1, &b, a.len() - 1)
    }
}

pub(in crate::bigfloat) fn cmp_scaled_decimal_strings(s1: &str, z1: usize, s2: &str, z2: usize) -> Ordering {
    let total_len = s1.len() + z1;
    let total_len2 = s2.len() + z2;
    
    if total_len != total_len2 {
        return total_len.cmp(&total_len2);
    }
    
    for i in 0..total_len {
        let c1 = if i < s1.len() { s1.as_bytes()[i] } else { b'0' };
        let c2 = if i < s2.len() { s2.as_bytes()[i] } else { b'0' };
        if c1 != c2 {
            return c1.cmp(&c2);
        }
    }
    
    Ordering::Equal
} 