use super::super::super::BigFloat;

impl BigFloat {
    pub fn to_radian(&self) -> Self {
        let prec = self.precision;
        let work = prec + self.guard_digits_for_precision(prec) + 8;

        let mut x = self.clone();
        x.precision = work;

        let pi = BigFloat::PI(work);

        let mut y = &x * &pi;
        y = y.div_u32(180).unwrap();

        y.precision = prec;
        y.trim_to_prec();
        y
    }
    
    pub fn to_degree(&self) -> Self {
        let prec = self.precision;
        let work = prec + self.guard_digits_for_precision(prec) + 8;

        let mut x = self.clone();
        x.precision = work;

        let pi = BigFloat::PI(prec);

        let mut y = x.mul_u32(180);
        y = &y / &pi;

        y.precision = prec;
        y.trim_to_prec();
        y
    }
}
