use std::f64::consts::{E, PI};

use test4::bigfloat::BigFloat;


fn main() {
    let prec = 50;
    let pi = BigFloat::PI(prec);
    
    let pi_3 = pi.div_u32(3).unwrap();
    let pi_6 = pi.div_u32(6).unwrap();
    
    println!("pi/3 = {}", pi_3);
    println!("to_deg = {}", pi_3.to_degree());
    
    println!("pi/6 = {}", pi_6);
    println!("to_deg = {}", pi_6.to_degree());
    
    let d30 = BigFloat::from_with_prec(30, prec);
    let d60 = BigFloat::from_with_prec(60, prec);
    
    println!("30 -> {}", d30.to_radian());
    println!("60 -> {}", d60.to_radian());
    
    println!();
    println!();
    
    println!("Pi before = {}", BigFloat::PI(prec));

    println!("deg(pi/3) = {}", BigFloat::PI(prec).div_u32(3).unwrap().to_degree());
    println!("deg(pi/6) = {}", BigFloat::PI(prec).div_u32(6).unwrap().to_degree());

    println!("Pi after  = {}", BigFloat::PI(prec));
}

// fn main() {
//     let p = 50;

//     let pi = BigFloat::PI(p);
//     println!("PI = {}", pi);

//     let x = pi.div_u32(3).unwrap();
//     println!("PI/3 = {}", x);

//     let y = pi.div_u32(6).unwrap();
//     println!("PI/6 = {}", y);

//     println!("sin(PI/3) = {}", x.sin());
//     println!("cos(PI/3) = {}", x.cos());
// }

// fn main() {
//     let prec = 50;
//     let one = BigFloat::one(prec);
//     let pi = BigFloat::PI(prec);
    
//     println!("1 = {}", one);
//     println!("1/2 = {}", one.div_u32(2).unwrap());
//     println!("1/3 = {}", one.div_u32(3).unwrap());
//     println!("1/4 = {}", one.div_u32(4).unwrap());
//     println!("1/6 = {}", one.div_u32(6).unwrap());
    
//     println!("pi = {}", pi);
//     println!("pi/2 = {}", pi.div_u32(2).unwrap());
//     println!("pi/3 = {}", pi.div_u32(3).unwrap());
//     println!("pi/4 = {}", pi.div_u32(4).unwrap());
//     println!("pi/6 = {}", pi.div_u32(6).unwrap());
// }



// #[test]
// fn from_test1() {
//     let e1 = BigFloat::E(16);
//     let e2 = E;
    
//     assert_eq!(e1.to_string(), e2.to_string());
// }

// #[test]
// fn sin_cos_basic() {
//     let p = 50;

//     let x = BigFloat::PI(p).div_u32(6).unwrap(); // pi/6
//     let s = x.sin();
//     // let c = x.cos();

//     println!("sin(pi/6) = {}", s.to_string()); // ~0.5
//     // println!("cos(pi/6) = {}", c.to_string()); // ~0.8660...
// }