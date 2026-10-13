//! Correspondance entre une table calculée sur `{1..n}` et les `n` numéros choisis par le joueur.
//!
//! Le problème est invariant par permutation des numéros : remplacer `i` par le `i`-ème numéro
//! choisi (trié) transforme une couverture de `{1..n}` en couverture des numéros choisis.

use crate::combinatorics::MAX_NUMBER;
use crate::error::{Error, Result};
use crate::grid::Grid;
use crate::table::Table;

/// Vérifie une sélection de numéros et la renvoie triée.
pub fn normalize_selection(numbers: &[u8]) -> Result<Vec<u8>> {
    let mut sorted = numbers.to_vec();
    sorted.sort_unstable();
    for w in sorted.windows(2) {
        if w[0] == w[1] {
            return Err(Error::DuplicateNumber(w[0]));
        }
    }
    if let Some(&bad) = sorted.iter().find(|&&x| x == 0 || x > MAX_NUMBER) {
        return Err(Error::InvalidNumber(i64::from(bad)));
    }
    Ok(sorted)
}

/// Applique la table aux numéros choisis. La sélection de l'appelant n'est pas modifiée.
pub fn map_table(table: &Table, numbers: &[u8]) -> Result<Vec<Grid>> {
    let selection = normalize_selection(numbers)?;
    if selection.len() != table.n() {
        return Err(Error::NoTable {
            n: selection.len(),
            t: table.t(),
        });
    }
    table
        .grids()
        .iter()
        .map(|g| {
            let mapped = g.numbers().map(|i| selection[usize::from(i) - 1]);
            Grid::new(&mapped)
        })
        .collect()
}

/// Grilles à jouer pour ces numéros et cette garantie, à partir des tables embarquées.
pub fn reduce(numbers: &[u8], t: u8) -> Result<Vec<Grid>> {
    let selection = normalize_selection(numbers)?;
    map_table(Table::embedded(selection.len(), t)?, &selection)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mapping_replaces_indices_and_keeps_order() {
        let table = Table::parse(7, 3, "01,02,03,04,05\n03,04,05,06,07\n").unwrap();
        let picks = [49, 3, 12, 7, 33, 21, 40];
        let grids = map_table(&table, &picks).unwrap();
        // Sélection triée : 3 7 12 21 33 40 49.
        assert_eq!(grids[0].to_string(), "03,07,12,21,33");
        assert_eq!(grids[1].to_string(), "12,21,33,40,49");
        // Régression : Grille.remplacer triait la liste de l'appelant.
        assert_eq!(picks, [49, 3, 12, 7, 33, 21, 40]);
    }

    #[test]
    fn mapping_rejects_bad_selections() {
        let table = Table::parse(7, 3, "01,02,03,04,05\n").unwrap();
        assert!(matches!(
            map_table(&table, &[1, 2, 3, 4, 5, 6]),
            Err(Error::NoTable { n: 6, .. })
        ));
        assert_eq!(
            map_table(&table, &[1, 2, 3, 4, 5, 6, 6]),
            Err(Error::DuplicateNumber(6))
        );
        assert_eq!(
            map_table(&table, &[1, 2, 3, 4, 5, 6, 51]),
            Err(Error::InvalidNumber(51))
        );
    }

    #[test]
    fn normalize_sorts() {
        assert_eq!(normalize_selection(&[5, 1, 3]).unwrap(), vec![1, 3, 5]);
    }
}
