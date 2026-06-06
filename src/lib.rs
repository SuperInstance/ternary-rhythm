#![forbid(unsafe_code)]

//! Temporal pattern recognition and generation using ternary time patterns.
//!
//! Provides rhythm structures, metronomes, polyrhythms, syncopation detection,
//! groove analysis, and rhythmic evolution for ternary-valued temporal coordination.

/// Canonical ternary type re-exported from `ternary-types`.
pub use ternary_types::Ternary;

/// Extension trait providing methods previously on the custom `Ternary` type.
pub trait TernaryExt {
    /// Create a `Ternary` from an `i8` value (-1, 0, or 1).
    fn from_i8(v: i8) -> Option<Self>
    where
        Self: Sized;
    /// Return the `i8` value of this ternary state.
    fn to_i8(self) -> i8;
    /// Generate a random `Ternary` value using a simple LCG.
    fn random(seed: &mut u64) -> Self
    where
        Self: Sized;
}

impl TernaryExt for ternary_types::Ternary {
    fn from_i8(v: i8) -> Option<Self> {
        Self::try_from(v).ok()
    }

    fn to_i8(self) -> i8 {
        i8::from(self)
    }

    fn random(seed: &mut u64) -> Self {
        *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        match (*seed % 3) as i8 {
            0 => Ternary::Negative,
            1 => Ternary::Neutral,
            _ => Ternary::Positive,
        }
    }
}

/// A ternary time pattern: a sequence of ternary values over discrete ticks.
#[derive(Clone, Debug)]
pub struct Rhythm {
    /// Pattern values.
    pub pattern: Vec<Ternary>,
    /// Current playback position.
    pub position: usize,
}

impl Rhythm {
    pub fn new(pattern: Vec<Ternary>) -> Self {
        Self { pattern, position: 0 }
    }

    /// Length of the pattern.
    pub fn len(&self) -> usize {
        self.pattern.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pattern.is_empty()
    }

    /// Advance one tick and return the current value.
    pub fn tick(&mut self) -> Ternary {
        if self.pattern.is_empty() { return Ternary::Neutral; }
        let val = self.pattern[self.position];
        self.position = (self.position + 1) % self.pattern.len();
        val
    }

    /// Peek at current value without advancing.
    pub fn current(&self) -> Ternary {
        if self.pattern.is_empty() { Ternary::Neutral } else { self.pattern[self.position] }
    }

    /// Reset to beginning.
    pub fn reset(&mut self) {
        self.position = 0;
    }

    /// Density: fraction of non-zero elements.
    pub fn density(&self) -> f64 {
        if self.pattern.is_empty() { return 0.0; }
        let non_zero = self.pattern.iter().filter(|&&v| v != Ternary::Neutral).count();
        non_zero as f64 / self.pattern.len() as f64
    }

    /// Balance: ratio of positive to negative values.
    /// Returns (pos_count, neg_count).
    pub fn balance(&self) -> (usize, usize) {
        let pos = self.pattern.iter().filter(|&&v| v == Ternary::Positive).count();
        let neg = self.pattern.iter().filter(|&&v| v == Ternary::Negative).count();
        (pos, neg)
    }

    /// Reverse the pattern.
    pub fn reverse(&mut self) {
        self.pattern.reverse();
    }

    /// Shift the pattern by n positions (rotate).
    pub fn rotate(&mut self, n: usize) {
        if self.pattern.is_empty() { return; }
        let n = n % self.pattern.len();
        let split = self.pattern.len() - n;
        let rotated: Vec<Ternary> = self.pattern[split..].iter().chain(self.pattern[..split].iter()).copied().collect();
        self.pattern = rotated;
    }
}

/// A steady tick generator (metronome).
#[derive(Clone, Debug)]
pub struct Metronome {
    /// Beats per measure.
    pub beats: usize,
    /// Current beat position.
    pub position: usize,
    /// Accent pattern: which beats are accented (Pos), normal (Zero), rest (Neg).
    pub accents: Vec<Ternary>,
}

impl Metronome {
    pub fn new(beats: usize) -> Self {
        let mut accents = vec![Ternary::Neutral; beats];
        if beats > 0 { accents[0] = Ternary::Positive; } // accent first beat
        Self { beats, position: 0, accents }
    }

