/// Ternary value: -1 (ghost/unaccented), 0 (silence), +1 (accented/downbeat).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Ternary {
    Neg = -1,
    Zero = 0,
    Pos = 1,
}

impl Ternary {
    /// Create a `Ternary` from an `i8`, returning `None` for invalid values.
    #[inline]
    pub fn from_i8(v: i8) -> Option<Self> {
        match v {
            -1 => Some(Ternary::Neg),
            0 => Some(Ternary::Zero),
            1 => Some(Ternary::Pos),
            _ => None,
        }
    }

    /// Convert this value to `i8`.
    #[inline]
    pub fn to_i8(self) -> i8 {
        self as i8
    }

    /// Simple pseudo-random `Ternary` value using a linear congruential generator.
    pub fn random(seed: &mut u64) -> Self {
        *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        match (*seed % 3) as i8 {
            0 => Ternary::Neg,
            1 => Ternary::Zero,
            _ => Ternary::Pos,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ternary_from_i8() {
        assert_eq!(Ternary::from_i8(-1), Some(Ternary::Neg));
        assert_eq!(Ternary::from_i8(0), Some(Ternary::Zero));
        assert_eq!(Ternary::from_i8(1), Some(Ternary::Pos));
        assert_eq!(Ternary::from_i8(2), None);
    }

    #[test]
    fn test_ternary_to_i8() {
        assert_eq!(Ternary::Neg.to_i8(), -1);
        assert_eq!(Ternary::Zero.to_i8(), 0);
        assert_eq!(Ternary::Pos.to_i8(), 1);
    }

    #[test]
    fn test_ternary_random_deterministic() {
        let mut seed = 42;
        let v1 = Ternary::random(&mut seed);
        let mut seed2 = 42;
        let v2 = Ternary::random(&mut seed2);
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_ternary_random_all_values() {
        let mut seed = 12345u64;
        let mut found = [false; 3];
        for _ in 0..100 {
            let v = Ternary::random(&mut seed);
            match v {
                Ternary::Neg => found[0] = true,
                Ternary::Zero => found[1] = true,
                Ternary::Pos => found[2] = true,
            }
            if found.iter().all(|&x| x) {
                break;
            }
        }
        assert!(found.iter().all(|&x| x), "random did not produce all three ternary values in 100 tries");
    }
}
