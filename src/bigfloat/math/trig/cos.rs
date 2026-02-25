use super::super::super::BigFloat;
use super::utils::{__sin, __cos};

impl BigFloat {
    pub fn cos(&self) -> BigFloat {
        if self.is_zero(){
            return BigFloat::zero(self.precision);
        }
        
        let p = self.precision;
        
        let mag = self.sci_exp10_abs().max(0) as usize;
        let prec_work = p + mag + 12;
        
        let (q, r) = self.rem_pi_over_2();
        
        let mut res = match q {
            0 => __cos(&r, p, prec_work),
            1 => -__sin(&r, p, prec_work),
            2 => -__cos(&r, p, prec_work),
            3 => __sin(&r, p, prec_work),
            _ => unreachable!()
        };
        
        res.precision = p;
        res.trim_to_prec();
        res
    }
}