    /// Set accent pattern.
    pub fn set_accents(&mut self, accents: Vec<Ternary>) {
        self.accents = accents;
        // Pad or truncate to match beats
        self.accents.resize(self.beats, Ternary::Neutral);
    }

    /// Tick forward one beat.
    pub fn tick(&mut self) -> Ternary {
        if self.beats == 0 { return Ternary::Neutral; }
        let val = if self.position < self.accents.len() { self.accents[self.position] } else { Ternary::Neutral };
        self.position = (self.position + 1) % self.beats;
        val
    }

    /// Whether we're on the downbeat.
    pub fn is_downbeat(&self) -> bool {
        self.position == 0
    }

    /// Reset to beat 0.
    pub fn reset(&mut self) {
        self.position = 0;
    }
}

/// Multiple simultaneous rhythm patterns.
#[derive(Clone, Debug)]
pub struct Polyrhythm {
    /// Individual rhythms.
    pub rhythms: Vec<Rhythm>,
}

impl Polyrhythm {
    pub fn new(rhythms: Vec<Rhythm>) -> Self {
        Self { rhythms }
    }

    /// Tick all rhythms and return their combined output.
    pub fn tick(&mut self) -> Vec<Ternary> {
        self.rhythms.iter_mut().map(|r| r.tick()).collect()
    }

    /// Least common multiple of all rhythm lengths (full cycle length).
    pub fn cycle_length(&self) -> usize {
        if self.rhythms.is_empty() { return 0; }
        let mut lcm = self.rhythms[0].len();
        for r in &self.rhythms[1..] {
            lcm = lcm_value(lcm, r.len());
        }
        lcm
    }

    /// Number of rhythms.
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

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm_value(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 { return 0; }
    a / gcd(a, b) * b
}

/// Off-beat emphasis detection and creation.
#[derive(Clone, Debug)]
pub struct Syncopation;

impl Syncopation {
    /// Measure syncopation: count beats where strong position is Zero
    /// and weak position is non-zero.
    pub fn measure(pattern: &[Ternary], strong_positions: &[usize]) -> f64 {
        if pattern.is_empty() { return 0.0; }
        let len = pattern.len();
        let mut syncopated = 0usize;
        let mut total = 0usize;

        for i in 0..len {
            let is_strong = strong_positions.contains(&i);
            let val = pattern[i];
            // Syncopation: weak position has event, strong doesn't
            if !is_strong && val != Ternary::Neutral {
                syncopated += 1;
            }
            total += 1;
        }
        syncopated as f64 / total as f64
    }

    /// Create a syncopated version by shifting onsets to off-beats.
    pub fn syncopate(pattern: &[Ternary], shift: usize) -> Vec<Ternary> {
        let mut result = vec![Ternary::Neutral; pattern.len()];
        for i in 0..pattern.len() {
            if pattern[i] != Ternary::Neutral {
                let new_pos = (i + shift) % pattern.len();
                result[new_pos] = pattern[i];
            }
        }
        result
    }
}

/// Groove/feel detection from a rhythm pattern.
#[derive(Clone, Debug)]
pub struct Groove {
    /// The rhythm being analyzed.
    pub pattern: Vec<Ternary>,
}

impl Groove {
    pub fn new(pattern: Vec<Ternary>) -> Self {
        Self { pattern }
    }

    /// Swing ratio: ratio of long to short intervals.
    /// In a swung rhythm, alternating intervals are long-short-long-short.
    pub fn swing_ratio(&self) -> f64 {
        let mut intervals = Vec::new();
        let mut last_onset = None;
        for (i, &v) in self.pattern.iter().enumerate() {
            if v != Ternary::Neutral {
                if let Some(last) = last_onset {
                    intervals.push(i - last);
                }
                last_onset = Some(i);
            }
        }
        if intervals.len() < 2 { return 1.0; }
        let longs: f64 = intervals.iter().enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .map(|(_, &v)| v as f64)
            .sum::<f64>();
        let short_count = intervals.iter().enumerate().filter(|(i, _)| i % 2 == 1).count();
        let long_count = intervals.iter().enumerate().filter(|(i, _)| i % 2 == 0).count();
        if short_count == 0 || long_count == 0 { return 1.0; }
        let avg_long = longs / long_count as f64;
        let shorts: f64 = intervals.iter().enumerate()
            .filter(|(i, _)| i % 2 == 1)
            .map(|(_, &v)| v as f64)
            .sum::<f64>();
        let avg_short = shorts / short_count as f64;
        if avg_short == 0.0 { return 1.0; }
        avg_long / avg_short
    }

