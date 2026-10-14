//! Probabilités et espérance : ce que le système garantit vraiment, et ce qu'il coûte.
//!
//! Modèle : 5 numéros gagnants tirés uniformément parmi `pool` (50 à l'EuroMillions, 49 au Loto).
//! Les compléments sont traités dans [`crate::extras`] : leur tirage est indépendant.

use rand::SeedableRng;
use rand::seq::IndexedRandom;
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;
use serde::Serialize;

use crate::combinatorics::{GRID_SIZE, binomial, next_combination, unrank};
use crate::error::Result;
use crate::grid::Grid;
use crate::mapping::normalize_selection;

/// Taux de retour aux joueurs par défaut (environ la moitié des mises ; valeur approximative).
pub const DEFAULT_RETURN_RATE: f64 = 0.5;

/// Nombre de tirages possibles des numéros principaux : `C(pool, 5)`.
pub fn draw_count(pool: u8) -> u64 {
    binomial(u64::from(pool), GRID_SIZE as u64) as u64
}

/// Probabilité que exactement `k` des 5 numéros gagnants soient parmi les `n` numéros choisis.
pub fn prob_winners_in_selection(pool: u8, n: usize, k: usize) -> f64 {
    let total = u64::from(pool);
    let (n, k) = (n as u64, k as u64);
    if k > n || k > GRID_SIZE as u64 || n > total {
        return 0.0;
    }
    (binomial(n, k) * binomial(total - n, GRID_SIZE as u64 - k)) as f64 / draw_count(pool) as f64
}

/// Probabilité que les 5 numéros gagnants soient tous parmi les `n` choisis.
pub fn prob_guarantee_applies(pool: u8, n: usize) -> f64 {
    prob_winners_in_selection(pool, n, GRID_SIZE)
}

/// Répartition des tirages selon le meilleur résultat obtenu (0 à 5 bons numéros) avec un jeu de grilles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Distribution {
    pub counts: [u64; GRID_SIZE + 1],
}

impl Distribution {
    pub fn total(&self) -> u64 {
        self.counts.iter().sum()
    }

    pub fn prob(&self, k: usize) -> f64 {
        self.counts[k] as f64 / self.total() as f64
    }

    pub fn prob_at_least(&self, k: usize) -> f64 {
        self.counts[k..].iter().sum::<u64>() as f64 / self.total() as f64
    }
}

/// Pour chacun des `C(pool, 5)` tirages, meilleur nombre de bons numéros parmi les grilles. Calcul exact.
///
/// L'ordre des rangs (colexicographique) énumère d'abord les sous-ensembles des plus petits numéros :
/// les rangs `0..C(pool, 5)` sont exactement les tirages de `{1..pool}`.
pub fn best_match_distribution(pool: u8, grids: &[Grid]) -> Distribution {
    const CHUNK: u64 = 1 << 14;
    let total = draw_count(pool);
    let masks: Vec<u64> = grids.iter().map(|g| g.mask()).collect();
    let counts = (0..total.div_ceil(CHUNK))
        .into_par_iter()
        .map(|c| {
            let mut local = [0u64; GRID_SIZE + 1];
            let start = c * CHUNK;
            let end = (start + CHUNK).min(total);
            let mut d = unrank(start);
            for _ in start..end {
                let mut best = 0;
                for &g in &masks {
                    best = best.max((g & d).count_ones());
                    if best == GRID_SIZE as u32 {
                        break;
                    }
                }
                local[best as usize] += 1;
                d = next_combination(d);
            }
            local
        })
        .reduce(
            || [0u64; GRID_SIZE + 1],
            |mut a, b| {
                for (x, y) in a.iter_mut().zip(b) {
                    *x += y;
                }
                a
            },
        );
    Distribution { counts }
}

/// `count` grilles tirées au hasard parmi les numéros choisis : le point de comparaison honnête
/// (mêmes numéros, même coût, sans système).
pub fn random_grids_within(numbers: &[u8], count: usize, seed: u64) -> Result<Vec<Grid>> {
    let selection = normalize_selection(numbers)?;
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..count)
        .map(|_| {
            let picked: Vec<u8> = selection.sample(&mut rng, GRID_SIZE).copied().collect();
            Grid::new(&picked)
        })
        .collect()
}

/// Perte moyenne attendue pour une mise donnée.
pub fn expected_loss(stake: f64, return_rate: f64) -> f64 {
    stake * (1.0 - return_rate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combinatorics::all_grids;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn draw_counts() {
        assert_eq!(draw_count(50), 2_118_760);
        assert_eq!(draw_count(49), 1_906_884);
    }

    #[test]
    fn hypergeometric_sums_to_one() {
        for pool in [49, 50] {
            for n in [6, 11, 20, 30, usize::from(pool)] {
                let s: f64 = (0..=5).map(|k| prob_winners_in_selection(pool, n, k)).sum();
                assert!(close(s, 1.0), "pool={pool} n={n} somme={s}");
            }
        }
    }

    #[test]
    fn guarantee_probability_values() {
        assert!(close(
            prob_guarantee_applies(50, 20),
            15_504.0 / 2_118_760.0
        ));
        assert!((prob_guarantee_applies(50, 20) - 0.007317).abs() < 1e-6);
        assert!(close(
            prob_guarantee_applies(50, 30),
            142_506.0 / 2_118_760.0
        ));
        assert!((prob_guarantee_applies(50, 30) - 0.067259).abs() < 1e-6);
        assert!(close(
            prob_guarantee_applies(49, 20),
            15_504.0 / 1_906_884.0
        ));
        assert!(close(prob_guarantee_applies(50, 50), 1.0));
        assert_eq!(prob_guarantee_applies(50, 4), 0.0);
    }

    #[test]
    fn distribution_of_one_grid_is_hypergeometric() {
        let g = Grid::new(&[1, 2, 3, 4, 5]).unwrap();
        for pool in [49, 50] {
            let d = best_match_distribution(pool, &[g]);
            assert_eq!(d.total(), draw_count(pool));
            for k in 0..=5 {
                assert!(
                    close(d.prob(k), prob_winners_in_selection(pool, 5, k)),
                    "pool={pool} k={k}"
                );
            }
            assert_eq!(d.counts[5], 1);
        }
    }

    #[test]
    fn distribution_matches_naive_count_on_sample() {
        let grids = vec![
            Grid::new(&[1, 2, 3, 4, 5]).unwrap(),
            Grid::new(&[3, 10, 20, 30, 49]).unwrap(),
        ];
        for pool in [49, 50] {
            let d = best_match_distribution(pool, &grids);
            let mut naive = [0u64; 6];
            for m in all_grids(pool) {
                let best = grids
                    .iter()
                    .map(|g| (g.mask() & m).count_ones())
                    .max()
                    .unwrap();
                naive[best as usize] += 1;
            }
            assert_eq!(d.counts, naive, "pool={pool}");
            assert!(close(d.prob_at_least(0), 1.0));
        }
    }

    #[test]
    fn random_grids_stay_within_selection() {
        let sel: Vec<u8> = (10..22).collect();
        let grids = random_grids_within(&sel, 50, 1).unwrap();
        assert_eq!(grids.len(), 50);
        assert!(
            grids
                .iter()
                .all(|g| g.numbers().iter().all(|n| sel.contains(n)))
        );
        assert_eq!(grids, random_grids_within(&sel, 50, 1).unwrap());
    }

    #[test]
    fn expected_loss_is_proportional() {
        assert!(close(expected_loss(150.0, 0.5), 75.0));
        assert!(close(expected_loss(100.0, 1.0), 0.0));
    }
}
