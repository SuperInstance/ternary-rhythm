# CROSS-POLLINATION.md — ternary-rhythm

> **Conservation Law Connection:** Rhythms reveal γ/η oscillation modes

## Role in the Conservation Law

`ternary-rhythm` studies temporal patterns in ternary signals. In the conservation
law framework:

- **Steady rhythm** → system near equilibrium (γ stable, η ≈ δ(n))
- **Accelerating rhythm** → system transitioning (η spiking, γ reorganizing)
- **Irregular rhythm** → system under stress (adversarial pressure, drift > 15%)
- **Silent channels** → dead agents (η contribution zero, γ lost)

The rhythm of a healthy fleet should show δ(n)-bounded oscillations: periodic η
fluctuations with amplitude ≈ δ(n) × √n, frequency related to fleet size.

## delta-clt Verification Results

The delta-clt time-series data (1000 trials × 8 fleet sizes) contains rhythm
information that the current suite does not extract:

- **Predicted rhythm frequency:** f ≈ 1/(2π√n) — larger fleets oscillate slower
- **Predicted rhythm amplitude:** A ≈ δ(n) — the CLT floor
- **Anomaly detection:** any oscillation with A > 2δ(n) indicates hidden correlation

The correlated fleet simulation (30% shared bias) would show rhythm disruption —
this is detectable by `ternary-rhythm` analysis in production.

## Cross-Repo Connections

### → ternary-hamiltonian
Rhythm extracts the eigenmodes of the Hamiltonian. Each rhythm frequency IS a
normal mode of the fleet's energy dynamics.

**Shared:** Both describe temporal dynamics of ternary systems.
**Different:** Hamiltonian gives equations; rhythm extracts observed frequencies.

### → ternary-entropy
Entropy and rhythm are complementary: entropy = spatial uncertainty, rhythm =
temporal pattern. Together: spatiotemporal characterization of fleet behavior.

**Shared:** Both measure fleet signal properties over ternary distributions.
**Different:** Entropy is distribution-focused (snapshot); rhythm is time-focused (trajectory).

### → ternary-pid
Rhythm detects oscillations; PID damps them. Closed-loop: rhythm analysis feeds
PID tuning parameters.

**Shared:** Both operate on time-series ternary data.
**Different:** Rhythm is analytical; PID is operational control.

## Fleet Position

```
┌──────────────────────────────────────────────────┐
│  ternary-rhythm — THE TEMPORAL LENS               │
│                                                   │
│  Input:  time-series of Trit values per agent     │
│  Output: frequencies, amplitudes, anomalies       │
│                                                   │
│  Healthy fleet: f ≈ 1/(2π√n), A ≈ δ(n)           │
│  Anomalous fleet: A > 2δ(n) → hidden correlation │
│                                                   │
│  Pairs with:                                      │
│  ├─ ternary-entropy (spatial uncertainty)         │
│  ├─ ternary-hamiltonian (normal modes)            │
│  └─ ternary-pid (oscillation damping)             │
│                                                   │
│  Can detect: adversarial agents, correlation      │
│  bias, impending phase transitions                │
└──────────────────────────────────────────────────┘
```

