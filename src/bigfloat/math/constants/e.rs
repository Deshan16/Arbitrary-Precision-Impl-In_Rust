use super::super::super::{BigFloat, BigInt};

impl BigFloat {
    #[allow(non_snake_case)]
    pub fn E(prec: usize) -> BigFloat {
        let prec_work = prec + 10;
        
        let mut term = BigFloat::one(prec_work);
        let mut sum = term.clone();
        let mut k: u32 = 1;
        
        loop {
            term = term.div_u32(k).expect("k is never zero");
            let next = &sum + &term;
            
            if next == sum {
                break;
            }
            
            if k > 20000 { break; } // Safety
            
            sum = next;
            k += 1;
        }
        
        sum.precision = prec + 1;
        sum.trim_to_prec();
        sum
    }
}