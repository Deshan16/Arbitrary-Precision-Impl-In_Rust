use super::{BigFloat, BigInt, BigFloatError};

impl BigFloat {
    pub fn from_with_prec<T: IntoBigFloatWithPrec>(value: T, prec: usize) -> Self {
        value.into_bigfloat_with_prec(prec)
    }
    
    pub fn from_str(s: &str, prec: usize) -> Result<Self, BigFloatError> {
        let mut temp = s.trim();
        
        if temp.is_empty() { return Err(BigFloatError::ParseError); }
        
        let mut sign: i8 = 1;
        if temp.starts_with('-') { sign = -1; temp = &temp[1..]; }
        else if temp.starts_with('+') { temp = &temp[1..]; }
        
        if temp.is_empty() { return Err(BigFloatError::ParseError); }
        
        let (base_part, exp_part) = match temp.find(|c| c == 'e' || c == 'E') {
            Some(i) => (&temp[..i], Some(&temp[i + 1..])),
            None => (temp, None),
        };
        
        let mut exp10: i64 = 0;
        if let Some(ep) = exp_part {
            if ep.trim().is_empty() { return Err(BigFloatError::ParseError); }
            exp10 = ep.trim().parse::<i64>().map_err(|_| BigFloatError::ParseError)?;
        }
        
        let mut int_part = base_part;
        let mut frac_part = "";
        if let Some(dot) = base_part.find('.') {
            int_part = &base_part[..dot];
            frac_part = &base_part[dot + 1..];
        }
        
        
        if int_part.is_empty() { int_part = "0"; }
        
        if !int_part.chars().all(|c| c.is_ascii_digit()) { return Err(BigFloatError::ParseError); }
        if !frac_part.chars().all(|c| c.is_ascii_digit()) { return Err(BigFloatError::ParseError); }
        
        let int_trimed = int_part.trim_start_matches('0');
        let int_trimed = if int_trimed.is_empty() { "0" } else { int_trimed };
        
        let all_digits = format!("{}{}", int_trimed, frac_part);
        if all_digits.chars().all(|c| c == '0') {
            return Ok(BigFloat { sign: 0, mantissa: BigInt::zero(), exp10: 0, precision: prec });
        }
        
        exp10 -= frac_part.len() as i64;
        
        let mant = BigInt::from_str(&all_digits)?;
        Ok(BigFloat::new(sign, mant, exp10, prec))
    }
}

pub trait IntoBigFloatWithPrec {
    fn into_bigfloat_with_prec(self, prec: usize) -> BigFloat;
}

impl IntoBigFloatWithPrec for u32 {
    fn into_bigfloat_with_prec(self, prec: usize) -> BigFloat {
        if self == 0 { return BigFloat::zero(prec); }
        BigFloat::new(1, BigInt::from(self), 0, prec)
    }
}

impl IntoBigFloatWithPrec for i32 {
    fn into_bigfloat_with_prec(self, prec: usize) -> BigFloat {
        if self == 0 { return BigFloat::zero(prec); }
        let sign = if self > 0 { 1 } else { -1 };
        BigFloat::new(sign, BigInt::from(self), 0, prec)
    }
}

impl IntoBigFloatWithPrec for u64 {
    fn into_bigfloat_with_prec(self, prec: usize) -> BigFloat {
        if self == 0 { return BigFloat::zero(prec); }
        BigFloat::new(1, BigInt::from(self), 0, prec)
    }
}

impl IntoBigFloatWithPrec for i64 {
    fn into_bigfloat_with_prec(self, prec: usize) -> BigFloat {
        if self == 0 { return BigFloat::zero(prec); }
        let sign = if self > 0 { 1 } else { -1 };
        BigFloat::new(sign, BigInt::from(self), 0, prec)
    }
}

impl IntoBigFloatWithPrec for BigInt {
    fn into_bigfloat_with_prec(self, prec: usize) -> BigFloat {
        if self.is_zero() { return BigFloat::zero(prec); }
        let sign = if self.is_negative() { -1 } else { 1 };
        BigFloat::new(sign, self.clone(), 0, prec)
    }
}

impl IntoBigFloatWithPrec for f32 {
    fn into_bigfloat_with_prec(self, prec: usize) -> BigFloat {
        BigFloat::from_str(&self.to_string(), prec).expect("value is not finite or nan")
    }
}

impl IntoBigFloatWithPrec for f64 {
    fn into_bigfloat_with_prec(self, prec: usize) -> BigFloat {
        BigFloat::from_str(&self.to_string(), prec).expect("value is not finite or nan")
    }
}