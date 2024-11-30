#[cfg(test)]
extern crate modc;
use crate::modc::modc_math::math::Field;
use primitive_types::U256;

// #[test]
// fn test_field_creation_with_zero_order() {
//     let value=U256::from(0u64);
//     let result = Field::new(value);
//     assert_eq!(result, Err(ModCError::ModulusZero));
// }

#[test]
#[allow(non_snake_case)]
fn test_modc_addition() {
    let modu = U256::from(7u64);
    let F = Field::new(modu).unwrap();
    let addition = F.add(U256::from(3), U256::from(6)).unwrap();
    assert_eq!(addition, U256::from(2));
}

#[test]
#[allow(non_snake_case)]
fn test_modc_subtraction_1() {
    let F = Field::new(U256::from(7)).unwrap();
    let difference = F.sub(9, 6);
    assert_eq!(difference.unwrap(), U256::from(3));
}

#[test]
#[allow(non_snake_case)]
fn test_modc_subtraction_2() {
    let F = Field::new(U256::from(7)).unwrap();
    let difference = F.sub(6, 9);
    assert_eq!(difference.unwrap(), U256::from(4));
}
#[test]
#[allow(non_snake_case)]
fn test_modc_addv_inv() {
    let F = Field::new(U256::from(7)).unwrap();
    let addv_inv = F.add_inv(U256::from(6));
    assert_eq!(addv_inv.unwrap(), U256::from(1));
}
#[test]
#[allow(non_snake_case)]
fn test_modc_mult_inv() {
    let F = Field::new(U256::from(7)).unwrap();
    let mult_inv = F.mult_inv(U256::from(2));
    assert_eq!(mult_inv.unwrap(), U256::from(4));
}
#[test]
#[allow(non_snake_case)]
fn test_modc_div() {
    let F = Field::new(U256::from(7)).unwrap();
    let remainder = F.div(4, 2);
    assert_eq!(remainder.unwrap(), U256::from(2));
}
#[test]
#[allow(non_snake_case)]
fn test_modc_pow() {
    let F = Field::new(U256::from(7)).unwrap();
    let remainder = F.pow(8, 2);
    assert_eq!(remainder.unwrap(), U256::from(1));
}
