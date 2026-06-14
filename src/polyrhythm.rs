use crate::{Rhythm, Ternary};


/// Multiple simultaneous rhythm patterns (polyrhythm).
#[derive(Clone, Debug)]
pub struct Polyrhythm {
    /// Individual rhythms.
    pub rhythms: Vec<Rhythm>,
}

impl Polyrhythm {
    /// Create a new polyrhythm from a collection of individual rhythms.
    pub fn new(rhythms: Vec<Rhythm>) -> Self {
        Self { rhythms }
    }

    /// Tick all rhythms and return their combined output.
    pub fn tick(&mut self) -> Vec<Ternary> {
        self.rhythms.iter_mut().map(|r| r.tick()).collect()
    }

    /// Least common multiple of all rhythm lengths (full cycle length).
    pub fn cycle_length(&self) -> usize {
        if self.rhythms.is_empty() {
            return 0;
        }
        let mut lcm = self.rhythms[0].len();
        for r in &self.rhythms[1..] {
            lcm = crate::lcm_value(lcm, r.len());
        }
        lcm
    }

    /// Number of rhythms (voices).
    pub fn voice_count(&self) -> usize {
        self.rhythms.len()
    }

    /// Reset all rhythms.
    pub fn reset(&mut self) {
        for r in &mut self.rhythms {
            r.reset();
        }
    }
}
