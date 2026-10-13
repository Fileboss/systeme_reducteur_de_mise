//! Rapport complet pour une sélection : grilles à jouer, coût, probabilités, comparaison au hasard.
//! Partagé par la CLI, la TUI et l'interface web.

use serde::Serialize;

use crate::combinatorics::GRID_SIZE;
use crate::error::Result;
use crate::grid::Grid;
use crate::mapping::{normalize_selection, reduce};
use crate::odds::{
    Distribution, best_match_distribution, expected_loss, prob_guarantee_applies,
    prob_winners_in_selection, random_grids_within,
};

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Settings {
    pub grid_price: f64,
    pub return_rate: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            grid_price: crate::pricing::DEFAULT_GRID_PRICE,
            return_rate: crate::odds::DEFAULT_RETURN_RATE,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Report {
    pub numbers: Vec<u8>,
    pub t: u8,
    pub grids: Vec<Grid>,
    pub cost: f64,
    /// Probabilité que les 5 numéros gagnants soient tous parmi les numéros choisis.
    pub prob_guarantee: f64,
    /// `prob_winners[k]` : probabilité que exactement `k` gagnants soient parmi les numéros choisis.
    pub prob_winners: [f64; GRID_SIZE + 1],
    pub expected_loss: f64,
    /// Nombre de fois où chaque numéro choisi apparaît dans les grilles (même ordre que `numbers`).
    pub usage: Vec<usize>,
}

/// Distribution du meilleur résultat : le système contre autant de grilles prises au hasard
/// parmi les mêmes numéros.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Comparison {
    pub system: Distribution,
    pub random: Distribution,
}

pub fn build(numbers: &[u8], t: u8, settings: Settings) -> Result<Report> {
    let numbers = normalize_selection(numbers)?;
    let grids = reduce(&numbers, t)?;
    let cost = grids.len() as f64 * settings.grid_price;
    let usage = numbers
        .iter()
        .map(|&x| grids.iter().filter(|g| g.contains(x)).count())
        .collect();
    let n = numbers.len();
    Ok(Report {
        t,
        cost,
        prob_guarantee: prob_guarantee_applies(n),
        prob_winners: std::array::from_fn(|k| prob_winners_in_selection(n, k)),
        expected_loss: expected_loss(cost, settings.return_rate),
        usage,
        grids,
        numbers,
    })
}

pub fn compare(report: &Report, seed: u64) -> Result<Comparison> {
    let random = random_grids_within(&report.numbers, report.grids.len(), seed)?;
    Ok(Comparison {
        system: best_match_distribution(&report.grids),
        random: best_match_distribution(&random),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;

    #[test]
    fn report_is_consistent() {
        let numbers: Vec<u8> = vec![49, 3, 12, 7, 33, 21, 40, 1, 2, 4, 5, 6];
        let r = build(&numbers, 3, Settings::default()).unwrap();
        assert_eq!(r.numbers.len(), 12);
        assert!(r.numbers.windows(2).all(|w| w[0] < w[1]));
        assert_eq!(r.cost, r.grids.len() as f64 * 2.5);
        assert_eq!(r.expected_loss, r.cost * 0.5);
        assert_eq!(r.usage.iter().sum::<usize>(), r.grids.len() * 5);
        assert!((r.prob_winners.iter().sum::<f64>() - 1.0).abs() < 1e-9);
        assert_eq!(r.prob_winners[5], r.prob_guarantee);
    }

    #[test]
    fn comparison_totals_and_guarantee() {
        let numbers: Vec<u8> = (1..=12).collect();
        let r = build(&numbers, 3, Settings::default()).unwrap();
        let c = compare(&r, 1).unwrap();
        assert_eq!(c.system.total(), crate::odds::DRAW_COUNT);
        assert_eq!(c.random.total(), crate::odds::DRAW_COUNT);
        // Les C(12,5) = 792 tirages inclus dans la sélection ont tous au moins 3 bons numéros.
        let inside = crate::combinatorics::all_grids(12)
            .map(Grid::from_mask_unchecked)
            .filter(|d| r.grids.iter().any(|g| g.common(*d) >= 3))
            .count();
        assert_eq!(inside, 792);
    }

    #[test]
    fn missing_table_is_an_error() {
        let numbers: Vec<u8> = (1..=31).collect();
        assert!(matches!(
            build(&numbers, 3, Settings::default()),
            Err(Error::UnsupportedParams { .. })
        ));
    }
}
