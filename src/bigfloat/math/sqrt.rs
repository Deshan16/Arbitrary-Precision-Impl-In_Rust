use std::cmp::Ordering;

use super::super::{BigFloat, BigInt, BigFloatError};

impl BigFloat {
    pub fn sqrt(&self) -> Result<BigFloat, BigFloatError> {
        if self.is_zero() {
            return Ok(BigFloat::zero(self.precision));
        }
        
        if self.is_negative() {
            return Err(BigFloatError::NegativeSquareRoot);
        }
        
        let out_prec = self.precision;
        let guard = BigFloat::guard_digits_for_precision(out_prec);
        let work_prec = out_prec + guard;
        
        let x = self.abs().with_precision(work_prec);
        
        let e = x.sci_exp10_abs();
        
        let e_even = if (e & 1) != 0 { e - 1 } else { e };
        
        let mut x_scaled = x.clone();
        x_scaled.exp10 -= e_even;
        x_scaled.normalize();
        
        let ten = BigFloat::from_with_prec(10u32, work_prec);
        let mut y = if x_scaled.comp(&ten) == Ordering::Less {
            BigFloat::from_with_prec(3u32, work_prec)
        } else {
            BigFloat::from_with_prec(8u32, work_prec)
        };
        
        let eps = BigFloat::new(1, BigInt::one(), -((out_prec as i64) + 2), work_prec);
        
        let max_iter = 2 * (out_prec.max(8)).ilog2() as usize + 16;
        
        for _ in 0..max_iter {
            let t = x_scaled.div(&y)?;
            
            let sum = BigFloat::add(&y, &t);
            let y_next = sum.div_u32(2)?;
            
            let diff = BigFloat::sub(&y_next, &y).abs();
            y = y_next;
            
            if diff.comp_abs(&eps) == Ordering::Less {
                break;
            }
        }
        
        y.exp10 += e_even;
        y.normalize();
        y.precision = out_prec;
        y.trim_to_prec();
        Ok(y)
    }
}


#[test]
fn sqrt_basic() {
    use crate::bigfloat::BigFloat;

    let x = BigFloat::from_str("2", 50).unwrap();
    let r = x.sqrt().unwrap();
    // r^2 should be close to 2
    let rr = &r * &r ;
    let diff = (&rr - &x).abs();
    println!("sqrt(2) = {}", r);
    println!("r*r = {}", rr);
    println!("diff = {}", diff);
}