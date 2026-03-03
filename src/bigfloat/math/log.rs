use std::cmp::Ordering;

use super::super::{BigFloat, BigFloatError};

impl BigFloat {
    pub fn ln(&self) -> Result<BigFloat, BigFloatError> {
        if self.sign <= 0 { return Err(BigFloatError::NonPositiveLogError); }
        if self.is_one() { return Ok(BigFloat::zero(self.precision)); }
        
        let out = self.precision;
        let work = out + BigFloat::guard_digits_for_precision(out) + 8;
        
        let mut y = self.with_precision(work);
        let lo = BigFloat::from_str("0.9", work).unwrap();
        let hi = BigFloat::from_str("1.1", work).unwrap();
        let mut k: usize = 0;
        
        while y.comp(&lo) == Ordering::Less || y.comp(&hi) == Ordering::Greater {
            y = y.sqrt()?;
            k += 1;
            if k > 2048 { break; }
        }
        
        let one = BigFloat::one(work);
        let t = (&y - &one).div(&(&y + &one))?;
        let t2 = &t * &t;
        
        let mut sum = BigFloat::zero(work);
        let mut term = t.clone();
        let mut d: u64 = 1;
        
        loop {
            let add = term.div_u64(d)?;
            let next = &sum + &add;
            if next == sum { break; }
            
            sum = next;
            term = &term * &t2;
            d += 2;
            if d > 400001 { break; }
        }
        
        let mut out_ln = sum.mul_u32(2);
        for _ in 0..k { out_ln = out_ln.mul_u32(2); }
        
        out_ln.precision = out;
        out_ln.trim_to_prec();
        Ok(out_ln)
    }
    
    pub fn log10(&self) -> Result<BigFloat, BigFloatError> {
        let out = self.precision;
        let w = self.precision + BigFloat::guard_digits_for_precision(self.precision);
        
        let num = self.with_precision(w).ln()?;
        let den = BigFloat::from_with_prec(10u32, w).ln()?;
        
        let mut res = num.div(&den)?;
        res.precision = out;
        res.trim_to_prec();
        Ok(res)
    }
    
    pub fn log2(&self) -> Result<BigFloat, BigFloatError> {
        let out = self.precision;
        let w = self.precision + BigFloat::guard_digits_for_precision(self.precision);
        
        let num = self.with_precision(w).ln()?;
        let den = BigFloat::from_with_prec(2u32, w).ln()?;
        
        let mut res = num.div(&den)?;
        res.precision = out;
        res.trim_to_prec();
        Ok(res)
    }
    
    pub fn log(&self, other: Self) -> Result<BigFloat, BigFloatError> {
        let out = self.precision;
        let w = self.precision + BigFloat::guard_digits_for_precision(self.precision);
        
        let num = self.with_precision(w).ln()?;
        let den = other.with_precision(w).ln()?;
        
        let mut res = num.div(&den)?;
        res.precision = out;
        res.trim_to_prec();
        Ok(res)
    }
}

#[test]
fn test1() {
    let l = BigFloat::from_with_prec(10u32, 50).ln().unwrap();
    
    println!("{}", l)
}

#[test]
fn test2() {
    let lg2 = BigFloat::from_with_prec(10u32, 50).log10().unwrap();
    
    println!("{}", lg2)
}


#[test]
fn test3() {
    let l1 = BigFloat::from_with_prec(2, 50);
    let l2 = BigFloat::from_with_prec(10, 50);
    
    println!("{}", l1.log(l2).unwrap())
}