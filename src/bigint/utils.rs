use super::{BigInt, BASE_DIGITS};

impl BigInt {
    pub fn abs(&self) -> Self {
        if self.sign == 0 {
            Self::zero()
        } else {
            let mut res = self.clone();
            res.sign = 1;
            res
        }
    }
}

impl BigInt {
    pub(in crate::bigint) fn normalize(&mut self) {
        while let Some(&last) = self.digits.last() {
            if last == 0 { self.digits.pop(); }
            else { break; }
        }
        
        if self.digits.is_empty() { self.sign = 0; }
    }
    
    pub(crate) fn neg(&self) -> Self {
        if self.sign == 0 { Self::zero() }
        else {
            let mut res = self.clone();
            res.sign *= -1;
            res
        }
    }
    
    pub(crate) fn decimal_len(&self) -> usize {
        if self.sign == 0 { return 1; }
        let last = *self.digits.last().unwrap();
        let last_len = if last >= 100_000_000 { 9 }
        else if last >= 10_000_000 { 8 }
        else if last >= 1_000_000 { 7 }
        else if last >= 100_000 { 6 }
        else if last >= 10_000 { 5 }
        else if last >= 1_000 { 4 }
        else if last >= 100 { 3 }
        else if last >= 10 { 2 }
        else { 1 };
        
        (self.digits.len() - 1) * BASE_DIGITS + last_len
    }
}