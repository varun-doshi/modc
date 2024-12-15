#[cfg(test)]
extern crate modc;
use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

use crate::modc::modc_math::math::Field;
use primitive_types::U256;

#[test]
#[allow(non_snake_case)]
fn test_mersenne_addition() {
    let mersenne_mod = get_mersenne_mod(7_u16);
    let F = Field::new(mersenne_mod).unwrap();
    let addition = F.add(U256::from(91), U256::from(108)).unwrap();
    assert_eq!(addition, U256::from(72));
}

fn get_mersenne_mod<
    T: Copy + Add + Sub + Mul + Div + Debug + PartialOrd + PartialEq + Into<usize>,
>(
    n: T,
) -> U256 {
    let p;
    if !is_prime(n.into()) {
        println!("n is not prime. Mersenne is not possible!");
        std::process::exit(1);
    } else {
        p = ((2_usize).pow(Into::<usize>::into(n).try_into().unwrap())) - 1;
    }
    U256::from(p)
}

fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    } else if n <= 3 {
        return true;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    } else {
        let limit = (n as f64).sqrt() as usize + 1;
        for i in (5..limit).step_by(6) {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
        }
        return true;
    }
}
