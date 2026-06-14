use crate::{Groove, Rhythm, Ternary};


/// Evolve rhythmic patterns through mutation and selection (genetic algorithm).
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
    /// Create a new evolver with an initial population.
    pub fn new(population: Vec<Rhythm>, mutation_rate: u32, seed: u64) -> Self {
        Self {
            population,
            mutation_rate,
            seed,
        }
    }

    /// Fitness: balance between density and regularity.
    ///
    /// Rewards moderate density (near 0.5) and high regularity.
    pub fn fitness(rhythm: &Rhythm) -> f64 {
        if rhythm.is_empty() {
            return 0.0;
        }
        let groove = Groove::new(rhythm.pattern.clone());
        let density = rhythm.density();
        let reg = groove.regularity();
        // Reward moderate density and high regularity
        let density_score = 1.0 - (density - 0.5).abs() * 2.0;
        (density_score + reg) / 2.0
    }

    /// Mutate a single rhythm in-place.
    pub fn mutate(&mut self, idx: usize) {
        if idx >= self.population.len() {
            return;
        }
        let pattern = &mut self.population[idx].pattern;
        for val in pattern.iter_mut() {
            self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            if ((self.seed % 1000) as u32) < self.mutation_rate {
                *val = Ternary::random(&mut self.seed);
            }
        }
    }

    /// Crossover two rhythms: child takes first half from `a`, second half from `b`.
    pub fn crossover(a: &Rhythm, b: &Rhythm) -> Rhythm {
        let len = a.len().min(b.len());
        let mid = len / 2;
        let mut child = Vec::with_capacity(len);
        child.extend_from_slice(&a.pattern[..mid]);
        child.extend_from_slice(&b.pattern[mid..len]);
        Rhythm::new(child)
    }

    /// Run one generation: evaluate, select, breed, mutate.
    ///
    /// Returns the best fitness in the population after evolution.
    pub fn evolve(&mut self) -> f64 {
        if self.population.len() < 2 {
            return 0.0;
        }

        // Evaluate fitness
        let mut scored: Vec<(usize, f64)> = self
            .population
            .iter()
            .enumerate()
            .map(|(i, r)| (i, Self::fitness(r)))
            .collect();

        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(core::cmp::Ordering::Equal));

        // Keep top half
        let keep = scored.len() / 2;
        let survivors: Vec<usize> = scored[..keep].iter().map(|&(i, _)| i).collect();

        // Breed new individuals from survivors
        let mut new_pop: Vec<Rhythm> =
            survivors.iter().map(|&i| self.population[i].clone()).collect();

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
    fn test_rhythm_evolver_fitness() {
        let r = Rhythm::new(vec![
            Ternary::Pos,
            Ternary::Zero,
            Ternary::Pos,
            Ternary::Zero,
        ]);
        let f = RhythmEvolver::fitness(&r);
        assert!(f > 0.0);
    }

    #[test]
    fn test_rhythm_evolver_crossover() {
        let a = Rhythm::new(vec![Ternary::Pos, Ternary::Pos, Ternary::Pos, Ternary::Pos]);
        let b = Rhythm::new(vec![Ternary::Neg, Ternary::Neg, Ternary::Neg, Ternary::Neg]);
        let child = RhythmEvolver::crossover(&a, &b);
        assert_eq!(child.len(), 4);
        // First half from a, second from b
        assert_eq!(child.pattern[0], Ternary::Pos);
        assert_eq!(child.pattern[3], Ternary::Neg);
    }

    #[test]
    fn test_rhythm_evolver_evolve() {
        let pop = vec![
            Rhythm::new(vec![
                Ternary::Pos,
                Ternary::Zero,
                Ternary::Neg,
                Ternary::Zero,
            ]),
            Rhythm::new(vec![
                Ternary::Neg,
                Ternary::Pos,
                Ternary::Zero,
                Ternary::Neg,
            ]),
            Rhythm::new(vec![
                Ternary::Zero,
                Ternary::Pos,
                Ternary::Zero,
                Ternary::Pos,
            ]),
            Rhythm::new(vec![
                Ternary::Pos,
                Ternary::Neg,
                Ternary::Pos,
                Ternary::Neg,
            ]),
        ];
        let mut evolver = RhythmEvolver::new(pop, 100, 42);
        let best = evolver.evolve();
        assert!(best >= 0.0);
        assert_eq!(evolver.population.len(), 4);
    }
}
