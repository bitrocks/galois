use super::FiniteField;

pub struct PrimeField {
    prime: i128,
}

impl FiniteField for PrimeField {
    type T = i128;

    fn zero() -> i128 {
        0 as i128
    }

    fn one() -> i128 {
        1 as i128
    }
    fn add(&self, a: i128, b: i128) -> i128 {
        (a + b) % self.prime
    }
    fn sub(&self, a: Self::T, b: Self::T) -> Self::T {
        let c = (a - b) % self.prime;
        if c < (0 as i128) {
            c + self.prime
        } else {
            c
        }
    }
    fn mul(&self, a: Self::T, b: Self::T) -> Self::T {
        (a * b) % self.prime
    }
    fn inv(&self, rhs: Self::T) -> Self::T {
        let (_gcd, _, inv) = extend_euclidean_algorithm(self.prime, rhs);
        inv
    }
}

fn extend_euclidean_algorithm(a: i128, b: i128) -> (i128, i128, i128) {
    let (mut r, mut next_r, mut s, mut next_s, mut t, mut next_t) = (a, b, 1, 0, 0, 1);
    let mut quotient;
    let mut tmp;
    while next_r > 0 {
        quotient = r / next_r;
        tmp = next_r;
        next_r = r - next_r * quotient;
        r = tmp;
        tmp = next_s;
        next_s = s - next_s * quotient;
        s = tmp;

        tmp = next_t;
        next_t = t - next_t * quotient;
        t = tmp;
    }
    (r, s, t)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_extend_euclidean_algorithm() {
        assert_eq!(extend_euclidean_algorithm(32, 24), (8, 1, -1));
        assert_eq!(extend_euclidean_algorithm(2024, 748), (44, -7, 19));
    }
}
