#[derive(Debug, PartialEq)]
pub enum ModCError {
    ModulusZero,
    InverseZero,
    Overflow,
    Underflow,
    NoAdditiveInverse,
}
