use std::cmp::{self, Ordering};
use super::super::{BigInt, BASE};


impl BigInt {
    pub fn add(&self, other: &Self) -> Self {
        match (self.sign, other.sign) {
            (0, _) => other.clone(),
            (_, 0) => self.clone(),
            
            (1, 1) => {
                let mut res = self.add_abs(other);
                res.sign = 1;
                res
            },
            
            (-1, -1) => {
                let mut res = self.add_abs(other);
                res.sign = -1;
                res
            },
            
            (1, -1) => {
                match self.comp_abs(other) {
                    Ordering::Greater => {
                        let mut res = self.sub_abs(other);
                        res.sign = 1;
                        res
                    },
                    Ordering::Less => {
                        let mut res = other.sub_abs(self);
                        res.sign = -1;
                        res
                    },
                    Ordering::Equal => Self::zero(),
                }
            },
            
            (-1, 1) => other.add(self),
            
            _ => unreachable!()
        }
    }
    
    pub(in crate::bigint) fn add_abs(&self, other: &Self) -> Self {
        let mut result = Vec::<u32>::new();
        let mut carry: u64 = 0;
        
        let max_len = cmp::max(self.digits.len(), other.digits.len());
        
        for i in 0..max_len {
            let a = *self.digits.get(i).unwrap_or(&0) as u64;
            let b = *other.digits.get(i).unwrap_or(&0) as u64;
            
            let sum = a + b + carry;
            
            result.push((sum % BASE as u64) as u32);
            carry = sum / BASE as u64;
        }
        
        if carry > 0 {
            result.push(carry as u32);
        }
        
        let mut res = Self{ sign: 1, digits: result };
        res.normalize();
        res
    }
}