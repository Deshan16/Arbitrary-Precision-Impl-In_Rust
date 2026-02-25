use super::super::super::BigFloat;

impl BigFloat {
    pub(in crate::bigfloat)  fn rem_pi_over_2(&self) -> (u32, BigFloat) {
        let p = self.precision;
        
        let mag = self.sci_exp10_abs().max(0) as usize;
        
        let prec_work = p + mag + 12;
        
        let x = self.with_precision(prec_work);
        
        let invers_v = invers_pi_over_2(prec_work);
        let v = pi_over_2(prec_work);
        
        let y = &x * &invers_v;
        let k = y.round_to_bigint();
        
        let mut q = k.rem_u32(4);
        q &= 3;
        
        let kbf = BigFloat::from_with_prec(k, prec_work);
        let mut r = &x - &(&kbf * &v);
        
        r.trim_to_prec();
        
        (q, r)
    }
}

pub(in crate::bigfloat) fn __sin(r: &BigFloat, out_prec: usize, work_prec: usize) -> BigFloat {
    let extra: i64 = 8;
    let stop_exp: i64 = -((out_prec as i64) + extra);
    
    let r = r.with_precision(work_prec);
    let r2 = &r * &r;
    
    let mut sin_sum = BigFloat::zero(work_prec);
    let mut t_sin = r.clone();
    let mut n: u32 = 0;
    
    loop {
        let next = &sin_sum + &t_sin;
        
        let a: u64 = (2 * n + 2) as u64;
        let b: u64 = (2 * n + 3) as u64;
        let denom = a * b;
        
        t_sin = -(&t_sin * &r2);
        t_sin = t_sin.div_u64(denom).expect("denom is never zero");
        
        if next == sin_sum || t_sin.sci_exp10_abs() < stop_exp {
            break;
        }
        
        n += 1;
        
        sin_sum = next;
        
        if n > 20000 { break; }
    }
    
    let mut s = sin_sum;
    s.trim_to_prec();
    s
}

pub(in crate::bigfloat) fn __cos(r: &BigFloat, out_prec: usize, work_prec: usize) -> BigFloat {
    let extra: i64 = 8;
    let stop_exp: i64 = -((out_prec as i64) + extra);
    
    let r = r.with_precision(work_prec);
    let r2 = &r * &r;
    
    let mut cos_sum = BigFloat::zero(work_prec);
    let mut t_cos = BigFloat::one(work_prec);
    let mut n: u32 = 0;
    
    loop {
        let next = &cos_sum + &t_cos;
        
        if next == cos_sum { break; }
        
        cos_sum = next;
        
        let a: u64 = (2 * n + 1) as u64;
        let b: u64 = (2 * n + 2) as u64;
        let denom = a * b;
        
        t_cos = -(&t_cos * &r2);
        t_cos = t_cos.div_u64(denom).expect("denom is never zero");
        
        if t_cos.sci_exp10_abs() < stop_exp { break; }
        
        n += 1;
        if n > 20000 { break; }
    }
    
    let mut c = cos_sum;
    c.trim_to_prec();
    c
}

pub(in crate::bigfloat) fn pi_over_2(p: usize) -> BigFloat {
    BigFloat::PI(p + 5).div_u32(2).expect("2 != 0")
}

pub(in crate::bigfloat) fn invers_pi_over_2(p: usize) -> BigFloat {
    &BigFloat::from_with_prec(2, p + 5) / &BigFloat::PI(p + 5)
}