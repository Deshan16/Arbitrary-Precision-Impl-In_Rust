use std::{cmp::Ordering, fmt, ops::{Add, Div, Mul, Neg, Sub}};

use super::BigFloat;

impl fmt::Display for BigFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.sign == 0 {
            return f.write_str("0")
        }
        
        let mut s = self.mantissa.to_string();
        
        let sign = if self.sign < 0 { "-" } else {""};
        
        if self.exp10 >= 0 {
            s.push_str(&"0".repeat(self.exp10 as usize));
            s = format!("{}{}", sign, s);
        } else {
            let frac = -self.exp10 as usize;
            if s.len() > frac {
                s = format!("{}{}.{}", sign, &s[..s.len() - frac], &s[s.len() - frac..]);
            } else {
                s = format!("{}0.{}{}", sign, "0".repeat(frac - s.len()), s);
            }
        }
        
        f.write_str(&s)
    }
}

impl PartialEq for BigFloat {
    fn eq(&self, other: &Self) -> bool {
        self.comp(other) == Ordering::Equal
    }
}

impl Eq for BigFloat {}

impl PartialOrd for BigFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.comp(other))
    }
}

impl Ord for BigFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        BigFloat::comp(&self, other)
    }
}

impl Add for &BigFloat {
    type Output = BigFloat;
    
    fn add(self, rhs: Self) -> Self::Output {
        BigFloat::add(&self, rhs)
    }
}

impl Sub for &BigFloat {
    type Output = BigFloat;
    
    fn sub(self, rhs: Self) -> Self::Output {
        BigFloat::sub(&self, rhs)
    }
}

impl Mul for &BigFloat {
    type Output = BigFloat;
    
    fn mul(self, rhs: Self) -> Self::Output {
        BigFloat::mul(&self, rhs)
    }
}

impl Div for &BigFloat {
    type Output = BigFloat;
    
    fn div(self, rhs: Self) -> Self::Output {
        BigFloat::div(&self, rhs).expect("BigFloat divide by zero")
    }
}

impl Neg for BigFloat {
    type Output = BigFloat;

    fn neg(self) -> Self::Output {
        BigFloat::neg(&self)
    }
}