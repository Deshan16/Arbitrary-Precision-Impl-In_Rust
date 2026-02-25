use std::cmp::Ordering;
use super::BigInt;

impl BigInt {
    pub fn comp(&self, other: &Self) -> Ordering {
        match (self.sign, other.sign) {
            (0, 0) => Ordering::Equal,
            (0, 1) => Ordering::Less,
            (0, -1) => Ordering::Greater,
            
            (1, 0) => Ordering::Greater,
            (-1, 0) => Ordering::Less,
            
            (1, -1) => Ordering::Greater,
            (-1, 1) => Ordering::Less,
            
            (1, 1) => self.comp_abs(other),
            (-1, -1) => other.comp_abs(self),
            _ => unreachable!()
        }
    }
    
    pub fn is_zero(&self) -> bool {
        self.sign == 0
    }
    
    pub fn is_negative(&self) -> bool {
        self.sign == -1
    }
    
    pub fn is_one(&self) -> bool {
        self.sign == 1 && self.digits.len() == 1 && self.digits[0] == 1
    }
}


impl BigInt {
    pub(crate) fn comp_abs(&self, other: &Self) -> Ordering {
        if self.digits.len() != other.digits.len() {
            return self.digits.len().cmp(&other.digits.len());
        }
        
        for (a, b) in self.digits.iter().rev().zip(other.digits.iter().rev()) {
            if a != b { return a.cmp(b); }
        }
        
        Ordering::Equal
    }
}