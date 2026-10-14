//! Numéros complémentaires : étoiles (EuroMillions) ou numéro chance (Loto).
//!
//! Deux modes :
//! - **fixe** : le joueur choisit exactement les compléments d'une grille (2 étoiles, 1 chance),
//!   joués sur toutes les grilles, sans surcoût ;
//! - **réduit** : le joueur choisit `m` compléments et une garantie `s` ; on joue un ensemble minimal
//!   de combinaisons tel que, si les compléments gagnants sont parmi les `m`, une combinaison en
//!   contienne au moins `s`. Chaque grille de numéros est jouée avec chaque combinaison.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::combinatorics::binomial;
use crate::error::{Error, Result};
use crate::game::Game;

/// Combinaison de compléments d'une grille (bit `i` ⇔ complément `i + 1`).
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExtraSet(u16);

impl ExtraSet {
    pub fn from_numbers(numbers: &[u8]) -> Self {
        Self(numbers.iter().fold(0, |m, &n| m | 1 << (n - 1)))
    }

    pub fn numbers(self) -> Vec<u8> {
        (0..16u8)
            .filter(|i| self.0 & (1 << i) != 0)
            .map(|i| i + 1)
            .collect()
    }

    pub fn common(self, other: ExtraSet) -> u32 {
        (self.0 & other.0).count_ones()
    }

    pub fn len(self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl fmt::Display for ExtraSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: Vec<String> = self.numbers().iter().map(|n| format!("{n:02}")).collect();
        f.write_str(&s.join(" "))
    }
}

impl fmt::Debug for ExtraSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ExtraSet({self})")
    }
}

impl Serialize for ExtraSet {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        self.numbers().serialize(s)
    }
}

/// Mode de jeu des compléments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "mode")]
pub enum ExtraMode {
    /// Exactement les compléments d'une grille, joués partout.
    Fixed,
    /// Couverture des compléments choisis avec une garantie `t` (0 à compléments par grille).
    Reduced { t: u8 },
}

impl ExtraMode {
    /// Garantie effective sur les compléments : en mode fixe, tous ceux de la grille.
    pub fn guarantee(self, game: Game) -> u8 {
        match self {
            ExtraMode::Fixed => game.extras_per_grid(),
            ExtraMode::Reduced { t } => t,
        }
    }
}

/// Vérifie une sélection de compléments et la renvoie triée.
pub fn normalize_extras(game: Game, extras: &[u8]) -> Result<Vec<u8>> {
    let mut sorted = extras.to_vec();
    sorted.sort_unstable();
    if let Some(w) = sorted.windows(2).find(|w| w[0] == w[1]) {
        return Err(Error::Extras(format!(
            "{} en double : {}",
            game.extra_name(),
            w[0]
        )));
    }
    if let Some(&bad) = sorted.iter().find(|&&x| x == 0 || x > game.extra_pool()) {
        return Err(Error::OutOfPool {
            what: game.extra_name(),
            value: i64::from(bad),
            max: game.extra_pool(),
        });
    }
    Ok(sorted)
}

