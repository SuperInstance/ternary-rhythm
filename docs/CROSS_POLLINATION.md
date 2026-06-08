# Cross-Pollination: ternary-rhythm ⇄ fleet-midi-tidalcycles

## The Chain

```
ternary-rhythm (Rust) → rhythm pattern → fleet-midi-tidalcycles (Python) → TidalCycles code
```

`ternary-rhythm` has a Rhythm struct with `tick()`, `density()`, `balance()` methods.
`fleet-midi-tidalcycles` converts ternary vectors to TidalCycles pattern strings.

These two repos solve complementary halves of the same problem:
- **ternary-rhythm** analyzes *what the rhythm IS* (metrics, structure)
- **fleet-midi-tidalcycles** generates *what the rhythm SOUNDS LIKE* (TidalCycles code)

## How to Glue

```python
# 1. Use ternary-rhythm to analyze a pattern
# 2. Pass the analysis to tidalcycles for rendering
    
from lib.pattern_engine import vector_to_pattern

# Any ternary rhythm analyzed by ternary-rhythm
density = 0.625  # from ternary-rhythm analysis
vector = [1, 0, -1, 1, 0, -1, 1, 1]  # the original pattern

# tidalcycles renders it
pattern = vector_to_pattern(vector, "cross-pollinated")
print(f"TidalCycles output: {pattern}")
```

## Related Repos
- [fleet-midi-tidalcycles](https://github.com/SuperInstance/fleet-midi-tidalcycles)
- [ternary-rhythm](https://github.com/SuperInstance/ternary-rhythm)
- [fleet-ternary-music](https://github.com/SuperInstance/fleet-ternary-music) — the core mathematical bridge
