//! Rapport complet pour une sélection : grilles à jouer, compléments, coût, probabilités,
//! comparaison au hasard. Partagé par la CLI, la TUI et l'interface web.

use serde::Serialize;

use crate::combinatorics::GRID_SIZE;
use crate::error::Result;
use crate::extras::{
    ExtraMode, ExtraSet, cover_extras, normalize_extras, prob_extras_in_selection,
};
use crate::game::Game;
use crate::grid::Grid;
use crate::mapping::{normalize_for_game, reduce};
use crate::odds::{
    Distribution, best_match_distribution, expected_loss, prob_guarantee_applies,
    prob_winners_in_selection, random_grids_within,
};

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Settings {
    pub game: Game,
    pub grid_price: f64,
    pub return_rate: f64,
}

impl Settings {
    /// Réglages par défaut d'un jeu (prix de grille du jeu).
    pub fn for_game(game: Game) -> Self {
        Self {
            game,
            grid_price: game.default_price(),
            return_rate: crate::odds::DEFAULT_RETURN_RATE,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::for_game(Game::default())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Report {
    pub game: Game,
    pub numbers: Vec<u8>,
    pub t: u8,
    /// Grilles de numéros (sans compléments).
    pub grids: Vec<Grid>,
    /// Compléments choisis (vide : compléments au choix du joueur, non pris en compte).
    pub extras: Vec<u8>,
    pub extra_mode: Option<ExtraMode>,
    /// Garantie effective sur les compléments (toutes celles d'une grille en mode fixe).
    pub extra_t: Option<u8>,
    /// Combinaisons de compléments ; chaque grille est jouée avec chacune.
    pub extra_sets: Vec<ExtraSet>,
    /// Nombre de tickets : grilles × combinaisons de compléments.
    pub tickets: usize,
    pub cost: f64,
    /// Probabilité que les 5 numéros gagnants soient tous parmi les numéros choisis.
    pub prob_numbers: f64,
    /// Probabilité que les compléments gagnants soient tous parmi ceux choisis.
    pub prob_extras: Option<f64>,
    /// Probabilité que la garantie complète s'applique (produit des deux, tirages indépendants).
    pub prob_guarantee: f64,
    /// `prob_winners[k]` : probabilité que exactement `k` gagnants soient parmi les numéros choisis.
    pub prob_winners: [f64; GRID_SIZE + 1],
    pub expected_loss: f64,
    /// Nombre de fois où chaque numéro choisi apparaît dans les grilles (même ordre que `numbers`).
    pub usage: Vec<usize>,
}

impl Report {
    /// Tous les tickets à jouer : chaque grille avec chaque combinaison de compléments.
    pub fn tickets(&self) -> impl Iterator<Item = (Grid, Option<ExtraSet>)> + '_ {
        self.grids.iter().flat_map(move |&g| {
            let sets: Vec<Option<ExtraSet>> = if self.extra_sets.is_empty() {
                vec![None]
            } else {
                self.extra_sets.iter().copied().map(Some).collect()
            };
            sets.into_iter().map(move |e| (g, e))
        })
    }
}

/// Distribution du meilleur résultat sur les numéros : le système contre autant de grilles prises
/// au hasard parmi les mêmes numéros.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Comparison {
    pub system: Distribution,
    pub random: Distribution,
}

/// Construit le rapport. `extras` vide : compléments non pris en compte (`mode` ignoré).
pub fn build(
    numbers: &[u8],
    t: u8,
    extras: &[u8],
    mode: ExtraMode,
    settings: Settings,
) -> Result<Report> {
    let game = settings.game;
    let numbers = normalize_for_game(game, numbers)?;
    let grids = reduce(&numbers, t)?;
    let (extras, extra_mode, extra_sets) = if extras.is_empty() {
        (Vec::new(), None, Vec::new())
    } else {
        let sets = cover_extras(game, extras, mode)?;
        (normalize_extras(game, extras)?, Some(mode), sets)
    };
    let tickets = grids.len() * extra_sets.len().max(1);
    let cost = tickets as f64 * settings.grid_price;
    let usage = numbers
        .iter()
        .map(|&x| grids.iter().filter(|g| g.contains(x)).count())
        .collect();
    let n = numbers.len();
    let pool = game.pool();
    let prob_numbers = prob_guarantee_applies(pool, n);
    let prob_extras = extra_mode.map(|_| prob_extras_in_selection(game, extras.len()));
    Ok(Report {
        game,
        t,
        extra_t: extra_mode.map(|m| m.guarantee(game)),
        extra_mode,
        extra_sets,
        extras,
        tickets,
        cost,
        prob_numbers,
        prob_extras,
        prob_guarantee: prob_numbers * prob_extras.unwrap_or(1.0),
        prob_winners: std::array::from_fn(|k| prob_winners_in_selection(pool, n, k)),
        expected_loss: expected_loss(cost, settings.return_rate),
        usage,
        grids,
        numbers,
    })
}

pub fn compare(report: &Report, seed: u64) -> Result<Comparison> {
    let random = random_grids_within(&report.numbers, report.grids.len(), seed)?;
    let pool = report.game.pool();
    Ok(Comparison {
        system: best_match_distribution(pool, &report.grids),
        random: best_match_distribution(pool, &random),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use crate::odds::draw_count;

    fn none() -> (&'static [u8], ExtraMode) {
        (&[], ExtraMode::Fixed)
    }

    #[test]
    fn report_is_consistent() {
        let numbers: Vec<u8> = vec![49, 3, 12, 7, 33, 21, 40, 1, 2, 4, 5, 6];
        let (e, m) = none();
        let r = build(&numbers, 3, e, m, Settings::default()).unwrap();
        assert_eq!(r.numbers.len(), 12);
        assert!(r.numbers.windows(2).all(|w| w[0] < w[1]));
        assert_eq!(r.tickets, r.grids.len());
        assert_eq!(r.cost, r.grids.len() as f64 * 2.5);
        assert_eq!(r.expected_loss, r.cost * 0.5);
        assert_eq!(r.usage.iter().sum::<usize>(), r.grids.len() * 5);
        assert!((r.prob_winners.iter().sum::<f64>() - 1.0).abs() < 1e-9);
        assert_eq!(r.prob_winners[5], r.prob_guarantee);
        assert_eq!(r.extra_mode, None);
        assert_eq!(r.tickets().count(), r.grids.len());
    }

    #[test]
    fn fixed_extras_cost_nothing_more() {
        let numbers: Vec<u8> = (1..=12).collect();
        let r = build(&numbers, 3, &[9, 2], ExtraMode::Fixed, Settings::default()).unwrap();
        assert_eq!(r.tickets, r.grids.len());
        assert_eq!(r.extra_t, Some(2));
        assert!(r.tickets().all(|(_, e)| e.unwrap().to_string() == "02 09"));
        // Les 2 étoiles gagnantes sont les vôtres : 1 chance sur C(12,2) = 66.
        assert!((r.prob_guarantee - r.prob_numbers / 66.0).abs() < 1e-15);
    }

    #[test]
    fn reduced_extras_multiply_tickets() {
        let numbers: Vec<u8> = (1..=12).collect();
        let s = Settings::default();
        let base = build(&numbers, 3, &[], ExtraMode::Fixed, s).unwrap();
        let r = build(&numbers, 3, &[1, 4, 7, 9], ExtraMode::Reduced { t: 1 }, s).unwrap();
        assert_eq!(r.extra_sets.len(), 2);
        assert_eq!(r.tickets, base.grids.len() * 2);
        assert_eq!(r.tickets().count(), r.tickets);
        assert_eq!(r.cost, base.cost * 2.0);
        assert_eq!(r.prob_extras, Some(6.0 / 66.0));
    }

    #[test]
    fn loto_uses_49_numbers_and_its_price() {
        let s = Settings::for_game(Game::Loto);
        let numbers: Vec<u8> = (38..=49).collect();
        let r = build(&numbers, 3, &[1, 4, 7], ExtraMode::Reduced { t: 1 }, s).unwrap();
        assert_eq!(r.tickets, r.grids.len() * 3);
        assert!((r.cost - r.tickets as f64 * 2.2).abs() < 1e-9);
        let with_50: Vec<u8> = (39..=50).collect();
        assert!(matches!(
            build(&with_50, 3, &[], ExtraMode::Fixed, s),
            Err(Error::OutOfPool { value: 50, .. })
        ));
        let c = compare(&r, 1).unwrap();
        assert_eq!(c.system.total(), draw_count(49));
    }

    #[test]
    fn comparison_totals_and_guarantee() {
        let numbers: Vec<u8> = (1..=12).collect();
        let (e, m) = none();
        let r = build(&numbers, 3, e, m, Settings::default()).unwrap();
        let c = compare(&r, 1).unwrap();
        assert_eq!(c.system.total(), draw_count(50));
        assert_eq!(c.random.total(), draw_count(50));
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
        let (e, m) = none();
        assert!(matches!(
            build(&numbers, 3, e, m, Settings::default()),
            Err(Error::UnsupportedParams { .. })
        ));
    }
}
