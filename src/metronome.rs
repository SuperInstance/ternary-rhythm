use crate::Ternary;

/// A steady tick generator (metronome).
#[derive(Clone, Debug)]
pub struct Metronome {
    pub beats: usize,
    pub position: usize,
    pub accents: Vec<Ternary>,
}

impl Metronome {
    pub fn new(beats: usize) -> Self {
        let mut accents = vec![Ternary::Zero; beats];
        if beats > 0 {
            accents[0] = Ternary::Pos;
        }
        Self { beats, position: 0, accents }
    }

    pub fn set_accents(&mut self, accents: Vec<Ternary>) {
        self.accents = accents;
        self.accents.resize(self.beats, Ternary::Zero);
    }

    pub fn tick(&mut self) -> Ternary {
        if self.beats == 0 { return Ternary::Zero; }
        let val = if self.position < self.accents.len() { self.accents[self.position] } else { Ternary::Zero };
        self.position = (self.position + 1) % self.beats;
        val
    }

    pub fn is_downbeat(&self) -> bool { self.position == 0 }
    pub fn reset(&mut self) { self.position = 0; }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_metronome_tick() {
        let mut m = Metronome::new(4);
        assert_eq!(m.tick(), Ternary::Pos);
        assert_eq!(m.tick(), Ternary::Zero);
    }
    #[test] fn test_metronome_custom() {
        let mut m = Metronome::new(3);
        m.set_accents(vec![Ternary::Neg, Ternary::Pos, Ternary::Neg]);
        assert_eq!(m.tick(), Ternary::Neg);
    }
    #[test] fn test_metronome_reset() {
        let mut m = Metronome::new(4);
        m.tick(); m.tick(); m.reset();
        assert_eq!(m.position, 0);
    }
}
