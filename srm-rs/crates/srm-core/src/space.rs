//! Espace de recherche d'une couverture : toutes les grilles de `{1..n}`, et la relation
//! « partage au moins `t` numéros ».

use crate::combinatorics::{GRID_SIZE, all_grids, full_mask, rank, small_binomial};
use crate::error::{Error, Result};

/// Plus petite taille de système réducteur supportée.
pub const MIN_N: usize = 6;
/// Plus grande taille de système réducteur supportée (limite des tables pré-générées).
pub const MAX_N: usize = 30;
pub const MIN_T: u8 = 2;
pub const MAX_T: u8 = 4;

/// Vérifie que `(n, t)` est un couple supporté.
pub fn check_params(n: usize, t: u8) -> Result<()> {
    if (MIN_N..=MAX_N).contains(&n) && (MIN_T..=MAX_T).contains(&t) {
        Ok(())
    } else {
        Err(Error::UnsupportedParams { n, t })
    }
}

/// Toutes les grilles de `{1..n}` indexées par rang, avec le seuil de garantie `t`.
///
/// Une grille `g` couvre un tirage `d` si `|g ∩ d| ≥ t`. La relation est symétrique,
/// donc l'ensemble des voisins d'une grille est aussi l'ensemble des tirages qu'elle couvre.
pub struct Space {
    n: u8,
    t: u8,
    masks: Vec<u64>,
    degree: usize,
}

impl Space {
    pub fn new(n: usize, t: u8) -> Result<Self> {
        check_params(n, t)?;
        let n8 = n as u8;
        let masks: Vec<u64> = all_grids(n8).collect();
        let degree = (usize::from(t)..=GRID_SIZE)
            .map(|k| small_binomial(GRID_SIZE, k) * small_binomial(n - GRID_SIZE, GRID_SIZE - k))
            .sum::<u64>() as usize;
        Ok(Self {
            n: n8,
            t,
            masks,
            degree,
        })
    }

    pub fn n(&self) -> usize {
        usize::from(self.n)
    }

    pub fn t(&self) -> u8 {
        self.t
    }

    /// Nombre de grilles (et de tirages) : `C(n, 5)`.
    pub fn len(&self) -> usize {
        self.masks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.masks.is_empty()
    }

    #[inline]
    pub fn mask(&self, rank: u32) -> u64 {
        self.masks[rank as usize]
    }

    /// Nombre de tirages couverts par une grille (identique pour toutes les grilles).
    pub fn degree(&self) -> usize {
        self.degree
    }

    #[inline]
    pub fn covers(&self, a: u64, b: u64) -> bool {
        (a & b).count_ones() >= u32::from(self.t)
    }

    /// Appelle `f(rang, masque)` pour chaque grille ayant au moins `t` numéros en commun avec `g`
    /// (`g` incluse). Les voisins sont construits directement, sans parcourir tout l'espace :
    /// on choisit `k ≥ t` numéros dans `g` et `5 − k` hors de `g`.
    #[inline]
    pub fn for_each_neighbor(&self, g: u64, mut f: impl FnMut(u32, u64)) {
        let mut inside = [0u64; GRID_SIZE];
        let mut m = g;
        for slot in &mut inside {
            *slot = m.isolate_lowest_one();
            m &= m - 1;
        }
        let mut outside = [0u64; 64];
        let mut n_out = 0;
        let mut m = full_mask(self.n) & !g;
        while m != 0 {
            outside[n_out] = m.isolate_lowest_one();
            n_out += 1;
            m &= m - 1;
        }

        for sel in 0u32..(1 << GRID_SIZE) {
            let k = sel.count_ones() as usize;
            if k < usize::from(self.t) {
                continue;
            }
            let mut a = 0u64;
            for (i, bit) in inside.iter().enumerate() {
                if sel & (1 << i) != 0 {
                    a |= bit;
                }
            }
            match GRID_SIZE - k {
                0 => f(rank(a), a),
                1 => {
                    for &x in &outside[..n_out] {
                        let h = a | x;
                        f(rank(h), h);
                    }
                }
                2 => {
                    for i in 0..n_out {
                        let ai = a | outside[i];
                        for &y in &outside[i + 1..n_out] {
                            let h = ai | y;
                            f(rank(h), h);
                        }
                    }
                }
                3 => {
                    for i in 0..n_out {
                        let ai = a | outside[i];
                        for j in i + 1..n_out {
                            let aj = ai | outside[j];
                            for &z in &outside[j + 1..n_out] {
                                let h = aj | z;
                                f(rank(h), h);
                            }
                        }
                    }
                }
                _ => unreachable!("t ≥ 2 donc au plus 3 numéros hors de la grille"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbors_match_brute_force() {
        for n in [6usize, 9, 13] {
            for t in 2..=4u8 {
                let space = Space::new(n, t).unwrap();
                for r in (0..space.len() as u32).step_by(7) {
                    let g = space.mask(r);
                    let mut got = Vec::new();
                    space.for_each_neighbor(g, |rk, m| {
                        assert_eq!(space.mask(rk), m);
                        got.push(rk);
                    });
                    got.sort_unstable();
                    let expected: Vec<u32> = (0..space.len() as u32)
                        .filter(|&d| space.covers(g, space.mask(d)))
                        .collect();
                    assert_eq!(got, expected, "n={n} t={t} g={g:#b}");
                    assert_eq!(got.len(), space.degree());
                }
            }
        }
    }

    #[test]
    fn degree_values() {
        // 30 numéros : C(5,2)C(25,3) + C(5,3)C(25,2) + C(5,4)·25 + 1
        assert_eq!(
            Space::new(30, 2).unwrap().degree(),
            23_000 + 3_000 + 125 + 1
        );
        assert_eq!(Space::new(30, 4).unwrap().degree(), 126);
        assert_eq!(Space::new(20, 3).unwrap().len(), 15_504);
    }

    #[test]
    fn rejects_unsupported_params() {
        assert!(Space::new(5, 2).is_err());
        assert!(Space::new(31, 2).is_err());
        assert!(Space::new(20, 1).is_err());
        assert!(Space::new(20, 5).is_err());
    }
}
