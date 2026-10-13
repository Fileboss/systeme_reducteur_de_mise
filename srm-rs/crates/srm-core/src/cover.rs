//! Vérification indépendante d'une couverture, par force brute.
//!
//! Volontairement sans lien avec [`crate::space::Space`] ni avec les générateurs :
//! c'est l'oracle qui garantit qu'une table respecte sa promesse.

use rayon::prelude::*;

use crate::combinatorics::all_grids;
use crate::error::{Error, Result};
use crate::grid::Grid;

/// Premier tirage de `{1..n}` qui n'a pas `t` numéros en commun avec une des grilles, s'il existe.
pub fn find_uncovered(n: usize, t: u8, grids: &[Grid]) -> Option<Grid> {
    let draws: Vec<u64> = all_grids(n as u8).collect();
    let masks: Vec<u64> = grids.iter().map(|g| g.mask()).collect();
    let t = u32::from(t);
    draws
        .par_iter()
        .find_first(|&&d| !masks.iter().any(|&g| (g & d).count_ones() >= t))
        .map(|&d| Grid::from_mask_unchecked(d))
}

/// Vérifie que chaque tirage de `{1..n}` partage au moins `t` numéros avec une grille,
/// et que toutes les grilles n'utilisent que des numéros de `{1..n}`.
pub fn verify(n: usize, t: u8, grids: &[Grid]) -> Result<()> {
    if let Some(g) = grids.iter().find(|g| g.numbers()[4] as usize > n) {
        return Err(Error::InvalidGrid(format!("{g} sort de 1..{n}")));
    }
    match find_uncovered(n, t, grids) {
        Some(d) => Err(Error::NotCovered(d.to_string())),
        None => Ok(()),
    }
}

/// Fraction des tirages de `{1..n}` couverts (utile pour diagnostiquer une table invalide).
pub fn coverage_ratio(n: usize, t: u8, grids: &[Grid]) -> f64 {
    let draws: Vec<u64> = all_grids(n as u8).collect();
    let t = u32::from(t);
    let covered = draws
        .par_iter()
        .filter(|&&d| grids.iter().any(|g| (g.mask() & d).count_ones() >= t))
        .count();
    covered as f64 / draws.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn g(v: [u8; 5]) -> Grid {
        Grid::new(&v).unwrap()
    }

    #[test]
    fn all_grids_cover_everything() {
        let all: Vec<Grid> = all_grids(7).map(|m| Grid::from_mask(m).unwrap()).collect();
        assert!(verify(7, 4, &all).is_ok());
        assert_eq!(coverage_ratio(7, 4, &all), 1.0);
    }

    #[test]
    fn detects_missing_draw() {
        // Pour n = 6 et t = 5, il faut toutes les grilles ; en retirer une laisse un trou.
        let mut all: Vec<Grid> = all_grids(6).map(|m| Grid::from_mask(m).unwrap()).collect();
        let removed = all.pop().unwrap();
        assert_eq!(find_uncovered(6, 5, &all), Some(removed));
        assert_eq!(
            verify(6, 5, &all),
            Err(Error::NotCovered(removed.to_string()))
        );
    }

    #[test]
    fn single_grid_covers_n6_t2() {
        // Deux 5-sous-ensembles de {1..6} partagent toujours au moins 4 numéros.
        assert!(verify(6, 4, &[g([1, 2, 3, 4, 5])]).is_ok());
        assert!(verify(7, 4, &[g([1, 2, 3, 4, 5])]).is_err());
    }

    #[test]
    fn rejects_grid_outside_range() {
        assert!(matches!(
            verify(10, 2, &[g([1, 2, 3, 4, 11])]),
            Err(Error::InvalidGrid(_))
        ));
    }
}
