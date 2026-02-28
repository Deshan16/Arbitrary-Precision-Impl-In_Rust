use super::super::super::BigFloat;
use super::utils::{__cos, __sin};

impl BigFloat {
    pub fn tan(&self) -> BigFloat {
        if self.is_zero() {
            return BigFloat::zero(self.precision)
        }
        
        let p = self.precision;
        let mag = self.sci_exp10_abs().max(0) as usize;
        let prec_work = p + mag + 12;
        
        let (q, r) = self.rem_pi_over_2();
        
        let s = __sin(&r, p, prec_work);
        let c = __cos(&r, p, prec_work);
        
        let mut res = if (q & 1) == 0 {
            s.div(&c).expect("tan undefined: cos(r) == 0")
        } else {
            -(c.div(&s).expect("tan undefined: sin(r) == 0"))
        };
        
        res.precision = p;
        res.trim_to_prec();
        res
    }
}