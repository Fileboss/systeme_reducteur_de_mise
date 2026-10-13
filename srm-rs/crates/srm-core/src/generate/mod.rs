//! Génération d'une couverture minimale pour `(n, t)` : glouton puis recuit simulé,
//! sur plusieurs exécutions parallèles indépendantes.

pub mod anneal;
pub mod greedy;

use std::time::{Duration, Instant};

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;

use crate::cover::verify;
use crate::error::Result;
use crate::grid::Grid;
use crate::space::Space;
use crate::table::Table;

pub use anneal::{AnnealParams, Budget};

#[derive(Debug, Clone)]
pub struct GenConfig {
    pub seed: u64,
    /// Nombre d'exécutions indépendantes (graines `seed`, `seed + 1`, …), lancées en parallèle.
    pub runs: usize,
    /// Mouvements de recuit maximum par exécution (0 : glouton seul).
    pub max_moves: u64,
    /// Durée maximale de chaque exécution.
    pub time_limit: Option<Duration>,
    pub anneal: AnnealParams,
}

impl Default for GenConfig {
    fn default() -> Self {
        Self {
            seed: 1,
            runs: 1,
            max_moves: u64::MAX,
            time_limit: Some(Duration::from_secs(10)),
            anneal: AnnealParams::default(),
        }
    }
}

/// Résultat d'une génération.
#[derive(Debug, Clone)]
pub struct Generated {
    pub table: Table,
    /// Taille de la solution gloutonne de la meilleure exécution, pour comparaison.
    pub greedy_size: usize,
    /// Graine qui a produit la meilleure solution.
    pub seed: u64,
}

/// Une exécution : glouton puis recuit, avec sa propre graine.
pub fn run_once(space: &Space, seed: u64, cfg: &GenConfig) -> (usize, Vec<u64>) {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let start = greedy::greedy(space, &mut rng);
    let greedy_size = start.len();
    if cfg.max_moves == 0 {
        return (greedy_size, start);
    }
    let budget = Budget {
        max_moves: cfg.max_moves,
        deadline: cfg.time_limit.map(|d| Instant::now() + d),
    };
    (
        greedy_size,
        anneal::anneal(space, start, cfg.anneal, budget, &mut rng),
    )
}

/// Génère et vérifie une couverture pour `(n, t)`.
pub fn generate(n: usize, t: u8, cfg: &GenConfig) -> Result<Generated> {
    let space = Space::new(n, t)?;
    let (seed, greedy_size, best) = (0..cfg.runs.max(1) as u64)
        .into_par_iter()
        .map(|i| {
            let seed = cfg.seed.wrapping_add(i);
            let (g, sol) = run_once(&space, seed, cfg);
            (seed, g, sol)
        })
        .min_by_key(|(seed, _, sol)| (sol.len(), *seed))
        .expect("au moins une exécution");

    let mut grids: Vec<Grid> = best.into_iter().map(Grid::from_mask_unchecked).collect();
    grids.sort_unstable();
    verify(n, t, &grids)?;
    Ok(Generated {
        table: Table::new(n, t, grids)?,
        greedy_size,
        seed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_is_verified_and_deterministic_with_move_budget() {
        let cfg = GenConfig {
            seed: 5,
            runs: 3,
            max_moves: 30_000,
            time_limit: None,
            ..Default::default()
        };
        let a = generate(13, 3, &cfg).unwrap();
        let b = generate(13, 3, &cfg).unwrap();
        assert_eq!(a.table.grids(), b.table.grids());
        assert_eq!(a.seed, b.seed);
        assert!(a.table.len() <= a.greedy_size);
    }

    #[test]
    fn greedy_only_when_no_moves() {
        let cfg = GenConfig {
            max_moves: 0,
            time_limit: None,
            ..Default::default()
        };
        let g = generate(10, 2, &cfg).unwrap();
        assert_eq!(g.table.len(), g.greedy_size);
    }
}
