# ternary-rhythm

[![MIDI Tensor](https://img.shields.io/badge/MIDI-Tensor--Enabled-8B0000?style=flat)](../prototypes/README.md)
[![Rhythm Engine](https://img.shields.io/badge/Rhythm-Ternary-FF69B4?style=flat)](.)

**Temporal pattern recognition and generation. The pulse that drives everything.**

Temporal pattern recognition and generation using ternary time — rhythm structures, metronomes, polyrhythms, syncopation detection, groove analysis, and rhythmic evolution for ternary-valued temporal coordination.

Rhythm is the most fundamental musical element. Before melody, before harmony, before timbre — there's rhythm. A pattern of hits and silences that marks time, creates expectation, and resolves it. In ternary, rhythm is a sequence of {-1, 0, +1}: accented (+1), silent (0), or unaccented/ghost (-1).

This crate provides a complete rhythm toolkit: pattern generation (from simple beats to Euclidean algorithms), pattern analysis (syncopation, density, swing), pattern transformation (rotate, invert, permute), and pattern classification (identify the meter, the feel, the genre).

## What's Inside

- **`RhythmPattern`** — a sequence of {-1, 0, +1} values representing a rhythmic cycle
- **`euclidean(k, n)`** — Björklund's algorithm: distribute k hits as evenly as possible in n steps. The math behind every classic rhythm
- **`generate_meter(beats, subdivisions)`** — generate patterns for common meters (4/4, 3/4, 6/8, 7/8)
- **`syncopation(pattern)`** — measure how "off-beat" the pattern is. High syncopation = jazz/funk, low = march/polka
- **`density(pattern)`** — fraction of non-zero values. Sparse (0.2) = minimal techno, dense (0.8) = drum & bass
- **`swing(pattern, amount)`** — apply swing/shuffle timing. 0 = straight, 1 = full triplet swing
- **`rotate_beats(pattern, shift)`** — shift the pattern by N beats. New feel, same rhythm
- **`classify(pattern)`** — identify the meter and feel: 4/4 straight, 3/4 waltz, 6/8 shuffle, etc.

## Quick Example

```rust
use ternary_rhythm::*;

// Euclidean rhythm: 5 beats in 8 steps (bossa nova)
let bossa = euclidean(5, 8);
// [1, 0, 1, 1, 0, 1, 1, 0]

// Classic 4/4 rock beat
let rock = generate_meter(4, 4);
// [1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0]

// Measure syncopation
let offbeat = RhythmPattern::new(vec![0, 1, 0, 1, 0, 1, 0, 1]);
let onbeat = RhythmPattern::new(vec![1, 0, 1, 0, 1, 0, 1, 0]);
println!("Offbeat syncopation: {:.2}", syncopation(&offbeat)); // high
println!("Onbeat syncopation: {:.2}", syncopation(&onbeat));   // low

// Apply swing
let swung = swing(&rock, 0.6);
// Off-beat hits shift later — the feel changes completely
```

## The Deeper Truth

**Euclidean rhythms are mathematically optimal.** Björklund's algorithm distributes k hits in n steps as evenly as possible — and the result is almost every important rhythm in world music. E(3,8) = Cuban tresillo. E(5,8) = bossa nova. E(7,12) = West African bell pattern. E(2,3) = every waltz ever. The algorithm doesn't know about music — it just distributes things evenly — and yet it produces the rhythms that cultures around the world independently discovered. There's a deep connection between mathematical evenness and musical satisfaction.

The ternary dimension adds accent levels: +1 = downbeat (the ONE), 0 = silence, -1 = ghost note (quiet hit that fills the space). Ghost notes are what separates a stiff drum machine from a living drummer. They're the whispers between the shouts — felt more than heard. In ternary, they're the -1 values that give the rhythm its *feel* rather than just its *pattern*.

**Use cases:**
- **Algorithmic composition** — generate rhythm patterns from mathematical rules
- **Drum machines** — the Euclidean algorithm IS the drum machine
- **Music education** — teach rhythm theory with the simplest possible representation
- **Game audio** — adaptive rhythm that responds to gameplay
- **Dance** — rhythm generation for choreography

## See Also

- **ternary-polyrhythm** — multiple rhythms playing simultaneously
- **ternary-tempo** — how fast the rhythm plays (BPM)
- **ternary-fib** — period-8 as the natural ternary rhythm
- **ternary-jam** — rhythmic improvisation in a jam session
- **ternary-sync** — Z₃ synchronization (when rhythms lock in)
- **ternary-phase** — phase relationships between rhythmic layers
- **ternary-ear** — rhythm recognition training

## Install

```bash
cargo add ternary-rhythm
```

## License

MIT
