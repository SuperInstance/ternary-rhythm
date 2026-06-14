use crate::{Syncopation, Ternary};

/// Groove/feel detection from a rhythm pattern.
#[derive(Clone, Debug)]
pub struct Groove {
    pub pattern: Vec<Ternary>,
}

impl Groove {
    pub fn new(pattern: Vec<Ternary>) -> Self { Self { pattern } }

    /// Swing ratio: long interval / short interval (1.0 = even).
    pub fn swing_ratio(&self) -> f64 {
        let mut intervals = Vec::new();
        let mut last: Option<usize> = None;
        for (i, &v) in self.pattern.iter().enumerate() {
            if v != Ternary::Zero {
                if let Some(l) = last { intervals.push(i - l); }
                last = Some(i);
            }
        }
        if intervals.len() < 2 { return 1.0; }
        let longs: f64 = intervals.iter().enumerate().filter(|(i,_)| i % 2 == 0).map(|(_,&v)| v as f64).sum();
        let shorts: f64 = intervals.iter().enumerate().filter(|(i,_)| i % 2 == 1).map(|(_,&v)| v as f64).sum();
        let lc = (intervals.len() + 1) / 2;
        let sc = intervals.len() / 2;
        if sc == 0 { return 1.0; }
        let avg_long = longs / lc as f64;
        let avg_short = shorts / sc as f64;
        if avg_short == 0.0 { 1.0 } else { avg_long / avg_short }
    }

    /// Groove intensity: (density + syncopation) / 2, 0.0-1.0.
    pub fn intensity(&self) -> f64 {
        if self.pattern.is_empty() { return 0.0; }
        let density = self.pattern.iter().filter(|&&v| v != Ternary::Zero).count() as f64 / self.pattern.len() as f64;
        let strong: Vec<usize> = (0..self.pattern.len()).step_by(2).collect();
        let sync = Syncopation::measure(&self.pattern, &strong);
        (density + sync) / 2.0
    }

    /// Regularity: how evenly spaced are the onsets (0.0-1.0).
    pub fn regularity(&self) -> f64 {
        let mut intervals = Vec::new();
        let mut last: Option<usize> = None;
        for (i, &v) in self.pattern.iter().enumerate() {
            if v != Ternary::Zero {
                if let Some(l) = last { intervals.push(i - l); }
                last = Some(i);
            }
        }
        if intervals.len() < 2 { return 1.0; }
        let sum: usize = intervals.iter().sum();
        let mean = sum as f64 / intervals.len() as f64;
        let variance = intervals.iter().map(|&v| { let d = v as f64 - mean; d * d }).sum::<f64>() / intervals.len() as f64;
        if mean == 0.0 { return 1.0; }
        let cv = variance.sqrt() / mean;
        1.0 / (1.0 + cv)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_groove_swing() {
        let g = Groove::new(vec![Ternary::Pos, Ternary::Zero, Ternary::Pos, Ternary::Zero]);
        assert!((g.swing_ratio() - 1.0).abs() < 0.01);
    }
    #[test] fn test_groove_intensity() {
        let g = Groove::new(vec![Ternary::Zero, Ternary::Pos, Ternary::Zero, Ternary::Pos]);
        assert!(g.intensity() > 0.0);
    }
    #[test] fn test_groove_regularity() {
        let g = Groove::new(vec![Ternary::Pos, Ternary::Zero, Ternary::Pos, Ternary::Zero]);
        assert!(g.regularity() > 0.9);
    }
}