    /// Groove intensity: combination of syncopation and density.
    pub fn intensity(&self) -> f64 {
        let density = if self.pattern.is_empty() { 0.0 } else {
            self.pattern.iter().filter(|&&v| v != Ternary::Neutral).count() as f64 / self.pattern.len() as f64
        };
        let strong: Vec<usize> = (0..self.pattern.len()).step_by(2).collect();
        let sync = Syncopation::measure(&self.pattern, &strong);
        (density + sync) / 2.0
    }

    /// Regularity: how evenly spaced the onsets are.
    pub fn regularity(&self) -> f64 {
        let mut intervals = Vec::new();
        let mut last = None;
        for (i, &v) in self.pattern.iter().enumerate() {
            if v != Ternary::Neutral {
                if let Some(l) = last {
                    intervals.push(i - l);
                }
                last = Some(i);
            }
        }
        if intervals.len() < 2 { return 1.0; }
        let mean = intervals.iter().sum::<usize>() as f64 / intervals.len() as f64;
        let variance = intervals.iter()
            .map(|&v| {
                let diff = v as f64 - mean;
                diff * diff
            })
            .sum::<f64>() / intervals.len() as f64;
        if mean == 0.0 { return 1.0; }
        let cv = variance.sqrt() / mean; // coefficient of variation
        1.0 / (1.0 + cv) // normalized to 0..1
    }
}

/// Evolve rhythmic patterns through mutation and selection.
#[derive(Clone, Debug)]
pub struct RhythmEvolver {
    /// Population of rhythms.
    pub population: Vec<Rhythm>,
    /// Mutation rate (per-thousand per position).
    pub mutation_rate: u32,
    /// PRNG seed.
    pub seed: u64,
}

impl RhythmEvolver {
    pub fn new(population: Vec<Rhythm>, mutation_rate: u32, seed: u64) -> Self {
        Self { population, mutation_rate, seed }
    }

    /// Fitness: balance between density and regularity.
    pub fn fitness(rhythm: &Rhythm) -> f64 {
        if rhythm.is_empty() { return 0.0; }
        let groove = Groove::new(rhythm.pattern.clone());
        let density = rhythm.density();
        let reg = groove.regularity();
        // Reward moderate density and high regularity
        let density_score = 1.0 - (density - 0.5).abs() * 2.0;
        (density_score + reg) / 2.0
    }

    /// Mutate a single rhythm.
    pub fn mutate(&mut self, idx: usize) {
        if idx >= self.population.len() { return; }
        let pattern = &mut self.population[idx].pattern;
        for val in pattern.iter_mut() {
            self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            if ((self.seed % 1000) as u32) < self.mutation_rate {
                *val = Ternary::random(&mut self.seed);
            }
        }
    }

    /// Crossover two rhythms: child = first half of a + second half of b.
    pub fn crossover(a: &Rhythm, b: &Rhythm) -> Rhythm {
        let len = a.len().min(b.len());
        let mid = len / 2;
        let mut child = Vec::with_capacity(len);
        child.extend_from_slice(&a.pattern[..mid]);
        child.extend_from_slice(&b.pattern[mid..len]);
        Rhythm::new(child)
    }

