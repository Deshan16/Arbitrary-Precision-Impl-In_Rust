use std::f64::consts::{E, PI};

use test4::bigfloat::BigFloat;

fn main() {
    let pi = BigFloat::PI(50).div_u32(3).unwrap();
    let e = BigFloat::E(50);
    
    println!("{}", pi.sin());
    println!("{}", pi.cos());
    println!("{}", e);
}


#[test]
fn from_test1() {
    let e1 = BigFloat::E(16);
    let e2 = E;
    
    assert_eq!(e1.to_string(), e2.to_string());
}

#[test]
fn sin_cos_basic() {
    let p = 50;

    let x = BigFloat::PI(p).div_u32(6).unwrap(); // pi/6
    let s = x.sin();
    // let c = x.cos();

    println!("sin(pi/6) = {}", s.to_string()); // ~0.5
    // println!("cos(pi/6) = {}", c.to_string()); // ~0.8660...
}