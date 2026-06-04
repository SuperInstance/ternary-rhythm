# ternary-rhythm — Temporal pattern recognition and generation

Rhythm struct, metronome, polyrhythm, syncopation detection, groove analysis, and rhythmic evolution for ternary time patterns. Every beat is {-1, 0, +1}.

## Why This Exists

Ternary agents need temporal coordination — not just spatial or logical reasoning. Time is fundamental to music, speech, biological cycles, and any system that acts periodically. This crate provides ternary-valued rhythm structures where Pos means "onset/accent", Neg means "anti-onset/rest", and Zero means "silent/unmarked" — enabling pattern recognition, generation, and evolution of temporal structures.

## Core Concepts

- **Balanced ternary** — Three values: Neg (-1), Zero (0), Pos (+1). In rhythms, Pos = onset, Neg = anti-onset (off-beat emphasis), Zero = silence.
- **Rhythm** — A repeating ternary pattern with a playback cursor. `tick()` advances and returns the current value. Supports density, balance, reversal, and rotation.
- **Metronome** — A steady beat generator with configurable accent pattern. First beat is accented by default.
- **Polyrhythm** — Multiple simultaneous rhythms with independent lengths. The full cycle repeats at the LCM of all rhythm lengths (e.g., 4-beat and 6-beat → 12-tick cycle).
- **Syncopation** — Off-beat emphasis. Measured as the fraction of events on weak positions. Can also create syncopated versions by shifting onsets.
- **Groove** — The "feel" of a rhythm. Analyzed through swing ratio (long-short interval alternation), intensity (density + syncopation), and regularity (how evenly spaced the onsets are).
- **RhythmEvolver** — Genetic evolution of rhythms: fitness rewards moderate density and high regularity. Uses crossover and mutation to breed better patterns.

## Quick Start

```toml
[dependencies]
ternary-rhythm = "0.1"
```

```rust
use ternary_rhythm::*;

// Create a ternary rhythm
let mut rhythm = Rhythm::new(vec![
    Ternary::Pos, Ternary::Zero, Ternary::Neg, Ternary::Zero,
]);

// Tick through the pattern
assert_eq!(rhythm.tick(), Ternary::Pos);
assert_eq!(rhythm.tick(), Ternary::Zero);

// Build a polyrhythm (3 against 4)
let mut poly = Polyrhythm::new(vec![
    Rhythm::new(vec![Ternary::Pos, Ternary::Zero, Ternary::Zero]),
    Rhythm::new(vec![Ternary::Pos, Ternary::Zero, Ternary::Pos, Ternary::Zero]),
]);
let both = poly.tick(); // [Pos, Pos]
assert_eq!(poly.cycle_length(), 12); // LCM(3, 4) = 12

// Analyze groove
let groove = Groove::new(vec![Ternary::Pos, Ternary::Zero, Ternary::Pos, Ternary::Zero]);
println!("Regularity: {:.2}", groove.regularity());
println!("Intensity: {:.2}", groove.intensity());
```

## API Overview

| Type | Purpose |
|------|---------|
| `Ternary` | Core value: Neg (-1), Zero (0), Pos (+1) |
| `Rhythm` | Repeating ternary time pattern with playback cursor |
| `Metronome` | Steady beat generator with configurable accents |
| `Polyrhythm` | Multiple simultaneous rhythms, cycling at LCM |
| `Syncopation` | Off-beat measurement and syncopation creation |
| `Groove` | Swing ratio, intensity, and regularity analysis |
| `RhythmEvolver` | Genetic evolution of rhythmic patterns |

## How It Works

Rhythms are flat arrays of ternary values with a circular cursor. Each `tick()` returns the current value and advances. Polyrhythms combine multiple rhythms — since they have different lengths, the full pattern repeats at the LCM of all lengths, creating the characteristic interlocking patterns of real polyrhythms.

Groove analysis is purely interval-based: it measures the intervals between onsets and computes statistics. Swing ratio compares alternating long/short intervals (a swing feel has ratio > 1.0). Regularity is the inverse of the coefficient of variation of intervals.

Evolution uses a simple genetic algorithm: fitness rewards rhythms with density near 0.5 and high regularity. Top-half survive, breed via midpoint crossover, and mutate by randomly flipping positions.

## Known Limitations

- **No time signature awareness** — Patterns don't know about meters (4/4, 3/4). Metrical structure must be managed externally.
- **Discrete time only** — No fractional tick positions. Everything is integer-indexed.
- **Simple PRNG** — Uses a linear congruential generator. Not suitable for cryptographic purposes.
- **Groove analysis assumes monophonic** — Swing and regularity assume a single voice. Multiple simultaneous onsets confuse interval tracking.
- **Fitness is opinionated** — The built-in fitness function prefers moderate density and high regularity. Unusual aesthetics (very sparse, very irregular) score poorly.
- **No velocity** — Ternary values are discrete. No continuous dynamics (loud/soft).

## Use Cases

1. **Musical pattern generation** — Create and evolve ternary rhythm patterns for music applications, using polyrhythms for complex textures.
2. **Temporal agent coordination** — Synchronize ternary agents to shared temporal patterns, with metronomes providing master clocks.
3. **Biological cycle modeling** — Represent circadian or other periodic biological rhythms as ternary patterns (active/rest/neutral phases).

## Ecosystem Context

Part of the SuperInstance ternary crate family. `ternary-rhythm` connects to `ternary-music` for musical applications and `ternary-tidelight` for tidal/scheduling integration. It provides the temporal backbone that `ternary-agent` can use for periodic behavior and `ternary-scheduling` can use for time-based coordination.

## License

MIT
