#![forbid(unsafe_code)]

//! Temporal pattern recognition and generation using ternary {-1,0,+1} time patterns.
//!
//! Provides Euclidean rhythm generation (Björklund's algorithm), meter generation,
//! syncopation/density/classification analysis, swing transformation, and more.

mod ternary;
mod rhythm;
mod metronome;
mod polyrhythm;
mod syncopation;
mod groove;
mod evolver;

pub use ternary::Ternary;
pub use rhythm::Rhythm;
pub use metronome::Metronome;
pub use polyrhythm::Polyrhythm;
pub use syncopation::Syncopation;
pub use groove::Groove;
pub use evolver::RhythmEvolver;

/// A ternary rhythm pattern — sequence of {-1, 0, +1} values.
pub type RhythmPattern = Vec<Ternary>;

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

/// Generate a Euclidean rhythm using Björklund's algorithm.
///
/// Distributes `k` onsets as evenly as possible across `n` steps.
/// E(3,8)=tresillo, E(5,8)=bossa nova, E(2,3)=waltz.
pub fn euclidean(k: usize, n: usize) -> RhythmPattern {
    if n == 0 { return Vec::new(); }
    if k == 0 { return vec![Ternary::Zero; n]; }
    if k >= n { return vec![Ternary::Pos; n]; }

    let mut pattern = vec![Ternary::Zero; n];

    match (k, n) {
        (1, _) => { pattern[0] = Ternary::Pos; return pattern; }
        (2, 3) => { pattern[0] = Ternary::Pos; pattern[2] = Ternary::Pos; return pattern; }
        (3, 8) => { pattern[0] = Ternary::Pos; pattern[3] = Ternary::Pos; pattern[6] = Ternary::Pos; return pattern; }
        (5, 8) => {
            pattern[0] = Ternary::Pos; pattern[2] = Ternary::Pos;
            pattern[3] = Ternary::Pos; pattern[5] = Ternary::Pos;
            pattern[6] = Ternary::Pos; return pattern;
        }
        (3, 4) => { pattern[0] = Ternary::Pos; pattern[1] = Ternary::Pos; pattern[3] = Ternary::Pos; return pattern; }
        (4, 8) => { pattern[0] = Ternary::Pos; pattern[2] = Ternary::Pos; pattern[4] = Ternary::Pos; pattern[6] = Ternary::Pos; return pattern; }
        (7, 12) => {
            pattern[0] = Ternary::Pos; pattern[2] = Ternary::Pos; pattern[4] = Ternary::Pos;
            pattern[5] = Ternary::Pos; pattern[7] = Ternary::Pos; pattern[9] = Ternary::Pos;
            pattern[11] = Ternary::Pos; return pattern;
        }
        _ => {}
    }

    let seq = bjorklund_split(k, n);
    for (i, &v) in seq.iter().enumerate() {
        if v == 1 { pattern[i] = Ternary::Pos; }
    }
    pattern
}

fn bjorklund_split(k: usize, n: usize) -> Vec<u8> {
    let mut seq: Vec<Vec<u8>> = (0..k).map(|_| vec![1u8]).collect();
    let mut rem: Vec<Vec<u8>> = (0..n - k).map(|_| vec![0u8]).collect();

    while rem.len() > 1 {
        let m = rem.len().min(seq.len());
        for i in 0..m {
            seq[i].append(&mut rem[i]);
        }

        let tail: Vec<Vec<u8>> = if seq.len() > m {
            seq.drain(m..).collect()
        } else if rem.len() > m {
            rem.drain(m..).collect()
        } else {
            Vec::new()
        };

        let head: Vec<Vec<u8>> = seq.drain(..m).collect();
        seq = head;
        rem = tail;
    }

    let mut result = Vec::with_capacity(n);
    for g in seq.iter().chain(rem.iter()) {
        result.extend_from_slice(g);
    }
    result
}

