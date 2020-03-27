mod prime_field;

pub trait FiniteField {
    type T: Copy;
    fn zero() -> Self::T;
    fn one() -> Self::T;
    fn add(&self, a: Self::T, b: Self::T) -> Self::T;
    fn sub(&self, a: Self::T, b: Self::T) -> Self::T;
    fn mul(&self, a: Self::T, b: Self::T) -> Self::T;
    fn inv(&self, rhs: Self::T) -> Self::T;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
