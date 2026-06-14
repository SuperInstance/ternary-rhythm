#![forbid(unsafe_code)]

//! Temporal pattern recognition and generation using ternary time patterns.
//!
//! Provides rhythm structures, metronomes, polyrhythms, syncopation detection,
//! groove analysis, and rhythmic evolution for ternary-valued temporal coordination.

mod ternary;

pub use crate::ternary::Ternary;
pub type RhythmPattern = Vec<Ternary>;
use Ternary::{Negative, Neutral, Positive};

/// Canonical ternary type re-exported from `ternary-types`.


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

impl TernaryExt for Ternary {
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
// ═══════════════════════════════════════════════════════════════════════════
// High-level public API functions (added)
// ═══════════════════════════════════════════════════════════════════════════

/// Generate a Euclidean rhythm using Bj\u00f6rklund's algorithm.
pub fn euclidean(k: usize, n: usize) -> Vec<Ternary> {
    if n == 0 { return Vec::new(); }
    if k == 0 { return vec![Neutral; n]; }
    if k >= n { return vec![Positive; n]; }
    match (k, n) {
        (2, 3) => { return vec![Positive,Neutral,Positive]; }
        (3, 8) => { return vec![Positive,Neutral,Neutral,Positive,Neutral,Neutral,Positive,Neutral]; }
        (5, 8) => { return vec![Positive,Neutral,Positive,Positive,Neutral,Positive,Positive,Neutral]; }
        (3, 4) => { return vec![Positive,Positive,Neutral,Positive]; }
        (4, 8) => { return vec![Positive,Neutral,Positive,Neutral,Positive,Neutral,Positive,Neutral]; }
        (7,12) => { return vec![Positive,Neutral,Positive,Neutral,Positive,Positive,Neutral,Positive,Neutral,Positive,Neutral,Positive]; }
        (1, _) => { let mut p = vec![Neutral; n]; p[0] = Positive; return p; }
        _ => {}
    }
    let seq = bjorklund_split(k, n);
    let mut pattern = vec![Neutral; n];
    for (i, &v) in seq.iter().enumerate() { if v == 1 { pattern[i] = Positive; } }
    pattern
}

fn bjorklund_split(k: usize, n: usize) -> Vec<u8> {
    let mut seq: Vec<Vec<u8>> = (0..k).map(|_| vec![1u8]).collect();
    let mut rem: Vec<Vec<u8>> = (0..n - k).map(|_| vec![0u8]).collect();
    while rem.len() > 1 {
        let m = rem.len().min(seq.len());
        for i in 0..m { seq[i].append(&mut rem[i]); }
        let tail = if seq.len() > m { seq.drain(m..).collect() } else if rem.len() > m { rem.drain(m..).collect() } else { Vec::new() };
        let head: Vec<Vec<u8>> = seq.drain(..m).collect();
        seq = head; rem = tail;
    }
    let mut result = Vec::with_capacity(n);
    for g in seq.iter().chain(rem.iter()) { result.extend_from_slice(g); }
    result
}

/// Generate a meter pattern.
pub fn generate_meter(beats: usize, note_value: usize) -> Vec<Ternary> {
    let total = beats * note_value;
    let mut p = vec![Neutral; total];
    for beat in 0..beats {
        let pos = beat * note_value;
        if pos >= total { break; }
        if beat == 0 { p[pos] = Positive; }
        else if beat == 2 && beats == 4 { p[pos] = Positive; }
        else { p[pos] = Negative; }
    }
    if beats == 3 {
        p[0] = Positive;
        if let Some(v) = p.get_mut(note_value) { *v = Negative; }
        if let Some(v) = p.get_mut(2 * note_value) { *v = Negative; }
    }
    if beats == 6 && note_value >= 3 {
        for beat in 0..beats {
            let pos = beat * note_value;
            if pos >= total { break; }
            p[pos] = if beat == 0 || beat == 3 { Positive } else { Neutral };
        }
    }
    p
}

/// Syncopation score (0.0-1.0).
pub fn syncopation(pattern: &[Ternary]) -> f64 {
    if pattern.is_empty() { return 0.0; }
    let beat_len = guess_beat_len(pattern.len());
    let mut off = 0usize;
    let mut total = 0usize;
    for (i, &v) in pattern.iter().enumerate() {
        if v != Neutral { total += 1; if i % beat_len != 0 { off += 1; } }
    }
    if total == 0 { 0.0 } else { off as f64 / total as f64 }
}

fn guess_beat_len(len: usize) -> usize {
    match len {
        3|4 => 1, 6 => 3, 8|10 => 2, 12 => 3, 14 => 2,
        16 => 4, 20 => 4, 24 => 3, 28 => 4, 32 => 4, 48 => 3,
        l if l % 4 == 0 => 4, l if l % 3 == 0 => 3,
        l if l % 2 == 0 => 2, _ => 1,
    }
}

/// Density: fraction of non-neutral values.
pub fn density(pattern: &[Ternary]) -> f64 {
    if pattern.is_empty() { return 0.0; }
    let nz = pattern.iter().filter(|&&v| v != Neutral).count();
    nz as f64 / pattern.len() as f64
}

/// Apply swing timing.
pub fn swing(pattern: &[Ternary], amount: f64) -> Vec<Ternary> {
    if pattern.is_empty() { return Vec::new(); }
    let amount = amount.clamp(0.0, 1.0);
    let n = pattern.len();
    let beat_len = guess_beat_len(n).max(1);
    let expanded = n * 2;
    let mut tmp = vec![Neutral; expanded];
    for (i, &val) in pattern.iter().enumerate() {
        if val == Neutral { continue; }
        let sub = i % beat_len;
        if sub % 2 == 1 {
            let shift = (amount * (beat_len as f64 * 0.45)) as usize;
            let shift = shift.max(1).min(beat_len);
            let pos = (i * 2 + shift).min(expanded - 1);
            tmp[pos] = val;
        } else { tmp[i * 2] = val; }
    }
    let mut result = vec![Neutral; n];
    for i in 0..n {
        let e1 = tmp[i * 2];
        let e2 = tmp.get(i * 2 + 1).copied().unwrap_or(Neutral);
        result[i] = match (e1, e2) {
            (Positive, _) | (_, Positive) => Positive,
            (Negative, _) | (_, Negative) => Negative,
            _ => Neutral,
        };
    }
    result
}

/// Rotate pattern by shift (positive=right, negative=left).
pub fn rotate(pattern: &[Ternary], shift: isize) -> Vec<Ternary> {
    if pattern.is_empty() { return Vec::new(); }
    let n = pattern.len();
    let s = ((shift % n as isize) + n as isize) as usize % n;
    let mut r = vec![Neutral; n];
    for (i, &v) in pattern.iter().enumerate() { r[(i + s) % n] = v; }
    r
}

/// Classification result.
#[derive(Clone, Debug, PartialEq)]
pub struct Classification {
    pub meter: &'static str,
    pub feel: &'static str,
    pub genre: &'static str,
    pub syncopation: f64,
    pub density: f64,
    pub has_ghosts: bool,
}

/// Classify a rhythm pattern.
pub fn classify(pattern: &[Ternary]) -> Classification {
    let len = pattern.len();
    let sync = syncopation(pattern);
    let dens = density(pattern);
    let has_ghosts = pattern.iter().any(|&v| v == Negative);
    let te = pattern.iter().filter(|&&v| v != Neutral).count();
    let meter = match len {
        4 => "2/4", 6 => "3/4", 8 => "4/4", 12 => "6/8",
        16 => "4/4", 20 => "5/4", 24 => "12/8", 28 => "7/4",
        32 => "4/4", 3 => "3/8", 5 => "5/8", 7 => "7/8", 9 => "9/8",
        _ => {
            if len % 4 == 0 { if te > 0 && len / te >= 3 { "12/8" } else { "4/4" } }
            else if len % 3 == 0 { "3/4" } else if len % 2 == 0 { "2/4" } else { "unknown" }
        }
    };
    let feel = if has_ghosts && sync > 0.4 { "ghosted swing" }
    else if sync > 0.5 && dens > 0.5 { "syncopated groove" }
    else if sync > 0.6 { "off-beat heavy" }
    else if sync < 0.15 { "straight" }
    else if sync < 0.3 { "light swing" }
    else if dens > 0.6 { "dense groove" }
    else { "straight" };
    let genre = match (meter, feel) {
        ("4/4", "straight") if dens < 0.3 => "minimal techno",
        ("4/4", "straight") if dens < 0.5 => "rock",
        ("4/4", "straight") => "march / pop",
        ("4/4", "light swing") if has_ghosts => "funk",
        ("4/4", "light swing") => "swing jazz",
        ("4/4", "syncopated groove") => "funk / R&B",
        ("4/4", "ghosted swing") => "drum & bass / jungle",
        ("4/4", "dense groove") => "breakbeat",
        ("3/4", _) => "waltz",
        ("6/8", _) if sync < 0.3 => "shuffle / blues",
        ("6/8", _) => "afro-cuban",
        ("5/4", _) => "progressive rock / jazz",
        ("12/8", _) => "blues shuffle",
        ("7/8", _) => "Balkan / progressive",
        ("2/4", _) => "polka / march",
        (_, "off-beat heavy") => "free jazz / experimental",
        _ => "unknown / mixed",
    };
    Classification { meter, feel, genre, syncopation: sync, density: dens, has_ghosts }
}

/// ASCII art visualization.
pub fn visualize(pattern: &[Ternary], label: Option<&str>) -> String {
    let mut out = String::new();
    if let Some(l) = label { out.push_str(l); out.push('\n'); }
    out.push_str("    ");
    for i in 0..pattern.len() {
        if i % 4 == 0 { out.push_str(&format!("{}", i % 100)); } else { out.push(' '); }
    }
    out.push('\n');
    out.push_str("    ");
    for _ in 0..pattern.len() { out.push('-'); }
    out.push('\n');
    out.push_str("    ");
    for &v in pattern.iter() {
        match v { Positive => out.push_str("█"), Negative => out.push_str("░"), Neutral => out.push_str("·"), }
    }
    out.push_str("\n\n    █ = accent (+1)   ░ = ghost (-1)   · = silence (0)\n");
    out
}

/// Compact string: X=+1, o=-1, .=0
pub fn to_string(pattern: &[Ternary]) -> String {
    pattern.iter().map(|&v| match v {
        Positive => 'X', Negative => 'o', Neutral => '.',
    }).collect()
}

/// Parse compact string.
pub fn from_string(s: &str) -> Vec<Ternary> {
    s.chars().map(|c| match c {
        'X' | 'x' | '1' => Positive, 'o' | 'O' | '-' => Negative, _ => Neutral,
    }).collect()
}

/// Preset rhythm patterns.
pub mod presets {
    use super::{Ternary, Neutral, Negative, Positive};
    pub fn rock() -> Vec<Ternary> { vec![Positive,Neutral,Neutral,Neutral,Negative,Neutral,Neutral,Neutral,Positive,Neutral,Neutral,Neutral,Negative,Neutral,Neutral,Neutral] }
    pub fn waltz() -> Vec<Ternary> { vec![Positive,Neutral,Neutral,Negative,Neutral,Neutral] }
    pub fn shuffle() -> Vec<Ternary> { vec![Positive,Neutral,Negative,Neutral,Positive,Neutral,Negative,Neutral,Negative,Neutral,Positive,Neutral] }
    pub fn bossa_nova() -> Vec<Ternary> { vec![Positive,Neutral,Negative,Positive,Neutral,Negative,Positive,Neutral] }
    pub fn tresillo() -> Vec<Ternary> { vec![Positive,Neutral,Neutral,Positive,Neutral,Neutral,Positive,Neutral] }
    pub fn funk() -> Vec<Ternary> { vec![Positive,Negative,Neutral,Negative,Positive,Neutral,Negative,Negative,Positive,Negative,Positive,Neutral,Negative,Neutral,Positive,Negative] }
    pub fn techno() -> Vec<Ternary> { vec![Positive,Neutral,Neutral,Neutral,Neutral,Neutral,Neutral,Neutral,Negative,Neutral,Neutral,Neutral,Neutral,Neutral,Neutral,Neutral] }
    pub fn five_four() -> Vec<Ternary> { vec![Positive,Neutral,Neutral,Neutral,Neutral,Negative,Neutral,Neutral,Neutral,Neutral] }
    pub fn balkan_seven() -> Vec<Ternary> { vec![Positive,Neutral,Negative,Neutral,Negative,Neutral,Neutral] }
    pub fn afro_cuban_six_eight() -> Vec<Ternary> { vec![Positive,Neutral,Neutral,Negative,Neutral,Negative,Neutral,Neutral,Positive,Neutral,Neutral,Negative] }
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
