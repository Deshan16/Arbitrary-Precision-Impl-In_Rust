use super::super::{BigInt, BASE, BASE_DIGITS};

impl BigInt {
    pub fn mul(&self, other: &Self) -> Self {
        if self.sign == 0 || other.sign == 0 {
            return Self::zero();
        }
        
        let mut r = self.mul_abs(other);
        r.sign = self.sign * other.sign;
        r
    }
}

impl BigInt {
    pub(in crate::bigint) fn mul_abs(&self, other: &Self) -> Self {
        if self.sign == 0 || other.sign == 0 {
            return Self::zero();
        }
        
        let mut acc = vec![0u128; self.digits.len() + other.digits.len() + 1];
        
        for i1 in 0..self.digits.len() {
            let a = self.digits[i1] as u128;
            for i2 in 0..other.digits.len() {
                let b = other.digits[i2] as u128;
                acc[i1 + i2] += a * b;
            }
        }
        
        let mut carry: u128 = 0;
        
        for i in 0..acc.len() {
            let total = acc[i] + carry;
            acc[i] = total % BASE as u128;
            carry = total / BASE as u128;
        }
        
        let digits: Vec<u32> = acc.into_iter().map(|v| v as u32).collect();
        let mut res = Self { sign: 1, digits };
        res.normalize();
        res
    }
    
    pub(crate) fn mul_u32(&self, m: u32) -> Self {
        if self.sign == 0 || m == 0 {
            return Self { sign: 0, digits: Vec::new() };
        }
        
        let mut result = Vec::<u32>::with_capacity(self.digits.len() + 2);
        let mut carry: u64 = 0;
        
        let base = BASE as u64;
        let m_work = m as u64;
        
        for &digit in &self.digits {
            let temp = (digit as u64) * m_work + carry;
            result.push((temp % base) as u32);
            carry = temp / base;
        }
        
        if carry > 0 {
            result.push(carry as u32);
        }
        
        let mut r = Self{ sign: self.sign, digits: result };
        r.normalize();
        r
    }
    
    pub(crate) fn mul_u64(&self, m: u64) -> Self {
        if self.sign == 0 || m == 0 {
            return Self::zero();
        }
        
        let mut result = Vec::<u32>::with_capacity(self.digits.len() + 3);
        let mut carry: u128 = 0;
        
        let base = BASE as u128;
        let m_work = m as u128;
        
        for &digit in &self.digits {
            let temp = (digit as u128) * m_work + carry;
            result.push((temp % base) as u32);
            carry = temp / base;
        }
        
        while carry > 0 {
            result.push((carry % base) as u32);
            carry /= base;
        }
        
        let mut res = Self { sign: self.sign, digits: result };
        res.normalize();
        res
    }
    
    pub(crate) fn mul_pow10(&self, k: usize) -> Self {
        if self.is_zero() { return Self::zero(); }
        
        let whole_chunks = k / BASE_DIGITS;
        let rem_digits = k % BASE_DIGITS;
        
        let mut r = self.abs();
        
        if whole_chunks > 0 {
            let mut new_digits = vec![0u32; whole_chunks];
            new_digits.extend_from_slice(&r.digits);
            r.digits = new_digits;
            r.normalize();
        }
        
        if rem_digits > 0 {
            let mut m: u32 = 1;
            for _ in 0..rem_digits { m *= 10; }
            r = r.mul_u32(m);
        }
        
        r.sign = self.sign;
        r.normalize();
        r
    }
}