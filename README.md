# ternary-rhythm

[![CI](https://github.com/SuperInstance/ternary-rhythm/actions/workflows/rust.yml/badge.svg)](https://github.com/SuperInstance/ternary-rhythm/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/ternary-rhythm.svg)](https://crates.io/crates/ternary-rhythm)
[![docs.rs](https://img.shields.io/docsrs/ternary-rhythm)](https://docs.rs/ternary-rhythm)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Temporal pattern recognition and generation using ternary {-1, 0, +1} time patterns.

Rhythm is the most fundamental musical element. Before melody, before harmony, before timbre — there's rhythm. A pattern of hits and silences that marks time, creates expectation, and resolves it. In ternary, rhythm is a sequence of {-1, 0, +1}: accented (+1), silent (0), or unaccented/ghost (-1).

## Features

- **Euclidean rhythm generation** (Björklund's algorithm) — `euclidean(k, n)`
- **Meter generation** — `generate_meter(beats, note_value)` for 4/4, 3/4, 6/8, etc.
- **Syncopation analysis** — measure off-beat density
- **Density measurement** — fraction of non-zero values
- **Swing transformation** — apply shuffle/swing feel
- **Pattern rotation** — shift patterns in time
- **Classification** — detect meter, feel, and likely genre
- **Visualization** — ASCII art pattern display
- **Attractor-based evolution** (optional, requires `simd` feature)
- **Genetic algorithm evolver** — evolve patterns through mutation and selection
- **No external dependencies** for the core library
- **57+ tests** covering all functionality

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ternary-rhythm = "0.2"
```

## Quick Start

```rust
use ternary_rhythm::*;

// Euclidean rhythm: 5 hits in 8 steps (bossa nova)
let bossa = euclidean(5, 8);
assert_eq!(bossa.len(), 8);

// Measure syncopation
let offbeat = vec![0, 1, 0, 1, 0, 1, 0, 1];
let onbeat  = vec![1, 0, 1, 0, 1, 0, 1, 0];
assert!(syncopation(&offbeat) > syncopation(&onbeat));

// Generate a 4/4 rock beat
let rock = generate_meter(4, 4);
assert_eq!(rock.len(), 16);

// Apply swing
let swung = swing(&rock, 0.6);
assert_eq!(swung.len(), 16);

// Classify
let class = classify(&tresillo);
assert_eq!(class.meter, "4/4");
```

## CLI Usage

```bash
ternary-rhythm euclidean 3 8     # E(3,8) = tresillo
ternary-rhythm meter 4 4         # 4/4 meter pattern
ternary-rhythm analyze "X..X..X."  # Analyze a pattern
ternary-rhythm swing "X.X.X.X." 0.6  # Apply swing
ternary-rhythm preset bossa_nova  # Show preset
ternary-rhythm presets            # List all presets
```

### Pattern String Notation

| Char | Meaning |
|------|---------|
| `X`  | Accented hit (+1) |
| `o`  | Ghost note (-1) |
| `.`  | Silence (0) |

## Euclidean Rhythms

The [Euclidean algorithm](https://en.wikipedia.org/wiki/Euclidean_rhythm) distributes `k` beats evenly across `n` steps:

| k | n | Pattern Name | Rhythm |
|---|---|---|---|
| 2 | 3 | Waltz | `X.X` |
| 3 | 4 | — | `XX.X` |
| 3 | 8 | Tresillo | `X..X..X.` |
| 4 | 8 | Rhumba | `X.X.X.X.` |
| 5 | 8 | Bossa Nova | `X.XX.XX.` |
| 7 | 12 | Afro-Cuban | `X.XX.XX.XX.X` |

## API Overview

### Functions

- `euclidean(k, n)` — Euclidean rhythm generation
- `generate_meter(beats, note_value)` — Meter generation
- `syncopation(pattern)` — Syncopation score (0.0–1.0)
- `density(pattern)` — Density score (0.0–1.0)
- `swing(pattern, amount)` — Apply swing (0.0–1.0)
- `rotate(pattern, shift)` — Rotate pattern
- `classify(pattern)` — Full classification
- `visualize(pattern, label)` — ASCII art
- `to_string(pattern)` / `from_string(s)` — Compact string conversion

### Types

- `RhythmPattern` — `Vec<Ternary>` alias
- `Ternary` — `{-1, 0, +1}` values
- `Rhythm` — Pattern with position tracking
- `Metronome` — Beat counter
- `Polyrhythm` — Multiple rhythms simultaneously
- `Syncopation` — Syncopation analysis
- `Groove` — Groove/feel detection
- `RhythmEvolver` — Genetic algorithm pattern evolution
- `Classification` — Pattern classification result

### Presets

```rust
use ternary_rhythm::presets;

let rock = presets::rock();
let waltz = presets::waltz();
let bossa = presets::bossa_nova();
let funk = presets::funk();
```

## License

MIT © SuperInstance