    /// Run one generation: evaluate, select, breed, mutate.
    pub fn evolve(&mut self) -> f64 {
        if self.population.len() < 2 { return 0.0; }

        // Evaluate fitness
        let mut scored: Vec<(usize, f64)> = self.population.iter()
            .enumerate()
            .map(|(i, r)| (i, Self::fitness(r)))
            .collect();

        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Keep top half
        let keep = scored.len() / 2;
        let survivors: Vec<usize> = scored[..keep].iter().map(|&(i, _)| i).collect();

        // Breed new individuals
        let mut new_pop: Vec<Rhythm> = survivors.iter().map(|&i| self.population[i].clone()).collect();

        while new_pop.len() < self.population.len() {
            let p1 = survivors[self.seed as usize % survivors.len()];
            self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let p2 = survivors[self.seed as usize % survivors.len()];
            let child = Self::crossover(&self.population[p1], &self.population[p2]);
            new_pop.push(child);
        }

        self.population = new_pop;

        // Mutate
        for i in 0..self.population.len() {
            self.mutate(i);
        }

        // Return best fitness
        scored.first().map(|&(_, f)| f).unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rhythm_tick() {
        let mut r = Rhythm::new(vec![Ternary::Positive, Ternary::Neutral, Ternary::Negative]);
        assert_eq!(r.tick(), Ternary::Positive);
        assert_eq!(r.tick(), Ternary::Neutral);
        assert_eq!(r.tick(), Ternary::Negative);
        assert_eq!(r.tick(), Ternary::Positive); // wraps
    }

    #[test]
    fn test_rhythm_density() {
        let r = Rhythm::new(vec![Ternary::Positive, Ternary::Neutral, Ternary::Negative, Ternary::Neutral]);
        assert!((r.density() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_rhythm_balance() {
        let r = Rhythm::new(vec![Ternary::Positive, Ternary::Positive, Ternary::Negative, Ternary::Neutral]);
        assert_eq!(r.balance(), (2, 1));
    }

    #[test]
    fn test_rhythm_reverse() {
        let mut r = Rhythm::new(vec![Ternary::Positive, Ternary::Neutral, Ternary::Negative]);
        r.reverse();
        assert_eq!(r.pattern, vec![Ternary::Negative, Ternary::Neutral, Ternary::Positive]);
    }

    #[test]
    fn test_rhythm_rotate() {
        let mut r = Rhythm::new(vec![Ternary::Positive, Ternary::Neutral, Ternary::Negative]);
        r.rotate(1);
        assert_eq!(r.pattern, vec![Ternary::Negative, Ternary::Positive, Ternary::Neutral]);
    }

    #[test]
    fn test_rhythm_empty() {
        let mut r = Rhythm::new(vec![]);
        assert_eq!(r.tick(), Ternary::Neutral);
        assert!(r.is_empty());
        assert_eq!(r.density(), 0.0);
    }

    #[test]
    fn test_rhythm_reset() {
        let mut r = Rhythm::new(vec![Ternary::Positive, Ternary::Negative]);
        r.tick();
        r.tick();
        r.reset();
        assert_eq!(r.position, 0);
    }

    #[test]
    fn test_metronome_tick() {
        let mut m = Metronome::new(4);
        assert_eq!(m.tick(), Ternary::Positive); // first beat accented
        assert_eq!(m.tick(), Ternary::Neutral);
        assert_eq!(m.tick(), Ternary::Neutral);
        assert_eq!(m.tick(), Ternary::Neutral);
        assert!(m.is_downbeat()); // back to start
    }

    #[test]
    fn test_metronome_custom_accents() {
        let mut m = Metronome::new(3);
        m.set_accents(vec![Ternary::Positive, Ternary::Positive, Ternary::Positive]);
        assert_eq!(m.tick(), Ternary::Positive);
        assert_eq!(m.tick(), Ternary::Positive);
        assert_eq!(m.tick(), Ternary::Positive);
    }

    #[test]
    fn test_metronome_reset() {
        let mut m = Metronome::new(4);
        m.tick();
        m.tick();
        m.reset();
        assert_eq!(m.position, 0);
    }

    #[test]
    fn test_polyrhythm_tick() {
        let mut p = Polyrhythm::new(vec![
            Rhythm::new(vec![Ternary::Positive, Ternary::Neutral]),
            Rhythm::new(vec![Ternary::Negative, Ternary::Neutral, Ternary::Neutral]),
        ]);
        let vals = p.tick();
        assert_eq!(vals, vec![Ternary::Positive, Ternary::Negative]);
    }

    #[test]
    fn test_polyrhythm_cycle_length() {
        let p = Polyrhythm::new(vec![
            Rhythm::new(vec![Ternary::Neutral; 4]),
            Rhythm::new(vec![Ternary::Neutral; 6]),
        ]);
        assert_eq!(p.cycle_length(), 12); // LCM(4,6) = 12
    }

    #[test]
    fn test_polyrhythm_voice_count() {
        let p = Polyrhythm::new(vec![
            Rhythm::new(vec![Ternary::Neutral]),
            Rhythm::new(vec![Ternary::Neutral]),
            Rhythm::new(vec![Ternary::Neutral]),
        ]);
        assert_eq!(p.voice_count(), 3);
    }

    #[test]
    fn test_syncopation_measure() {
        // All on strong beats → low syncopation
        let pattern = vec![Ternary::Positive, Ternary::Neutral, Ternary::Positive, Ternary::Neutral];
        let strong = vec![0, 2];
        let sync = Syncopation::measure(&pattern, &strong);
        assert_eq!(sync, 0.0); // no off-beat events

        // All on weak beats → high syncopation
        let pattern2 = vec![Ternary::Neutral, Ternary::Positive, Ternary::Neutral, Ternary::Positive];
        let sync2 = Syncopation::measure(&pattern2, &strong);
        assert!(sync2 > 0.0);
    }

    #[test]
    fn test_syncopation_create() {
        let pattern = vec![Ternary::Positive, Ternary::Neutral, Ternary::Positive, Ternary::Neutral];
        let syncopated = Syncopation::syncopate(&pattern, 1);
        assert_eq!(syncopated, vec![Ternary::Neutral, Ternary::Positive, Ternary::Neutral, Ternary::Positive]);
    }

    #[test]
    fn test_groove_swing_ratio() {
        // Regular pattern: swing = 1.0
        let g = Groove::new(vec![Ternary::Positive, Ternary::Neutral, Ternary::Positive, Ternary::Neutral]);
        let swing = g.swing_ratio();
        assert!((swing - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_groove_intensity() {
        // Dense syncopated → high intensity
        let g = Groove::new(vec![Ternary::Neutral, Ternary::Positive, Ternary::Neutral, Ternary::Positive]);
        let intensity = g.intensity();
        assert!(intensity > 0.0);
    }

    #[test]
    fn test_groove_regularity() {
        // Perfectly regular: onset every 2
        let g = Groove::new(vec![Ternary::Positive, Ternary::Neutral, Ternary::Positive, Ternary::Neutral]);
        let reg = g.regularity();
        assert!(reg > 0.9);
    }

    #[test]
    fn test_rhythm_evolver_fitness() {
        let r = Rhythm::new(vec![Ternary::Positive, Ternary::Neutral, Ternary::Positive, Ternary::Neutral]);
        let f = RhythmEvolver::fitness(&r);
        assert!(f > 0.0);
    }

    #[test]
    fn test_rhythm_evolver_crossover() {
        let a = Rhythm::new(vec![Ternary::Positive, Ternary::Positive, Ternary::Positive, Ternary::Positive]);
        let b = Rhythm::new(vec![Ternary::Negative, Ternary::Negative, Ternary::Negative, Ternary::Negative]);
        let child = RhythmEvolver::crossover(&a, &b);
        assert_eq!(child.len(), 4);
        // First half from a, second from b
        assert_eq!(child.pattern[0], Ternary::Positive);
        assert_eq!(child.pattern[3], Ternary::Negative);
    }

    #[test]
    fn test_rhythm_evolver_evolve() {
        let pop = vec![
            Rhythm::new(vec![Ternary::Positive, Ternary::Neutral, Ternary::Negative, Ternary::Neutral]),
            Rhythm::new(vec![Ternary::Negative, Ternary::Positive, Ternary::Neutral, Ternary::Negative]),
            Rhythm::new(vec![Ternary::Neutral, Ternary::Positive, Ternary::Neutral, Ternary::Positive]),
            Rhythm::new(vec![Ternary::Positive, Ternary::Negative, Ternary::Positive, Ternary::Negative]),
        ];
        let mut evolver = RhythmEvolver::new(pop, 100, 42);
        let best = evolver.evolve();
        assert!(best >= 0.0);
        assert_eq!(evolver.population.len(), 4);
    }

    #[test]
    fn test_gcd_lcm() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(lcm_value(4, 6), 12);
        assert_eq!(lcm_value(0, 5), 0);
    }
}
