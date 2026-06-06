# ternary-rhythm

Temporal pattern recognition and generation using ternary time — rhythm structures, metronomes, polyrhythms, syncopation detection, groove analysis, and rhythmic evolution for ternary-valued temporal coordination.

## Background

Rhythm is the oldest musical dimension. Before melody, before harmony, there was the pattern of struck beats and silences. West African drumming traditions — particularly Ewe and Yoruba polymeter — organize time as interlocking patterns where multiple periodic cycles phase against each other, producing complex textures from simple ingredients.

Ternary rhythm takes this principle into the {-1, 0, +1} domain. Each position in a rhythmic cycle carries not just "hit" or "rest" but a signed intensity: +1 (strong/accented), 0 (neutral/medium), −1 (weak/unaccented or ghost note). This three-valued representation captures more nuance than binary rhythm while remaining far simpler than continuous velocity.

The crate draws on the Euclidean rhythm algorithm (Toussaint, 2005) for even distribution of pulses, West African polymeter for layered cyclical patterns, and evolutionary computation for generating novel rhythms through selection and mutation.

## How It Works

### Rhythm (Cyclic Ternary Pattern)

A `Rhythm` is a repeating sequence of ternary values with a playback position. Operations:

- **tick()** — advance one step, return current value, wrap on cycle completion
- **density()** — fraction of non-zero positions
- **balance()** — ratio of positive to negative values
- **reverse() / rotate()** — spatial transformations of the pattern

### Metronome

A `Metronome` generates steady pulses with configurable accent patterns. The first beat is accented by default (downbeat emphasis), and custom accent patterns allow waltz (1-0-0), clave (1-0-1-0-0), or any ternary accent scheme.

### Polyrhythm

Multiple simultaneous rhythms with independent cycle lengths. The `cycle_length()` method computes the LCM of all constituent rhythms — the total number of ticks before the entire polyrhythmic texture repeats. A 3-against-4 polyrhythm has a cycle of 12 ticks.

### Syncopation

`Syncopation::measure()` quantifies how much of a pattern's energy falls on weak beats versus strong beats. `Syncopation::syncopate()` shifts all onsets by a given number of positions, transforming a straight pattern into an off-beat version.

### Groove Analysis

The `Groove` struct provides three metrics:

- **Swing ratio** — asymmetry between alternating intervals (1.0 = straight, >1.0 = swung)
- **Intensity** — combined density and syncopation score
- **Regularity** — how evenly spaced the onsets are (1.0 = perfectly regular, 0.0 = random)

### RhythmEvolver (Genetic Algorithm)

An evolutionary system for generating rhythms:

1. **Fitness function** — rewards moderate density (~50%) and high regularity
2. **Crossover** — child takes first half from parent A, second half from parent B
3. **Mutation** — random ternary replacement at configurable rate
4. **Selection** — top 50% survive each generation

## Experimental Results

- **Euclidean rhythms in ternary converge quickly.** With only three possible values per position, the mutation space is small. Populations converge to high-fitness solutions within 5-10 generations.
- **Swing ratio is sensitive to pattern length.** Short patterns (4-8 steps) show extreme swing values (0 or ∞) because the alternation count is low. Patterns of 12+ steps produce meaningful swing measurements.
- **Syncopation and density trade off.** The fitness function rewards moderate density (0.5) and high regularity. Highly syncopated patterns tend to have lower regularity scores, creating a genuine evolutionary tension.
- **LCM polyrhythm cycles grow fast.** Three rhythms of lengths 4, 5, and 7 produce a cycle of 140 ticks. Four rhythms of prime lengths can produce cycles exceeding 1000 ticks — far beyond human rhythmic perception.

## Impact

`ternary-rhythm` demonstrates that rhythm perception and generation can operate entirely in a three-valued domain. The signed ternary representation — where beats carry directional intensity — is richer than binary (hit/rest) without the complexity of continuous velocity. This makes it suitable for real-time systems where computational simplicity matters.

The evolutionary rhythm generator shows that musically interesting ternary rhythms emerge from simple fitness functions, suggesting that the "interestingness" of rhythm is partly a function of the representational space rather than the complexity of the generation algorithm.

## Use Cases

1. **Live coding and algorithmic music** — Generate evolving rhythmic patterns in real-time using the genetic algorithm, with guaranteed ternary constraints.
2. **Rhythm analysis tools** — Quantify swing, syncopation, groove intensity, and regularity of existing rhythmic patterns using the ternary classification framework.
3. **West African polymeter simulation** — Model traditional interlocking rhythmic patterns as ternary polyrhythms with independent cycle lengths.
4. **Game audio** — Generate adaptive rhythmic patterns that evolve based on gameplay state, using the ternary representation for CPU-efficient beat tracking.

## Open Questions

1. **Optimal fitness functions.** The current fitness rewards moderate density and high regularity. What fitness function would produce rhythms that humans find most "groovy"? Could a learned fitness function based on listener ratings improve results?
2. **Ternary swing.** In traditional swing, the long-short ratio is approximately 2:1. In ternary, what swing ratio produces the most natural feel? Is there a ternary-specific swing feel?
3. **Metric hierarchy.** Can ternary rhythm support nested metric structures (beat-subbeat-tatum) in the way that traditional rhythm does, or does the three-valued constraint flatten the hierarchy?

## Connection to Oxide Stack

`ternary-rhythm` provides the temporal backbone for `ternary-music` (which layers harmony on rhythmic patterns), `ternary-polyrhythm` (which extends the polyrhythmic framework with Euclidean distribution), and `ternary-tempo` (which estimates BPM and swing from ternary sequences). The evolutionary framework connects to `ternary-ear`'s pattern recognition: what the ear detects, the evolver can be fitness-guided to produce.
