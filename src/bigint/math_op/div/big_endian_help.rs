use super::super::super::{BASE, BigIntError};

pub(in crate::bigint::math_op::div) fn trim_leading_zeros(v: &mut Vec<u32>) {
    let first_nz = v.iter().position(|&x| x != 0).unwrap_or(v.len());
    if first_nz == v.len() {
        v.clear();
    } else if first_nz > 0 {
        v.drain(0..first_nz);
    }
}

pub(in crate::bigint::math_op::div) fn mul_u32_big_endian(a: &[u32], m: u32) -> Vec<u32> {
    if m == 0 || a.is_empty() { return Vec::new(); }

    let mut out = vec![0u32; a.len() + 1];
    let mut carry: u64 = 0;

    for i in (0..a.len()).rev() {
        let prod = a[i] as u64 * m as u64 + carry;
        out[i + 1] = (prod % BASE as u64) as u32;
        carry = prod / BASE as u64;
    }
    out[0] = carry as u32;

    trim_leading_zeros(&mut out);
    out
}

pub(in crate::bigint::math_op::div) fn div_mod_u32_big_endian(a: &[u32], d: u32) -> Result<(Vec<u32>, u32), BigIntError> {
    if d == 0 {
        return Err(BigIntError::ZeroDivisionError);
    }
    if a.is_empty() { return Ok((Vec::new(), 0)); }

    let mut q = vec![0u32; a.len()];
    let mut rem: u64 = 0;

    for (i, &x) in a.iter().enumerate() {
        let cur = rem * BASE as u64 + x as u64;
        q[i] = (cur / d as u64) as u32;
        rem = cur % d as u64;
    }

    trim_leading_zeros(&mut q);
    Ok((q, rem as u32))
}

pub(in crate::bigint::math_op::div) fn sub_mul_at(u: &mut [u32], j: usize, v: &[u32], qhat: u32) -> bool {
    let n = v.len();
    let mut borrow: i64 = 0;
    let mut carry: u64 = 0;

    for k in (0..n).rev() {
        let p = v[k] as u64 * qhat as u64 + carry;
        carry = p / BASE as u64;
        let p_lo = (p % BASE as u64) as i64;

        let idx = j + k + 1;
        let cur = u[idx] as i64 - p_lo - borrow;
        if cur < 0 {
            u[idx] = (cur + BASE as i64) as u32;
            borrow = 1;
        } else {
            u[idx] = cur as u32;
            borrow = 0;
        }
    }

    let cur0 = u[j] as i64 - carry as i64 - borrow;
    if cur0 < 0 {
        u[j] = (cur0 + BASE as i64) as u32;
        true
    } else {
        u[j] = cur0 as u32;
        false
    }
}

pub(in crate::bigint::math_op::div) fn add_back(u: &mut [u32], j: usize, v: &[u32]) {
    let n = v.len();
    let mut carry: u64 = 0;

    for k in (0..n).rev() {
        let idx = j + k + 1;
        let sum = u[idx] as u64 + v[k] as u64 + carry;
        u[idx] = (sum % BASE as u64) as u32;
        carry = sum / BASE as u64;
    }

    let sum0 = u[j] as u64 + carry;
    u[j] = (sum0 % BASE as u64) as u32;
    let carry0 = sum0 / BASE as u64;

    if carry0 != 0 {
        if j > 0 {
            u[j - 1] = (u[j - 1] as u64 + carry0) as u32; // carry0 is tiny (0/1)
        }
    }
}