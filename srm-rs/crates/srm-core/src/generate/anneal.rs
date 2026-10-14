//! Recuit simulé pour réduire une couverture, dans l'esprit de Nurmela et Östergård (1993).
//!
//! On fixe un nombre de grilles `b` et on minimise le nombre de tirages non couverts.
//! Dès qu'il tombe à zéro, la solution est valide : on la garde, on retire la grille la moins
//! utile (`b − 1`) et on recommence, jusqu'à épuisement du budget.

use std::time::Instant;

use rand::{Rng, RngExt};

use crate::space::Space;

/// Réglages du recuit.
#[derive(Debug, Clone, Copy)]
pub struct AnnealParams {
    /// Température initiale (et de réchauffe).
    pub t0: f64,
    /// Facteur de refroidissement appliqué tous les `moves_per_step` mouvements.
    pub cooling: f64,
    pub moves_per_step: u32,
    /// En dessous de cette température, on réchauffe à `t0`.
    pub t_min: f64,
}

impl Default for AnnealParams {
    fn default() -> Self {
        Self {
            t0: 0.6,
            cooling: 0.995,
            moves_per_step: 200,
            t_min: 0.05,
        }
    }
}

/// Critère d'arrêt : un nombre de mouvements (reproductible) et/ou une échéance.
#[derive(Debug, Clone, Copy)]
pub struct Budget {
    pub max_moves: u64,
    pub deadline: Option<Instant>,
}

const NONE: u32 = u32::MAX;

struct State<'a> {
    space: &'a Space,
    sol: Vec<u64>,
    /// Nombre de grilles de `sol` qui couvrent chaque tirage.
    count: Vec<u16>,
    uncovered: Vec<u32>,
    uncovered_pos: Vec<u32>,
}

impl<'a> State<'a> {
    fn new(space: &'a Space, sol: Vec<u64>) -> Self {
        let mut s = Self {
            space,
            sol: Vec::with_capacity(sol.len()),
            count: vec![0; space.len()],
            uncovered: (0..space.len() as u32).collect(),
            uncovered_pos: (0..space.len() as u32).collect(),
        };
        for g in sol {
            s.add(g);
        }
        s
    }

    fn add(&mut self, g: u64) {
        let Self {
            space,
            count,
            uncovered,
            uncovered_pos,
            ..
        } = self;
        space.for_each_neighbor(g, |d, _| {
            let d = d as usize;
            if count[d] == 0 {
                let p = uncovered_pos[d] as usize;
                let last = uncovered.pop().unwrap();
                if last as usize != d {
                    uncovered[p] = last;
                    uncovered_pos[last as usize] = p as u32;
                }
                uncovered_pos[d] = NONE;
            }
            count[d] += 1;
        });
        self.sol.push(g);
    }

    fn remove_at(&mut self, i: usize) -> u64 {
        let g = self.sol.swap_remove(i);
        let Self {
            space,
            count,
            uncovered,
            uncovered_pos,
            ..
        } = self;
        space.for_each_neighbor(g, |d, _| {
            let d = d as usize;
            count[d] -= 1;
            if count[d] == 0 {
                uncovered_pos[d] = uncovered.len() as u32;
                uncovered.push(d as u32);
            }
        });
        g
    }

    /// Variation du nombre de tirages non couverts si l'on remplace `old` par `new`.
    fn delta(&self, old: u64, new: u64) -> i64 {
        let mut lost = 0i64;
        self.space.for_each_neighbor(old, |d, m| {
            if self.count[d as usize] == 1 && !self.space.covers(m, new) {
                lost += 1;
            }
        });
        let mut gained = 0i64;
        self.space.for_each_neighbor(new, |d, _| {
            if self.count[d as usize] == 0 {
                gained += 1;
            }
        });
        lost - gained
    }

    /// Nombre de tirages qui ne seraient plus couverts si l'on retirait la grille `i`.
    fn loss_if_removed(&self, i: usize) -> usize {
        let mut lost = 0;
        self.space.for_each_neighbor(self.sol[i], |d, _| {
            if self.count[d as usize] == 1 {
                lost += 1;
            }
        });
        lost
    }

    fn drop_least_useful(&mut self) {
        let i = (0..self.sol.len())
            .min_by_key(|&i| self.loss_if_removed(i))
            .unwrap();
        self.remove_at(i);
    }
}

