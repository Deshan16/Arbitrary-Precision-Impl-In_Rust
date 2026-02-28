use std::{collections::HashMap, sync::{Arc, OnceLock, RwLock}};

use super::super::super::{BigFloat, BigFloatError};

static PI_CACHE: OnceLock<RwLock<HashMap<usize, Arc<BigFloat>>>> = OnceLock::new();

impl BigFloat {
    #[allow(non_snake_case)]
    pub fn PI(prec: usize) -> Self {
        let cache = PI_CACHE.get_or_init(|| {
            RwLock::new(HashMap::new())
        });
        
        if let Some(pi) = cache.read().unwrap().get(&prec) {
            return (**pi).clone();
        }
        
        let pi = Self::compute_pi(prec);
        
        let mut w = cache.write().unwrap();
        
        if let Some(pi) = w.get(&prec) {
            return (**pi).clone();
        }
        
        w.insert(prec, Arc::new(pi.clone()));
        
        pi
    }
    
    pub fn compute_pi(prec: usize) -> Self {
        let guard = Self::one(prec).guard_digits_for_precision(prec);
        let work = prec + guard + 4;
        
        let a = Self::arctan_inv_u32(5, work).expect("internal div by non-zero");
        let b = Self::arctan_inv_u32(239, work).expect("internal div by non-zero");
        
        let p1 = &a * &Self::from_with_prec(16u32, work);
        let p2 = &b * &Self::from_with_prec(4u32, work);
        
        let mut pi = &p1 - &p2;
        pi.precision = prec;
        pi.trim_to_prec();
        pi
    }
    
    fn arctan_inv_u32(inv: u32, prec: usize) -> Result<Self, BigFloatError> {
        let x = Self::one(prec).div_u32(inv)?;
        let x2 = &x * &x;
        
        let mut term = x.clone();
        let mut sum = x;
        let mut k: u32 = 3;
        let mut add_next = false;
        
        loop {
            term = &term * &x2;
            let t = term.div_u32(k)?;
            
            let next = if add_next { &sum + &t } else { &sum - &t };
            
            if next == sum { break; }
            
            if k > 40000 { break; } // Safety 
            
            sum = next;
            add_next = !add_next;
            k += 2;
        }
        
        Ok(sum)
    }
}
