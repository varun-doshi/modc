use crate::utils::error::ModCError;
use primitive_types::U256;

#[derive(Debug, PartialEq)]
pub struct Field {
    modulus: U256,
}

impl Field {
    pub fn new(modulus: u64) -> Result<Field, ModCError> {
        if modulus == 0 {
            return Err(ModCError::ModulusZero);
        }
        let formatted_modulus = U256::from(modulus);
        Ok(Field {
            modulus: formatted_modulus,
        })
    }

    ///(a+b)=(a+b)mod p
    pub fn add(&self, a: u64, b: u64) -> Result<U256, ModCError> {
        let formatted_a = U256::from(a);
        let formatted_b = U256::from(b);

        let sum = formatted_a.checked_add(formatted_b);
        match sum {
            Some(s) => return Ok(s % self.modulus),
            None => return Err(ModCError::Overflow),
        }
    }

    ///(a % m + m - b % m ) % m
    pub fn sub(&self, a: u64, b: u64) -> Result<U256, ModCError> {
        //(6-9)mod7=-3mod7
        let formatted_a = self.self_mod(U256::from(a));
        let formatted_b = self.self_mod(U256::from(b));

        if formatted_b > formatted_a {
            let diff = (formatted_a + self.modulus - formatted_b) % self.modulus;
            Ok(diff)
        } else {
            let diff = formatted_a - formatted_b;
            Ok(diff)
        }
    }

    pub fn add_inv(&self, a: U256) -> Result<U256, ModCError> {
        Ok((self.modulus - a) % self.modulus)
    }

    /// a * b = (a*b)mod p
    pub fn mult(&self, a: u64, b: u64) -> Result<U256, ModCError> {
        let formatted_a = U256::from(a);
        let formatted_b = U256::from(b);

        let product = formatted_a.checked_mul(formatted_b);
        match product {
            Some(s) => return Ok(s % self.modulus),
            None => return Err(ModCError::Underflow),
        }
    }

    ///uses fermats little theorem:
    /// a^-1=[a^(p-a)]mod p
    pub fn mult_inv(&self, a: U256) -> Result<U256, ModCError> {
        if a == U256::from(0) {
            return Err(ModCError::InverseZero);
        } else if a > self.modulus {
            let mod_a = self.self_mod(a);
            let powered = mod_a.pow(self.modulus - mod_a);
            Ok(powered % self.modulus)
        } else {
            let powered = a.pow(self.modulus - a);
            Ok(powered % self.modulus)
        }
    }

    /// div(a,b) = a/b = [a*(b)^-1] mod p
    pub fn div(&self, a: u64, b: u64) -> Result<U256, ModCError> {
        let formatted_a = U256::from(a);
        let formatted_b = U256::from(b);

        let remainder = formatted_a.checked_div(formatted_b);
        match remainder {
            Some(s) => return Ok(s % self.modulus),
            None => return Err(ModCError::Underflow),
        }
    }

    /// self_mod(a) = a mod p
    pub fn self_mod(&self, a: U256) -> U256 {
        a % self.modulus
    }

    ///pow(a,b) = (a^b) mod p
    pub fn pow(&self, a: u64, power: u64) -> Result<U256, ModCError> {
        let formatted_a = U256::from(a);
        let formatted_power = U256::from(power);

        match formatted_a.checked_pow(formatted_power) {
            Some(r) => return Ok(r % self.modulus),
            None => return Err(ModCError::Overflow),
        }
    }
}
