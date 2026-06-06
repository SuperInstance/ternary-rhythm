# PLUG_AND_PLAY — Rhythm

> Temporal pattern recognition with ternary time signatures

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
ternary-rhythm = { git = "https://github.com/SuperInstance/ternary-rhythm" }
```

Use in your code:

```rust
use ternary_rhythm::{TernaryPattern, Metronome};

let pattern = TernaryPattern::new(&[1, -1, 0, 1]);
let mut metro = Metronome::new(120, pattern);
loop { metro.tick(); }
```

## 🔗 Integration

This crate is part of the [SuperInstance ternary fleet](https://github.com/SuperInstance). It uses the canonical `Ternary` type from `ternary-types` for cross-crate compatibility.

## 📄 License

MIT
