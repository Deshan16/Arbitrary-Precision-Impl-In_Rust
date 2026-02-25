use std::cmp::Ordering;

use super::super::{BigInt, BASE};

impl BigInt {
    pub fn sub(&self, other: &Self) -> Self {
        match (self.sign, other.sign) {
            (_, 0) => self.clone(),
            (0, _) => {
                let mut r = other.clone();
                r.sign = r.sign * -1;
                r
            },
            
            (1, -1) => {
                let mut r = self.add_abs(other);
                r.sign = 1;
                r
            },
            
            (-1, 1) => {
                let mut r = self.add_abs(other);
                r.sign = -1;
                r
            },
            
            (1, 1) => {
                match self.comp_abs(other) {
                    Ordering::Greater => {
                        let mut r = self.sub_abs(other);
                        r.sign = 1;
                        r
                    },
                    Ordering::Less => {
                        let mut r = other.sub_abs(self);
                        r.sign = -1;
                        r
                    },
                    Ordering::Equal => Self { sign: 0, digits: Vec::new() },
                }
            }
            
            (-1, -1) => {
                match self.comp_abs(other) {
                    Ordering::Greater => {
                        let mut r = self.sub_abs(other);
                        r.sign = -1;
                        r
                    },
                    Ordering::Less => {
                        let mut r = other.sub_abs(self);
                        r.sign = 1;
                        r
                    },
                    Ordering::Equal => Self { sign: 0, digits: Vec::new() },
                }
            }
            
            _ => unreachable!()
        }
    }
    
    pub(in crate::bigint) fn sub_abs(&self, other: &Self) -> Self {
        assert!(self.comp_abs(other) != Ordering::Less);
        
        let mut result = Vec::<u32>::new();
        let mut borrow: i64 = 0;
        
        for i in 0..self.digits.len() {
            let a = self.digits[i] as i64;
            let b = *other.digits.get(i).unwrap_or(&0) as i64;
            
            let mut diff = a - b - borrow;
            
            if diff < 0 {
                diff += BASE as i64;
                borrow = 1;
            } else { borrow = 0 }
            
            result.push(diff as u32);
        }
        
        let mut res = Self { sign: 1, digits: result };
        res.normalize();
        res
    }
}