/// Choisit un bit au hasard dans `mask` (non vide).
fn random_bit<R: Rng>(mask: u64, rng: &mut R) -> u64 {
    let mut k = rng.random_range(0..mask.count_ones());
    let mut m = mask;
    while k > 0 {
        m &= m - 1;
        k -= 1;
    }
    m.isolate_lowest_one()
}

/// Part d'une couverture valide et renvoie la plus petite couverture valide trouvée.
pub fn anneal<R: Rng>(
    space: &Space,
    initial: Vec<u64>,
    params: AnnealParams,
    budget: Budget,
    rng: &mut R,
) -> Vec<u64> {
    let mut best = initial.clone();
    if initial.len() <= 1 {
        return best;
    }
    let mut state = State::new(space, initial);
    debug_assert!(
        state.uncovered.is_empty(),
        "la solution de départ doit être valide"
    );
    state.drop_least_useful();

    let t_need = space.t() as u32;
    let mut temp = params.t0;
    let mut moves = 0u64;
    let mut since_cool = 0u32;

    while moves < budget.max_moves {
        if state.uncovered.is_empty() {
            best.clone_from(&state.sol);
            if state.sol.len() <= 1 {
                break;
            }
            state.drop_least_useful();
            temp = params.t0;
            continue;
        }
        if moves.is_multiple_of(1024) && budget.deadline.is_some_and(|d| Instant::now() >= d) {
            break;
        }
        moves += 1;

        // Mouvement : prendre un tirage non couvert, et la grille qui en est la plus proche
        // (au hasard parmi les ex æquo) ; échanger un de ses numéros pour s'en rapprocher.
        let d = space.mask(state.uncovered[rng.random_range(0..state.uncovered.len())]);
        let mut best_overlap = 0;
        let mut pick = 0;
        let mut ties = 0u32;
        for (i, &g) in state.sol.iter().enumerate() {
            let o = (g & d).count_ones();
            if o > best_overlap {
                best_overlap = o;
                pick = i;
                ties = 1;
            } else if o == best_overlap {
                ties += 1;
                if rng.random_range(0..ties) == 0 {
                    pick = i;
                }
            }
        }
        debug_assert!(best_overlap < t_need);
        let old = state.sol[pick];
        let new = old ^ random_bit(old & !d, rng) ^ random_bit(d & !old, rng);
        if state.sol.contains(&new) {
            continue;
        }

        let delta = state.delta(old, new);
        if delta <= 0 || rng.random::<f64>() < (-(delta as f64) / temp).exp() {
            state.remove_at(pick);
            state.add(new);
        }

        since_cool += 1;
        if since_cool >= params.moves_per_step {
            since_cool = 0;
            temp *= params.cooling;
            if temp < params.t_min {
                temp = params.t0;
            }
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::*;
    use crate::cover::verify;
    use crate::generate::greedy::greedy;
    use crate::grid::Grid;

    fn run(n: usize, t: u8, moves: u64, seed: u64) -> (usize, Vec<u64>) {
        let space = Space::new(n, t).unwrap();
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let start = greedy(&space, &mut rng);
        let budget = Budget {
            max_moves: moves,
            deadline: None,
        };
        (
            start.len(),
            anneal(&space, start, AnnealParams::default(), budget, &mut rng),
        )
    }

    fn to_grids(masks: &[u64]) -> Vec<Grid> {
        masks.iter().map(|&m| Grid::from_mask(m).unwrap()).collect()
    }

    #[test]
    fn anneal_output_is_valid_and_not_larger() {
        for (n, t) in [(9, 2), (11, 3), (12, 4), (14, 3)] {
            let (start, sol) = run(n, t, 50_000, 3);
            verify(n, t, &to_grids(&sol)).unwrap_or_else(|e| panic!("n={n} t={t}: {e}"));
            assert!(sol.len() <= start);
        }
    }

    #[test]
    fn anneal_is_reproducible_with_move_budget() {
        assert_eq!(run(12, 3, 20_000, 9).1, run(12, 3, 20_000, 9).1);
    }

    #[test]
    fn random_bit_picks_a_bit_of_the_mask() {
        let mut rng = ChaCha8Rng::seed_from_u64(0);
        for _ in 0..100 {
            let b = random_bit(0b1010_0110, &mut rng);
            assert_eq!(b.count_ones(), 1);
            assert_ne!(b & 0b1010_0110, 0);
        }
    }
}
