use std::cmp::Ordering;

use super::super::{BigFloat, BigFloatError};

impl BigFloat {
    pub fn exp(&self) -> Result<BigFloat, BigFloatError> {
        if self.is_zero() { return Ok(BigFloat::one(self.precision)); }
        
        let out = self.precision;
        let work = out + BigFloat::guard_digits_for_precision(out) + 8;
        
        let mut z = self.abs().with_precision(work);
        let half = BigFloat::one(work).div_u32(2)?;
        let mut k: usize = 0;
        
        while z.comp_abs(&half) == Ordering::Greater {
            z = z.div_u32(2)?;
            k += 1;
        }
        
        let mut term = BigFloat::one(work);
        let mut sum = term.clone();
        let mut n: u64 = 1;
        
        loop {
            term = (&term * &z).div_u64(n)?;
            let next = &sum + &term;
            if next == term { break; }
            sum = next;
            n += 1;
            if n > 20000 { break; }
        }
        
        for _ in 0..k {
            sum = &sum * &sum;
            sum.trim_work();
        }
        
        if self.is_negative() {
            sum = BigFloat::one(work).div(&sum)?;
        }
        
        sum.precision = out;
        sum.trim_to_prec();
        Ok(sum)
    }
}


#[test]
fn test1() {
    let e = BigFloat::from_with_prec(25, 50).exp().unwrap();
    
    print!("{}", e)
}