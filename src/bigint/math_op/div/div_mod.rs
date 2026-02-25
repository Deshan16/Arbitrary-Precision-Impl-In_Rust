use std::cmp::Ordering;
use super::super::super::{BASE, BigInt, math_op::div::big_endian_help::{add_back, div_mod_u32_big_endian, mul_u32_big_endian, sub_mul_at, trim_leading_zeros}, BigIntError};

impl BigInt {
    pub fn div_mod(&self, other: &Self) -> Result<(Self, Self), BigIntError> {
        if other.sign == 0 {
            return Err(BigIntError::ZeroDivisionError);
        }
        
        if self.sign == 0 {
            return Ok((
                Self::zero(),
                Self::zero()
            ));
        }
        
        let (mut q, mut r) = self.div_mod_abs(other)?;
        
        q.sign = self.sign * other.sign;
        r.sign = self.sign;
        
        q.normalize();
        r.normalize();
        
        Ok((q, r))
    }
}

impl BigInt {
    fn div_mod_abs(&self, other: &Self) -> Result<(Self, Self), BigIntError> {
        if other.sign == 0 {
            return Err(BigIntError::ZeroDivisionError);
        }
        
        if self.sign == 0 {
            return Ok((
                Self::zero(),
                Self::zero()
            ));
        }
        
        // |self| < |other|
        if self.comp_abs(other) == Ordering::Less {
            return Ok((Self::zero(), self.abs()));
        }
        
        if other.digits.len() == 1 {
            let (q, r) = self.div_mod_u32(other.digits[0])?;
            let rem = if r == 0 {
                Self::zero()
            } else {
                Self { sign: 1, digits: vec![r] }
            };
            
            return Ok((q.abs(), rem));
        }
        
        let mut u_be: Vec<u32> = self.digits.iter().rev().copied().collect();
        let mut v_be: Vec<u32> = other.digits.iter().rev().copied().collect();
        
        trim_leading_zeros(&mut u_be);
        trim_leading_zeros(&mut v_be);
        
        let n = v_be.len();
        let m = u_be.len() - n;
        
        let v0 = v_be[0] as u64;
        let d = (BASE as u64 / (v0 + 1)) as u32;
        
        let v_norm = if d == 1 { v_be.clone() } else { mul_u32_big_endian(&v_be, d) };
        let mut u_norm = if d == 1 { u_be.clone() } else { mul_u32_big_endian(&u_be, d) };
        
        u_norm.insert(0, 0);
        
        let v = v_norm;
        let mut u = u_norm;
        
        let mut q = vec![0u32; m + 1];
        
        for j in 0..=m {
            let uj = u[j] as u64;
            let uj1 = u[j + 1] as u64;
            let uj2 = u[j + 2] as u64;

            let v0 = v[0] as u64;
            let v1 = v[1] as u64;

            let mut qhat = ((uj * BASE as u64 + uj1) / v0) as u64;
            let mut rhat = ((uj * BASE as u64 + uj1) % v0) as u64;

            if qhat >= BASE as u64 {
                qhat = (BASE - 1) as u64;
            }
            
            while qhat * v1 > rhat * BASE as u64 + uj2 {
                qhat -= 1;
                rhat += v0;
                if rhat >= BASE as u64 {
                    break;
                }
            }
            
            let borrow = sub_mul_at(&mut u, j, &v, qhat as u32);
            
            if borrow {
                qhat -= 1;
                add_back(&mut u, j, &v);
            }
            
            q[j] = qhat as u32;
        }
        
        let rem_norm = u[(m + 1)..(m + 1 + n)].to_vec();
        let (mut rem_be, _r) = if d == 1 {
            (rem_norm, 0)
        } else {
            div_mod_u32_big_endian(&rem_norm, d)?
        };
        
        trim_leading_zeros(&mut rem_be);
        trim_leading_zeros(&mut q);
        
        let q_le: Vec<u32> = q.iter().rev().copied().collect();
        let r_le: Vec<u32> = rem_be.iter().rev().copied().collect();
        
        let mut quotient = if q_le.is_empty() { Self::zero() } else { Self { sign: 1, digits: q_le } };
        let mut reminder = if r_le.is_empty() { Self::zero() } else { Self { sign: 1, digits: r_le } };
        
        quotient.normalize();
        reminder.normalize();
        
        Ok((quotient, reminder))
    }
    
    pub(crate) fn div_mod_u32(&self, d: u32) -> Result<(Self, u32), BigIntError> {
        if d == 0 {
            return  Err(BigIntError::ZeroDivisionError);
        }
        
        if self.sign == 0 {
            return Ok((Self::zero(), 0));
        }
        
        let n = self.digits.len();
        let mut q_digits = vec![0u32; self.digits.len()];
        let mut rem: u64 = 0;
        
        let base = BASE as u64;
        let d_work = d as u64;
        
        for i in (0..n).rev() {
            let cur = rem * base + self.digits[i] as u64;
            q_digits[i] = (cur / d_work) as u32;
            rem = cur % d_work;
        }
        
        let mut q = Self { sign: self.sign, digits: q_digits };
        q.normalize();
        Ok((q, rem as u32))
    }
    
    pub(crate) fn div_mod_u64(&self, d: u64) -> Result<(BigInt, u64), BigIntError> {
        if d == 0 {
            return Err(BigIntError::ZeroDivisionError);
        }
        
        if self.sign == 0 {
            return Ok((Self::zero(), 0));
        }
        
        if d <= u32::MAX as u64 {
            let (q, r) = self.div_mod_u32(d as u32)?;
            return Ok((q, r as u64));
        }
        
        let n = self.digits.len();
        let mut q_digits = vec![0u32; n];
        let mut rem: u128 = 0;
        
        let base = BASE as u128;
        let d_work = d as u128;
        
        for i in (0..n).rev() {
            let cur = rem * base + self.digits[i] as u128;
            q_digits[i] = (cur / d_work) as u32;
            rem = cur % d_work;
        }
        
        let mut q = Self { sign: self.sign, digits: q_digits };
        q.normalize();
        Ok((q, rem as u64))
    }
    
    pub(crate) fn rem_u32(&self, m: u32) -> u32 {
        assert!(m != 0);
        
        if self.sign == 0 { return 0; }
        
        let mut rem: u64 = 0;
        
        let base = BASE as u64;
        let m_work = m as u64;
        
        for &d in self.digits.iter().rev() {
            rem = (rem * base + d as u64) % m_work;
        }
        
        rem as u32
    }
}