/// Combinaisons de compléments à jouer pour cette sélection et ce mode.
pub fn cover_extras(game: Game, extras: &[u8], mode: ExtraMode) -> Result<Vec<ExtraSet>> {
    let sel = normalize_extras(game, extras)?;
    let per = game.extras_per_grid();
    let m = sel.len();
    match mode {
        ExtraMode::Fixed => {
            if m != usize::from(per) {
                return Err(Error::Extras(format!(
                    "mode fixe : choisissez exactement {per} {} ({m} choisi{}) ; activez « réduire les compléments » pour en jouer davantage",
                    if per > 1 {
                        game.extra_plural()
                    } else {
                        game.extra_name()
                    },
                    if m > 1 { "s" } else { "" }
                )));
            }
            Ok(vec![ExtraSet::from_numbers(&sel)])
        }
        ExtraMode::Reduced { t } => {
            if t > per {
                return Err(Error::Extras(format!(
                    "garantie sur les {} invalide : {t} (entre 0 et {per})",
                    game.extra_plural()
                )));
            }
            if m < usize::from(per) {
                return Err(Error::Extras(format!(
                    "choisissez au moins {per} {}",
                    if per > 1 {
                        game.extra_plural()
                    } else {
                        game.extra_name()
                    }
                )));
            }
            Ok(match (per, t) {
                // Aucune garantie : une seule combinaison.
                (_, 0) => vec![ExtraSet::from_numbers(&sel[..usize::from(per)])],
                // Un complément par grille : jouer chacun.
                (1, 1) => sel.iter().map(|&x| ExtraSet::from_numbers(&[x])).collect(),
                // Deux par grille, garantie 2 : toutes les paires.
                (2, 2) => {
                    let mut out = Vec::new();
                    for i in 0..m {
                        for j in i + 1..m {
                            out.push(ExtraSet::from_numbers(&[sel[i], sel[j]]));
                        }
                    }
                    out
                }
                // Deux par grille, garantie 1 : un appariement. Toute paire gagnante a au moins un
                // élément apparié, puisqu'au plus un complément reste seul. ⌊m/2⌋ paires, optimal :
                // il faut toucher au moins m − 1 compléments et chaque paire en touche deux.
                (2, 1) => sel
                    .as_chunks::<2>()
                    .0
                    .iter()
                    .map(|p| ExtraSet::from_numbers(p))
                    .collect(),
                _ => unreachable!("jeux à 1 ou 2 compléments par grille"),
            })
        }
    }
}

/// Nombre de combinaisons de compléments jouées (multiplicateur du nombre de grilles).
pub fn combo_count(game: Game, m: usize, mode: ExtraMode) -> Result<usize> {
    let numbers: Vec<u8> = (1..=m as u8).collect();
    Ok(cover_extras(game, &numbers, mode)?.len())
}

/// Vérifie par force brute que chaque tirage de compléments inclus dans la sélection a au moins
/// `t` compléments en commun avec une des combinaisons.
pub fn verify_extras(game: Game, selection: &[u8], t: u8, sets: &[ExtraSet]) -> bool {
    let per = usize::from(game.extras_per_grid());
    let mut ok = true;
    for_each_subset(selection, per, &mut |draw| {
        let d = ExtraSet::from_numbers(draw);
        if !sets.iter().any(|s| s.common(d) >= u32::from(t)) {
            ok = false;
        }
    });
    ok
}

/// Probabilité que les compléments gagnants soient tous parmi les `m` choisis.
pub fn prob_extras_in_selection(game: Game, m: usize) -> f64 {
    let per = u64::from(game.extras_per_grid());
    binomial(m as u64, per) as f64 / binomial(u64::from(game.extra_pool()), per) as f64
}

