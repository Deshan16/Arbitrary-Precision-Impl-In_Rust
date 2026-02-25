use std::{cmp::Ordering, fmt, ops::{Add, Div, Mul, Neg, Sub}};
use super::BigInt;

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.comp(other) == Ordering::Equal
    }
}

impl Eq for BigInt {}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.comp(other))
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        BigInt::comp(&self, other)
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.sign == 0 {
            return f.write_str("0");
        }
        
        let mut result = String::new();
        
        if self.sign == -1 {
            result.push('-');
        }
        
        for (i, &digit) in self.digits.iter().rev().enumerate() {
            if i == 0 {
                result += &digit.to_string();
            } else {
                result += &format!("{:09}", digit);
            }
        }
        
        f.write_str(&result)
    }
}

impl Add for &BigInt {
    type Output = BigInt;

    fn add(self, rhs: Self) -> Self::Output {
        BigInt::add(&self, rhs)
    }
}

impl Sub for &BigInt {
    type Output = BigInt;

    fn sub(self, rhs: Self) -> Self::Output {
        BigInt::sub(&self, rhs)
    }
}

impl Mul for &BigInt {
    type Output = BigInt;

    fn mul(self, rhs: Self) -> Self::Output {
        BigInt::mul(&self, rhs)
    }
}

impl Div for &BigInt {
    type Output = BigInt;

    fn div(self, rhs: Self) -> Self::Output {
        BigInt::div_mod(self, rhs).expect("BigInt divide by zero").0
    }
}

impl Neg for &BigInt {
    type Output = BigInt;

    fn neg(self) -> Self::Output {
        BigInt::neg(&self)
    }
}