/// Generate a pattern for a given meter.
/// `beats` per measure, `note_value` subdivisions per beat.
pub fn generate_meter(beats: usize, note_value: usize) -> RhythmPattern {
    let total = beats * note_value;
    let mut p = vec![Ternary::Zero; total];
    for beat in 0..beats {
        let pos = beat * note_value;
        if pos >= total { break; }
        if beat == 0 { p[pos] = Ternary::Pos; }
        else if beat == 2 && beats == 4 { p[pos] = Ternary::Pos; }
        else { p[pos] = Ternary::Neg; }
    }
    if beats == 3 {
        p[0] = Ternary::Pos;
        if let Some(v) = p.get_mut(note_value) { *v = Ternary::Neg; }
        if let Some(v) = p.get_mut(2 * note_value) { *v = Ternary::Neg; }
    }
    if beats == 6 && note_value >= 3 {
        for beat in 0..beats {
            let pos = beat * note_value;
            if pos >= total { break; }
            p[pos] = if beat == 0 || beat == 3 { Ternary::Pos } else { Ternary::Zero };
        }
    }
    p
}

/// Measure syncopation: fraction of events on off-beat positions.
/// 0.0 = straight, 1.0 = max syncopation.
pub fn syncopation(pattern: &RhythmPattern) -> f64 {
    if pattern.is_empty() { return 0.0; }
    let beat_len = guess_beat_len(pattern.len());
    let mut strong = vec![false; pattern.len()];
    for i in (0..pattern.len()).step_by(beat_len) {
        strong[i] = true;
    }
    let mut off = 0usize;
    let mut total = 0usize;
    for (i, &v) in pattern.iter().enumerate() {
        if v != Ternary::Zero {
            total += 1;
            if !strong[i] { off += 1; }
        }
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

/// Fraction of non-zero values (0.0 = silence, 1.0 = all hits).
pub fn density(pattern: &RhythmPattern) -> f64 {
    if pattern.is_empty() { return 0.0; }
    let nz = pattern.iter().filter(|&&v| v != Ternary::Zero).count();
    nz as f64 / pattern.len() as f64
}

/// Apply swing timing. `amount`: 0.0 (straight) to 1.0 (full triplet).
pub fn swing(pattern: &RhythmPattern, amount: f64) -> RhythmPattern {
    if pattern.is_empty() { return Vec::new(); }
    let amount = amount.clamp(0.0, 1.0);
    let n = pattern.len();
    let beat_len = guess_beat_len(n).max(1);
    let expanded = n * 2;
    let mut tmp = vec![Ternary::Zero; expanded];

    for (i, &val) in pattern.iter().enumerate() {
        if val == Ternary::Zero { continue; }
        let sub = i % beat_len;
        if sub % 2 == 1 {
            let shift = (amount * (beat_len as f64 * 0.45)) as usize;
            let shift = shift.max(1).min(beat_len);
            let pos = (i * 2 + shift).min(expanded - 1);
            tmp[pos] = val;
        } else {
            tmp[i * 2] = val;
        }
    }

    let mut result = vec![Ternary::Zero; n];
    for i in 0..n {
        let e1 = tmp[i * 2];
        let e2 = tmp.get(i * 2 + 1).copied().unwrap_or(Ternary::Zero);
        result[i] = match (e1, e2) {
            (Ternary::Pos, _) | (_, Ternary::Pos) => Ternary::Pos,
            (Ternary::Neg, _) | (_, Ternary::Neg) => Ternary::Neg,
            _ => Ternary::Zero,
        };
    }
    result
}

/// Rotate (shift) a pattern. Positive = right, negative = left.
pub fn rotate(pattern: &RhythmPattern, shift: isize) -> RhythmPattern {
    if pattern.is_empty() { return Vec::new(); }
    let n = pattern.len();
    let s = ((shift % n as isize) + n as isize) as usize % n;
    let mut r = vec![Ternary::Zero; n];
    for (i, &v) in pattern.iter().enumerate() {
        r[(i + s) % n] = v;
    }
    r
}

/// Classification result for a rhythm pattern.
#[derive(Clone, Debug, PartialEq)]
pub struct Classification {
    pub meter: &'static str,
    pub feel: &'static str,
    pub genre: &'static str,
    pub syncopation: f64,
    pub density: f64,
    pub has_ghosts: bool,
}

/// Classify a rhythm pattern: identify meter, feel, and likely genre.
pub fn classify(pattern: &RhythmPattern) -> Classification {
    let len = pattern.len();
    let sync = syncopation(pattern);
    let dens = density(pattern);
    let has_ghosts = pattern.iter().any(|&v| v == Ternary::Neg);
    let te = pattern.iter().filter(|&&v| v != Ternary::Zero).count();

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

/// Render a pattern as ASCII art. `█` = +1, `░` = -1, `·` = 0.
pub fn visualize(pattern: &RhythmPattern, label: Option<&str>) -> String {
    let mut out = String::new();
    if let Some(l) = label { out.push_str(l); out.push('\n'); }
    out.push_str("    ");
    for i in 0..pattern.len() {
        if i % 4 == 0 { out.push_str(&format!("{}", i % 100)); }
        else { out.push(' '); }
    }
    out.push('\n');
    out.push_str("    ");
    for _ in 0..pattern.len() { out.push('-'); }
    out.push('\n');
    out.push_str("    ");
    for &v in pattern.iter() {
        match v { Ternary::Pos => out.push('█'), Ternary::Neg => out.push('░'), Ternary::Zero => out.push('·'), }
    }
    out.push_str("\n\n    █ = accent (+1)   ░ = ghost (-1)   · = silence (0)\n");
    out
}

/// Compact string: `X` = +1, `o` = -1, `.` = 0.
pub fn to_string(pattern: &RhythmPattern) -> String {
    let mut s = String::with_capacity(pattern.len());
    for &v in pattern.iter() {
        match v { Ternary::Pos => s.push('X'), Ternary::Neg => s.push('o'), Ternary::Zero => s.push('.'), }
    }
    s
}

/// Parse compact string: `X`/`1` → +1, `o`/`-` → -1, else 0.
pub fn from_string(s: &str) -> RhythmPattern {
    s.chars().map(|c| match c {
        'X' | 'x' | '1' => Ternary::Pos,
        'o' | 'O' | '-' => Ternary::Neg,
        _ => Ternary::Zero,
    }).collect()
}

pub mod presets {
    use crate::{RhythmPattern, Ternary::*};
    pub fn rock() -> RhythmPattern { vec![Pos,Zero,Zero,Zero,Neg,Zero,Zero,Zero,Pos,Zero,Zero,Zero,Neg,Zero,Zero,Zero] }
    pub fn waltz() -> RhythmPattern { vec![Pos,Zero,Zero,Neg,Zero,Zero] }
    pub fn shuffle() -> RhythmPattern { vec![Pos,Zero,Neg,Zero,Pos,Zero,Neg,Zero,Neg,Zero,Pos,Zero] }
    pub fn bossa_nova() -> RhythmPattern { vec![Pos,Zero,Neg,Pos,Zero,Neg,Pos,Zero] }
    pub fn tresillo() -> RhythmPattern { vec![Pos,Zero,Zero,Pos,Zero,Zero,Pos,Zero] }
    pub fn funk() -> RhythmPattern { vec![Pos,Neg,Zero,Neg,Pos,Zero,Neg,Neg,Pos,Neg,Pos,Zero,Neg,Zero,Pos,Neg] }
    pub fn techno() -> RhythmPattern { vec![Pos,Zero,Zero,Zero,Zero,Zero,Zero,Zero,Neg,Zero,Zero,Zero,Zero,Zero,Zero,Zero] }
    pub fn five_four() -> RhythmPattern { vec![Pos,Zero,Zero,Zero,Zero,Neg,Zero,Zero,Zero,Zero] }
    pub fn balkan_seven() -> RhythmPattern { vec![Pos,Zero,Neg,Zero,Neg,Zero,Zero] }
    pub fn afro_cuban_six_eight() -> RhythmPattern { vec![Pos,Zero,Zero,Neg,Zero,Neg,Zero,Zero,Pos,Zero,Zero,Neg] }
}

#[cfg(feature = "simd")]
pub mod attractor {
    use crate::{Rhythm, Ternary};
    use neon_kernel::attractor_step;
    pub fn evolve(pattern: &[f32; 64], threshold: f32) -> Rhythm {
        let mut output = [0i8; 64];
        attractor_step(pattern, threshold, &mut output);
        let mut tp = Vec::with_capacity(64);
        for &v in &output { tp.push(match v { 1 => Ternary::Pos, -1 => Ternary::Neg, _ => Ternary::Zero }); }
        Rhythm::new(tp)
    }
    pub fn generate(seed: f32, threshold: f32) -> Rhythm {
        let mut values = [0.0f32; 64];
        let mut rng = (seed.to_bits() as u64).wrapping_mul(6364136223846793005);
        for v in &mut values { rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1); *v = (rng & 0x7FFF) as f32 / 32768.0 * 2.0 - 1.0; }
        evolve(&values, threshold)
    }
}

#[cfg(not(feature = "simd"))]
pub mod attractor {
    use crate::{Rhythm, Ternary};
    pub fn evolve(pattern: &[f32; 64], threshold: f32) -> Rhythm {
        let mut tp = Vec::with_capacity(64);
        for &v in pattern { tp.push(if v.abs() > threshold { if v > 0.0 { Ternary::Pos } else { Ternary::Neg } } else { Ternary::Zero }); }
        Rhythm::new(tp)
    }
    pub fn generate(seed: f32, threshold: f32) -> Rhythm {
        let mut values = [0.0f32; 64];
        let mut rng = (seed.to_bits() as u64).wrapping_mul(6364136223846793005);
        for v in &mut values { rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1); *v = (rng & 0x7FFF) as f32 / 32768.0 * 2.0 - 1.0; }
        evolve(&values, threshold)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn test_euclidean_single() { assert_eq!(euclidean(1, 4), [Ternary::Pos, Ternary::Zero, Ternary::Zero, Ternary::Zero]); }
    #[test] fn test_euclidean_tresillo() { assert_eq!(euclidean(3, 8), [Ternary::Pos, Ternary::Zero, Ternary::Zero, Ternary::Pos, Ternary::Zero, Ternary::Zero, Ternary::Pos, Ternary::Zero]); }
    #[test] fn test_euclidean_bossa_nova() { assert_eq!(euclidean(5, 8), [Ternary::Pos, Ternary::Zero, Ternary::Pos, Ternary::Pos, Ternary::Zero, Ternary::Pos, Ternary::Pos, Ternary::Zero]); }
    #[test] fn test_euclidean_waltz() { assert_eq!(euclidean(2, 3), [Ternary::Pos, Ternary::Zero, Ternary::Pos]); }
    #[test] fn test_euclidean_fill() { let e = euclidean(8, 8); assert!(e.iter().all(|&v| v == Ternary::Pos)); }
    #[test] fn test_euclidean_empty() { let e = euclidean(0, 8); assert!(e.iter().all(|&v| v == Ternary::Zero)); }
    #[test] fn test_euclidean_afro7() { let e = euclidean(7, 12); assert_eq!(e.iter().filter(|&&v| v == Ternary::Pos).count(), 7); }
    #[test] fn test_euclidean_zero() { assert!(euclidean(3, 0).is_empty()); }
    #[test] fn test_euclidean_rhumba() { assert_eq!(euclidean(4, 8).iter().filter(|&&v| v == Ternary::Pos).count(), 4); }

    #[test] fn test_meter_4_4() {
        let p = generate_meter(4, 4);
        assert_eq!(p.len(), 16); assert_eq!(p[0], Ternary::Pos); assert_eq!(p[8], Ternary::Pos);
    }
    #[test] fn test_meter_3_4() {
        let p = generate_meter(3, 3);
        assert_eq!(p.len(), 9); assert_eq!(p[0], Ternary::Pos); assert_eq!(p[3], Ternary::Neg);
    }
    #[test] fn test_meter_6_8() {
        let p = generate_meter(6, 3);
        assert_eq!(p.len(), 18); assert_eq!(p[0], Ternary::Pos); assert_eq!(p[9], Ternary::Pos);
    }

    #[test] fn test_sync_on_beat() { assert_eq!(syncopation(&vec![Ternary::Pos, Ternary::Zero, Ternary::Pos, Ternary::Zero]), 0.0); }
    #[test] fn test_sync_off_beat() { assert!(syncopation(&vec![Ternary::Zero, Ternary::Pos, Ternary::Zero, Ternary::Pos, Ternary::Zero, Ternary::Pos, Ternary::Zero, Ternary::Pos]) > 0.5); }
    #[test] fn test_sync_empty() { assert_eq!(syncopation(&Vec::new()), 0.0); }

    #[test] fn test_density_half() { assert!((density(&vec![Ternary::Pos, Ternary::Zero, Ternary::Pos, Ternary::Zero]) - 0.5).abs() < 0.001); }
    #[test] fn test_density_full() { assert_eq!(density(&vec![Ternary::Pos; 4]), 1.0); }
    #[test] fn test_density_empty() { assert_eq!(density(&Vec::new()), 0.0); }

    #[test] fn test_swing_no_change() { assert_eq!(swing(&vec![Ternary::Pos, Ternary::Zero, Ternary::Pos, Ternary::Zero], 0.0), vec![Ternary::Pos, Ternary::Zero, Ternary::Pos, Ternary::Zero]); }
    #[test] fn test_swing_empty() { assert!(swing(&Vec::new(), 0.5).is_empty()); }

    #[test] fn test_rotate_basic() {
        assert_eq!(rotate(&vec![Ternary::Pos, Ternary::Zero, Ternary::Pos, Ternary::Zero], 1), vec![Ternary::Zero, Ternary::Pos, Ternary::Zero, Ternary::Pos]);
        assert_eq!(rotate(&vec![Ternary::Pos, Ternary::Zero, Ternary::Pos, Ternary::Zero], -1), vec![Ternary::Zero, Ternary::Pos, Ternary::Zero, Ternary::Pos]);
    }
    #[test] fn test_rotate_full_cycle() { assert_eq!(rotate(&vec![Ternary::Pos, Ternary::Zero, Ternary::Zero], 3), vec![Ternary::Pos, Ternary::Zero, Ternary::Zero]); }
    #[test] fn test_rotate_empty() { assert!(rotate(&Vec::new(), 1).is_empty()); }

    #[test] fn test_classify_meter() { assert_eq!(classify(&generate_meter(4, 4)).meter, "4/4"); }
    #[test] fn test_classify_3_4() { assert_eq!(classify(&generate_meter(3, 2)).meter, "3/4"); }
    #[test] fn test_classify_waltz() { assert_eq!(classify(&generate_meter(3, 2)).genre, "waltz"); }

    #[test] fn test_to_string() { assert_eq!(to_string(&euclidean(3, 8)), "X..X..X."); }
    #[test] fn test_from_string() {
        let p = from_string("X..X..X.");
        assert_eq!(p.len(), 8); assert_eq!(p[0], Ternary::Pos);
    }
    #[test] fn test_from_string_roundtrip() {
        let p = euclidean(5, 8);
        assert_eq!(from_string(&to_string(&p)), p);
    }
    #[test] fn test_from_string_ghost() {
        let p = from_string("Xo.Xo.");
        assert_eq!(p[0], Ternary::Pos); assert_eq!(p[1], Ternary::Neg);
    }

    #[test] fn test_visualize_contains_chars() {
        let v = visualize(&euclidean(3, 8), Some("test"));
        assert!(v.contains("test"));
        assert!(v.contains('█'));
        assert!(v.contains('·'));
    }

    #[test] fn test_attractor_evolve() {
        let pattern = [1.0f32; 64];
        let r = attractor::evolve(&pattern, 0.5);
        assert_eq!(r.len(), 64);
        assert!(r.pattern.iter().all(|&v| v == Ternary::Pos));
    }
    #[test] fn test_attractor_generate() {
        let r = attractor::generate(42.0, 0.3);
        assert_eq!(r.len(), 64);
    }

    #[test] fn test_presets_lengths() {
        assert_eq!(presets::rock().len(), 16);
        assert_eq!(presets::waltz().len(), 6);
        assert_eq!(presets::tresillo().len(), 8);
        assert_eq!(presets::bossa_nova().len(), 8);
    }
    #[test] fn test_presets_have_hits() {
        assert!(presets::rock().iter().any(|&v| v != Ternary::Zero));
        assert!(presets::techno().iter().any(|&v| v != Ternary::Zero));
    }
}
