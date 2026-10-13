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
    /// Couverture valide dont partir au lieu du glouton (par exemple la table existante),
    /// pour que chaque nouvelle recherche prolonge les précédentes.
    pub initial: Option<Vec<Grid>>,
}

impl Default for GenConfig {
    fn default() -> Self {
        Self {
            seed: 1,
            runs: 1,
            max_moves: u64::MAX,
            time_limit: Some(Duration::from_secs(10)),
            anneal: AnnealParams::default(),
            initial: None,
        }
    }
}

/// Résultat d'une génération.
#[derive(Debug, Clone)]
pub struct Generated {
    pub table: Table,
    /// Taille de la solution de départ (glouton ou `initial`) de la meilleure exécution.
    pub start_size: usize,
    /// Graine qui a produit la meilleure solution.
    pub seed: u64,
}

/// Une exécution : glouton (ou solution initiale) puis recuit, avec sa propre graine.
pub fn run_once(space: &Space, seed: u64, cfg: &GenConfig) -> (usize, Vec<u64>) {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let start = match &cfg.initial {
        Some(grids) => grids.iter().map(|g| g.mask()).collect(),
        None => greedy::greedy(space, &mut rng),
    };
    let start_size = start.len();
    if cfg.max_moves == 0 {
        return (start_size, start);
    }
    let budget = Budget {
        max_moves: cfg.max_moves,
        deadline: cfg.time_limit.map(|d| Instant::now() + d),
    };
    (
        start_size,
        anneal::anneal(space, start, cfg.anneal, budget, &mut rng),
    )
}

/// Génère et vérifie une couverture pour `(n, t)`.
pub fn generate(n: usize, t: u8, cfg: &GenConfig) -> Result<Generated> {
    let space = Space::new(n, t)?;
    if let Some(initial) = &cfg.initial {
        // Le recuit suppose une solution de départ valide.
        verify(n, t, initial)?;
    }
    let (seed, start_size, best) = (0..cfg.runs.max(1) as u64)
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
        start_size,
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
        assert!(a.table.len() <= a.start_size);
    }

    #[test]
    fn greedy_only_when_no_moves() {
        let cfg = GenConfig {
            max_moves: 0,
            time_limit: None,
            ..Default::default()
        };
        let g = generate(10, 2, &cfg).unwrap();
        assert_eq!(g.table.len(), g.start_size);
    }

    #[test]
    fn resumes_from_initial_solution() {
        let first = GenConfig {
            max_moves: 0,
            time_limit: None,
            ..Default::default()
        };
        let greedy = generate(14, 3, &first).unwrap().table;
        let resumed = GenConfig {
            runs: 2,
            max_moves: 20_000,
            time_limit: None,
            initial: Some(greedy.grids().to_vec()),
            ..Default::default()
        };
        let g = generate(14, 3, &resumed).unwrap();
        assert_eq!(g.start_size, greedy.len());
        assert!(g.table.len() <= greedy.len());
    }

    #[test]
    fn rejects_invalid_initial_solution() {
        let cfg = GenConfig {
            initial: Some(vec![Grid::new(&[1, 2, 3, 4, 5]).unwrap()]),
            ..Default::default()
        };
        assert!(generate(14, 3, &cfg).is_err());
    }
}
