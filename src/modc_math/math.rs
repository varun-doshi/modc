use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Rem, Sub};

use crate::utils::error::ModCError;
use primitive_types::U256;

#[derive(Debug, PartialEq)]
pub struct Field{
    modulus: U256,
}

impl Field
{
    pub fn new(modulus: U256) -> Result<Self, ModCError> {
        if modulus == U256::zero() {
            return Err(ModCError::ModulusZero);
        }
        let formatted_modulus = U256::from(modulus);
        Ok(Field {
            modulus: formatted_modulus,
        })
    }

    ///(a+b)=(a+b)mod p
    pub fn add<T: Copy+Into<U256>>(&self, a: T, b: T) -> Result<U256, ModCError> {
        let formatted_a: U256 = a.into();
        let formatted_b: U256 = b.into();

        let sum = formatted_a.checked_add(formatted_b);
        match sum {
            Some(s) => Ok(s % self.modulus),
            None => Err(ModCError::Overflow),
        }
    }

    ///(a % m + m - b % m ) % m
    pub fn sub<T: Copy+Into<U256>>(&self, a: T, b: T) -> Result<U256, ModCError> {
        //(6-9)mod7=-3mod7
        let formatted_a = self.self_mod(a);
        let formatted_b = self.self_mod(b);

        if formatted_b > formatted_a {
            let diff = (formatted_a + self.modulus - formatted_b) % self.modulus;
            Ok(diff)
        } else {
            let diff = formatted_a - formatted_b;
            Ok(diff)
        }
    }

    pub fn add_inv<T: Copy+Into<U256>>(&self, a: T) -> Result<U256, ModCError> {
        let formatted_a: U256 = a.into();
        if formatted_a >= self.modulus.into() {
            return Ok(((formatted_a - self.modulus) % self.modulus).into());
        }
        Ok(
            ((Into::<U256>::into(self.modulus) - Into::<U256>::into(formatted_a)) % self.modulus)
                .into(),
        )
    }

    /// a * b = (a*b)mod p
    pub fn mult<T: Copy+Into<U256>>(&self, a: T, b: T) -> Result<U256, ModCError> {
        let formatted_a: U256 = a.into();
        let formatted_b: U256 = b.into();

        let product = formatted_a.checked_mul(formatted_b);
        match product {
            Some(s) => return Ok(s % self.modulus),
            None => return Err(ModCError::Underflow),
        }
    }

    ///uses fermats little theorem:
    /// a^-1=[a^(p-a)]mod p
    pub fn mult_inv<T: Copy+Copy+Into<U256>>(&self, a: T) -> Result<U256, ModCError> {
        let formatted_a: U256 = a.into();
        if formatted_a == U256::from(0) {
            return Err(ModCError::InverseZero);
        } else if formatted_a > self.modulus.into() {
            let mod_a = self.self_mod(a);
            let powered = mod_a.pow(Into::<U256>::into(self.modulus) - mod_a);
            Ok(powered % self.modulus)
        } else {
            let powered = formatted_a.pow((self.modulus - a).into());
            Ok(powered % self.modulus)
        }
    }

    /// div(a,b) = a/b = [a*(b)^-1] mod p
    pub fn div<T: Copy+Into<U256>>(&self, a: T, b: T) -> Result<U256, ModCError> {
        let formatted_a: U256 = a.into();
        let formatted_b: U256 = b.into();

        let remainder = formatted_a.checked_div(formatted_b);
        match remainder {
            Some(s) => return Ok(s % self.modulus),
            None => return Err(ModCError::Underflow),
        }
    }

    /// self_mod(a) = a mod p
    pub fn self_mod<T: Copy+Into<U256>>(&self, a: T) -> U256 {
        a.into() % self.modulus
    }

    ///pow(a,b) = (a^b) mod p
    pub fn pow<T: Copy+Into<U256>>(&self, a: T, power: T) -> Result<U256, ModCError> {
        let formatted_a: U256 = a.into();
        let formatted_power: U256 = power.into();

        match formatted_a.checked_pow(formatted_power) {
            Some(r) => return Ok(r % self.modulus),
            None => return Err(ModCError::Overflow),
        }
    }
}

// impl Field {

// }
