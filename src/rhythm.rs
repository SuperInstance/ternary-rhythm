use crate::Ternary;


/// A ternary time pattern: a sequence of ternary values over discrete ticks.
///
/// `Rhythm` wraps a `Vec<Ternary>` with playback position tracking for
/// real-time use. For pure pattern analysis and transformation, see the
/// free functions in the crate root.
#[derive(Clone, Debug)]
pub struct Rhythm {
    /// Pattern values.
    pub pattern: Vec<Ternary>,
    /// Current playback position.
    pub position: usize,
}

impl Rhythm {
    /// Create a new `Rhythm` from a pattern vector.
    #[inline]
    pub fn new(pattern: Vec<Ternary>) -> Self {
        Self { pattern, position: 0 }
    }

    /// Length of the pattern.
    #[inline]
    pub fn len(&self) -> usize {
        self.pattern.len()
    }

    /// Whether the pattern is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.pattern.is_empty()
    }

    /// Advance one tick and return the current value.
    pub fn tick(&mut self) -> Ternary {
        if self.pattern.is_empty() {
            return Ternary::Zero;
        }
        let val = self.pattern[self.position];
        self.position = (self.position + 1) % self.pattern.len();
        val
    }

    /// Peek at current value without advancing.
    pub fn current(&self) -> Ternary {
        if self.pattern.is_empty() {
            Ternary::Zero
        } else {
            self.pattern[self.position]
        }
    }

    /// Reset to beginning.
    pub fn reset(&mut self) {
        self.position = 0;
    }

    /// Density: fraction of non-zero elements.
    pub fn density(&self) -> f64 {
        if self.pattern.is_empty() {
            return 0.0;
        }
        let non_zero = self.pattern.iter().filter(|&&v| v != Ternary::Zero).count();
        non_zero as f64 / self.pattern.len() as f64
    }

    /// Balance: ratio of positive to negative values.
    /// Returns `(pos_count, neg_count)`.
    pub fn balance(&self) -> (usize, usize) {
        let pos = self.pattern.iter().filter(|&&v| v == Ternary::Pos).count();
        let neg = self.pattern.iter().filter(|&&v| v == Ternary::Neg).count();
        (pos, neg)
    }

    /// Reverse the pattern in-place.
    pub fn reverse(&mut self) {
        self.pattern.reverse();
    }

    /// Shift (rotate) the pattern by `n` positions in-place.
    pub fn rotate(&mut self, n: usize) {
        if self.pattern.is_empty() {
            return;
        }
        let n = n % self.pattern.len();
        let split = self.pattern.len() - n;
        let rotated: Vec<Ternary> = self.pattern[split..]
            .iter()
            .chain(self.pattern[..split].iter())
            .copied()
            .collect();
        self.pattern = rotated;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rhythm_tick() {
        let mut r = Rhythm::new(vec![Ternary::Pos, Ternary::Zero, Ternary::Neg]);
        assert_eq!(r.tick(), Ternary::Pos);
        assert_eq!(r.tick(), Ternary::Zero);
        assert_eq!(r.tick(), Ternary::Neg);
        assert_eq!(r.tick(), Ternary::Pos); // wraps
    }

    #[test]
    fn test_rhythm_density() {
        let r = Rhythm::new(vec![Ternary::Pos, Ternary::Zero, Ternary::Neg, Ternary::Zero]);
        assert!((r.density() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_rhythm_balance() {
        let r = Rhythm::new(vec![Ternary::Pos, Ternary::Pos, Ternary::Neg, Ternary::Zero]);
        assert_eq!(r.balance(), (2, 1));
    }

    #[test]
    fn test_rhythm_reverse() {
        let mut r = Rhythm::new(vec![Ternary::Pos, Ternary::Zero, Ternary::Neg]);
        r.reverse();
        assert_eq!(r.pattern, vec![Ternary::Neg, Ternary::Zero, Ternary::Pos]);
    }

    #[test]
    fn test_rhythm_rotate() {
        let mut r = Rhythm::new(vec![Ternary::Pos, Ternary::Zero, Ternary::Neg]);
        r.rotate(1);
        assert_eq!(r.pattern, vec![Ternary::Neg, Ternary::Pos, Ternary::Zero]);
    }

    #[test]
    fn test_rhythm_empty() {
        let mut r = Rhythm::new(vec![]);
        assert_eq!(r.tick(), Ternary::Zero);
        assert!(r.is_empty());
        assert_eq!(r.density(), 0.0);
    }

    #[test]
    fn test_rhythm_reset() {
        let mut r = Rhythm::new(vec![Ternary::Pos, Ternary::Neg]);
        r.tick();
        r.tick();
        r.reset();
        assert_eq!(r.position, 0);
    }
}
