use crate::Ternary;

/// Off-beat emphasis detection and creation.
#[derive(Clone, Debug)]
pub struct Syncopation;

impl Syncopation {
    /// Measure syncopation: count weak-position events.
    pub fn measure(pattern: &[Ternary], strong_positions: &[usize]) -> f64 {
        if pattern.is_empty() { return 0.0; }
        let mut syncopated = 0usize;
        for i in 0..pattern.len() {
            let is_strong = strong_positions.contains(&i);
            if !is_strong && pattern[i] != Ternary::Zero {
                syncopated += 1;
            }
        }
        syncopated as f64 / pattern.len() as f64
    }

    /// Create a syncopated version by shifting onsets.
    pub fn syncopate(pattern: &[Ternary], shift: usize) -> Vec<Ternary> {
        let mut result = vec![Ternary::Zero; pattern.len()];
        for i in 0..pattern.len() {
            if pattern[i] != Ternary::Zero {
                result[(i + shift) % pattern.len()] = pattern[i];
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_sync_measure() {
        let p = vec![Ternary::Pos, Ternary::Zero, Ternary::Pos, Ternary::Zero];
        assert_eq!(Syncopation::measure(&p, &[0, 2]), 0.0);
        let p2 = vec![Ternary::Zero, Ternary::Pos, Ternary::Zero, Ternary::Pos];
        assert!(Syncopation::measure(&p2, &[0, 2]) > 0.0);
    }
    #[test] fn test_syncopate() {
        let p = vec![Ternary::Pos, Ternary::Zero, Ternary::Pos, Ternary::Zero];
        let s = Syncopation::syncopate(&p, 1);
        assert_eq!(s, vec![Ternary::Zero, Ternary::Pos, Ternary::Zero, Ternary::Pos]);
    }
}
