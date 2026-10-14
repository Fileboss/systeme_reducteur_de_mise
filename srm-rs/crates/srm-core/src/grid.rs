use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::combinatorics::{GRID_SIZE, MAX_NUMBER};
use crate::error::{Error, Result};

/// Une grille de 5 numéros distincts entre 1 et 50, stockée en bitmask (bit `i` ⇔ numéro `i + 1`).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Grid(u64);

impl Grid {
    /// Construit une grille à partir de 5 numéros, dans n'importe quel ordre.
    pub fn new(numbers: &[u8]) -> Result<Self> {
        if numbers.len() != GRID_SIZE {
            return Err(Error::InvalidGrid(format!(
                "{} numéros au lieu de 5",
                numbers.len()
            )));
        }
        let mut mask = 0u64;
        for &n in numbers {
            if n == 0 || n > MAX_NUMBER {
                return Err(Error::InvalidNumber(i64::from(n)));
            }
            let bit = 1u64 << (n - 1);
            if mask & bit != 0 {
                return Err(Error::DuplicateNumber(n));
            }
            mask |= bit;
        }
        Ok(Self(mask))
    }

    /// Construit une grille depuis un bitmask (5 bits parmi les 50 premiers).
    pub fn from_mask(mask: u64) -> Result<Self> {
        if mask.count_ones() as usize != GRID_SIZE || mask >> MAX_NUMBER != 0 {
            return Err(Error::InvalidGrid(format!("masque {mask:#b}")));
        }
        Ok(Self(mask))
    }

    #[inline]
    pub(crate) const fn from_mask_unchecked(mask: u64) -> Self {
        Self(mask)
    }

    #[inline]
    pub const fn mask(self) -> u64 {
        self.0
    }

    /// Les 5 numéros, triés par ordre croissant.
    pub fn numbers(self) -> [u8; GRID_SIZE] {
        let mut out = [0u8; GRID_SIZE];
        let mut m = self.0;
        for slot in &mut out {
            *slot = m.trailing_zeros() as u8 + 1;
            m &= m - 1;
        }
        out
    }

    /// Nombre de numéros en commun avec une autre grille (ou un tirage).
    #[inline]
    pub const fn common(self, other: Grid) -> u32 {
        (self.0 & other.0).count_ones()
    }

    #[inline]
    pub const fn contains(self, number: u8) -> bool {
        number >= 1 && number <= MAX_NUMBER && self.0 & (1 << (number - 1)) != 0
    }
}

impl Ord for Grid {
    /// Ordre lexicographique sur les numéros triés, comme les fichiers de l'ancienne version.
    fn cmp(&self, other: &Self) -> Ordering {
        self.numbers().cmp(&other.numbers())
    }
}

impl PartialOrd for Grid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.numbers();
        write!(
            f,
            "{:02},{:02},{:02},{:02},{:02}",
            n[0], n[1], n[2], n[3], n[4]
        )
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Grid({self})")
    }
}

impl FromStr for Grid {
    type Err = Error;

    /// Accepte `01,02,03,04,05`, `1 2 3 4 5` ou un mélange ; l'ordre n'importe pas.
    fn from_str(s: &str) -> Result<Self> {
        let numbers = parse_numbers(s)?;
        Grid::new(&numbers)
    }
}

impl Serialize for Grid {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        self.numbers().serialize(s)
    }
}

impl<'de> Deserialize<'de> for Grid {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let numbers = Vec::<u8>::deserialize(d)?;
        Grid::new(&numbers).map_err(serde::de::Error::custom)
    }
}

/// Découpe une saisie libre (`3 7,12 ; 18`) en numéros, sans valider les doublons.
pub fn parse_numbers(s: &str) -> Result<Vec<u8>> {
    s.split(|c: char| c == ',' || c == ';' || c.is_whitespace())
        .filter(|tok| !tok.is_empty())
        .map(|tok| {
            let value: i64 = tok
                .parse()
                .map_err(|_| Error::InvalidGrid(format!("« {tok} » n'est pas un nombre")))?;
            u8::try_from(value)
                .ok()
                .filter(|&v| (1..=MAX_NUMBER).contains(&v))
                .ok_or(Error::InvalidNumber(value))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction_sorts_numbers() {
        let g = Grid::new(&[5, 1, 4, 2, 3]).unwrap();
        assert_eq!(g.numbers(), [1, 2, 3, 4, 5]);
        assert_eq!(g.mask(), 0b11111);
    }

    #[test]
    fn parse_unsorted_string_is_sorted() {
        // Régression : Grille(String) en Java ne triait pas les numéros.
        let g: Grid = "05,01,04,02,03".parse().unwrap();
        assert_eq!(g.to_string(), "01,02,03,04,05");
        assert_eq!(g, Grid::new(&[1, 2, 3, 4, 5]).unwrap());
    }

    #[test]
    fn display_round_trip() {
        let g = Grid::new(&[50, 9, 10, 33, 1]).unwrap();
        assert_eq!(g.to_string(), "01,09,10,33,50");
        assert_eq!(g.to_string().parse::<Grid>().unwrap(), g);
    }

    #[test]
    fn rejects_invalid_grids() {
        assert_eq!(
            Grid::new(&[1, 2, 3, 4]),
            Err(Error::InvalidGrid("4 numéros au lieu de 5".into()))
        );
        assert_eq!(Grid::new(&[1, 2, 3, 4, 51]), Err(Error::InvalidNumber(51)));
        assert_eq!(Grid::new(&[0, 2, 3, 4, 5]), Err(Error::InvalidNumber(0)));
        assert_eq!(Grid::new(&[1, 2, 3, 4, 4]), Err(Error::DuplicateNumber(4)));
        assert!("1,2,3,4,x".parse::<Grid>().is_err());
        assert!("1,2,3,4,-3".parse::<Grid>().is_err());
        assert!(Grid::from_mask(0b1111).is_err());
        assert!(Grid::from_mask(0b11111 << 46).is_err());
    }

    #[test]
    fn common_numbers() {
        let a = Grid::new(&[1, 2, 3, 4, 5]).unwrap();
        let b = Grid::new(&[4, 5, 6, 7, 8]).unwrap();
        assert_eq!(a.common(b), 2);
        assert_eq!(a.common(a), 5);
        assert!(a.contains(3) && !a.contains(6) && !a.contains(0) && !a.contains(60));
    }

    #[test]
    fn ordering_is_lexicographic_on_numbers() {
        let a = Grid::new(&[1, 2, 3, 4, 50]).unwrap();
        let b = Grid::new(&[1, 2, 3, 5, 6]).unwrap();
        // En masque, b < a ; en lexicographique, a < b.
        assert!(b.mask() < a.mask());
        assert!(a < b);
    }

    #[test]
    fn parse_numbers_accepts_mixed_separators() {
        assert_eq!(parse_numbers("3 7,12 ;18").unwrap(), vec![3, 7, 12, 18]);
        assert_eq!(parse_numbers("  ").unwrap(), Vec::<u8>::new());
        assert_eq!(parse_numbers("51"), Err(Error::InvalidNumber(51)));
    }
}
