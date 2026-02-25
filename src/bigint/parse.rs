use super::{BigInt, BASE, BASE_DIGITS, BigIntError};

impl From<u32> for BigInt {
    fn from(value: u32) -> Self {
        from_u32(value)
    }
}

impl From<i32> for BigInt {
    fn from(value: i32) -> Self {
        let sign = if value > 0 { 1 } else { -1 };
        
        let v = if sign == -1 { (-value) as u32 } else { value as u32 };
        
        let mut res = from_u32(v);
        res.sign = sign;
        res
    }
}

impl From<u64> for BigInt {
    fn from(value: u64) -> Self {
        from_u64(value)
    }
}

impl From<i64> for BigInt {
    fn from(value: i64) -> Self {
        let sign = if value > 0 { 1 } else { -1 };
        
        let v = if sign == -1 { (-value) as u64 } else { value as u64 };
        
        let mut res = from_u64(v);
        res.sign = sign;
        res
    }
}

impl std::str::FromStr for BigInt {
    type Err = BigIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        from_str(s)
    }
}

fn from_u32(value: u32) -> BigInt {
    if value == 0 { return BigInt::zero(); }
    
    let low = value % BASE;
    let high = value / BASE;
    
    let mut digits = Vec::new();
    digits.push(low);
    
    if high > 0 {
        digits.push(high);
    }
    
    let mut res = BigInt { sign: 1, digits };
    res.normalize();
    res
}

fn from_u64(value: u64) -> BigInt {
    if value == 0 { return BigInt::zero(); }
    
    let mut digits = Vec::<u32>::new();
    let mut v = value;
    
    while v > 0 {
        digits.push((v % BASE as u64) as u32);
        v /= BASE as u64;
    }
    
    let mut res = BigInt { sign: 1, digits };
    res.normalize();
    res
}

pub(in crate::bigint) fn from_str(value: &str) -> Result<BigInt, BigIntError> {
    let s = value.trim();
    
    if s == "0" { return Ok(BigInt::zero()); }
    
    let mut sign = 1;
    let mut start_index = 0;
    
    if s.starts_with('-') {
        sign = -1;
        start_index = 1;
    }
    
    let s = &s[start_index..];
    
    let mut digits = Vec::<u32>::new();
    let mut i = s.len();
    
    while i > 0 {
        let start = if i >= BASE_DIGITS { i - BASE_DIGITS } else { 0 };
        let chunk = &s[start..i];
        
        // digits.push(chunk.parse::<u32>().unwrap());
        digits.push(match chunk.parse::<u32>() {
            Ok(v) => v,
            Err(_) => { return Err(BigIntError::ParseError); },
        });
        
        if start == 0 {
            break;
        }
        
        i -= BASE_DIGITS;
    }
    
    let mut res = BigInt { sign, digits };
    res.normalize();
    Ok(res)
}