fn for_each_subset(items: &[u8], k: usize, f: &mut impl FnMut(&[u8])) {
    fn rec(items: &[u8], k: usize, start: usize, cur: &mut Vec<u8>, f: &mut impl FnMut(&[u8])) {
        if cur.len() == k {
            f(cur);
            return;
        }
        for i in start..items.len() {
            cur.push(items[i]);
            rec(items, k, i + 1, cur, f);
            cur.pop();
        }
    }
    rec(items, k, 0, &mut Vec::with_capacity(k), f);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_reduced_construction_is_valid() {
        for game in Game::ALL {
            let per = game.extras_per_grid();
            for m in usize::from(per)..=usize::from(game.extra_pool()) {
                let sel: Vec<u8> = (1..=m as u8).collect();
                for t in 0..=per {
                    let sets = cover_extras(game, &sel, ExtraMode::Reduced { t }).unwrap();
                    assert!(verify_extras(game, &sel, t, &sets), "{game} m={m} t={t}");
                    assert!(sets.iter().all(|s| s.len() == usize::from(per)));
                }
            }
        }
    }

    #[test]
    fn sizes_match_formulas() {
        for m in 2..=12usize {
            let em = |t| combo_count(Game::EuroMillions, m, ExtraMode::Reduced { t }).unwrap();
            assert_eq!(em(0), 1);
            assert_eq!(em(1), m / 2);
            assert_eq!(em(2), m * (m - 1) / 2);
        }
        for m in 1..=10usize {
            let lo = |t| combo_count(Game::Loto, m, ExtraMode::Reduced { t }).unwrap();
            assert_eq!(lo(0), 1);
            assert_eq!(lo(1), m);
        }
    }

    /// L'appariement est minimal : aucun ensemble plus petit de paires ne garantit 1 étoile.
    #[test]
    fn matching_is_minimal_for_small_m() {
        for m in 2..=6u8 {
            let sel: Vec<u8> = (1..=m).collect();
            let mut pairs = Vec::new();
            for_each_subset(&sel, 2, &mut |p| pairs.push(ExtraSet::from_numbers(p)));
            let target = usize::from(m / 2);
            // Toutes les familles de target − 1 paires échouent.
            let mut chosen = Vec::new();
            let mut any_valid = false;
            fn search(
                pairs: &[ExtraSet],
                k: usize,
                start: usize,
                chosen: &mut Vec<ExtraSet>,
                sel: &[u8],
                found: &mut bool,
            ) {
                if chosen.len() == k {
                    if verify_extras(Game::EuroMillions, sel, 1, chosen) {
                        *found = true;
                    }
                    return;
                }
                for i in start..pairs.len() {
                    chosen.push(pairs[i]);
                    search(pairs, k, i + 1, chosen, sel, found);
                    chosen.pop();
                }
            }
            search(&pairs, target - 1, 0, &mut chosen, &sel, &mut any_valid);
            assert!(
                !any_valid,
                "m={m} : une solution à {} paires existe",
                target - 1
            );
        }
    }

    #[test]
    fn fixed_mode_requires_exact_count() {
        let one = cover_extras(Game::EuroMillions, &[7, 3], ExtraMode::Fixed).unwrap();
        assert_eq!(one.len(), 1);
        assert_eq!(one[0].to_string(), "03 07");
        assert_eq!(
            cover_extras(Game::Loto, &[4], ExtraMode::Fixed)
                .unwrap()
                .len(),
            1
        );

        let err = cover_extras(Game::EuroMillions, &[1, 2, 3], ExtraMode::Fixed).unwrap_err();
        assert!(err.to_string().contains("exactement 2 étoiles"), "{err}");
        assert!(err.to_string().contains("réduire"));
        assert!(cover_extras(Game::Loto, &[1, 2], ExtraMode::Fixed).is_err());
    }

    #[test]
    fn invalid_extras_are_rejected() {
        let err = normalize_extras(Game::EuroMillions, &[13]).unwrap_err();
        assert_eq!(
            err.to_string(),
            "étoile invalide : 13 (doit être entre 1 et 12)"
        );
        assert!(normalize_extras(Game::Loto, &[11]).is_err());
        assert!(normalize_extras(Game::Loto, &[3, 3]).is_err());
        assert!(cover_extras(Game::EuroMillions, &[1, 2, 3], ExtraMode::Reduced { t: 3 }).is_err());
        assert!(cover_extras(Game::EuroMillions, &[1], ExtraMode::Reduced { t: 1 }).is_err());
    }

    #[test]
    fn extras_probability() {
        // 4 étoiles : C(4,2) / C(12,2) = 6 / 66.
        assert!((prob_extras_in_selection(Game::EuroMillions, 4) - 6.0 / 66.0).abs() < 1e-12);
        assert!((prob_extras_in_selection(Game::Loto, 3) - 0.3).abs() < 1e-12);
        assert_eq!(prob_extras_in_selection(Game::EuroMillions, 12), 1.0);
    }

    #[test]
    fn mode_guarantee() {
        assert_eq!(ExtraMode::Fixed.guarantee(Game::EuroMillions), 2);
        assert_eq!(ExtraMode::Fixed.guarantee(Game::Loto), 1);
        assert_eq!(ExtraMode::Reduced { t: 1 }.guarantee(Game::EuroMillions), 1);
    }
}
