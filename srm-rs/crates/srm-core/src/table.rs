//! Tables pré-générées : pour chaque `(n, t)`, une couverture de `{1..n}`.
//!
//! Format texte, compatible avec les fichiers `data_n_t.txt` de la version Java :
//! une grille par ligne (`01,02,03,04,05`), lignes vides et commentaires `#` ignorés.

use std::sync::OnceLock;

use crate::error::{Error, Result};
use crate::grid::Grid;
use crate::space::check_params;

mod embedded {
    include!(concat!(env!("OUT_DIR"), "/tables.rs"));
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    n: usize,
    t: u8,
    grids: Vec<Grid>,
}

impl Table {
    /// Construit une table ; les grilles doivent n'utiliser que des numéros de `{1..n}`.
    /// La validité de la couverture n'est pas vérifiée ici (voir [`crate::cover::verify`]).
    pub fn new(n: usize, t: u8, mut grids: Vec<Grid>) -> Result<Self> {
        check_params(n, t)?;
        if let Some(g) = grids.iter().find(|g| usize::from(g.numbers()[4]) > n) {
            return Err(Error::InvalidGrid(format!("{g} sort de 1..{n}")));
        }
        grids.sort_unstable();
        grids.dedup();
        Ok(Self { n, t, grids })
    }

    pub fn parse(n: usize, t: u8, text: &str) -> Result<Self> {
        let grids = text
            .lines()
            .enumerate()
            .filter(|(_, l)| {
                let l = l.trim();
                !l.is_empty() && !l.starts_with('#')
            })
            .map(|(i, l)| {
                l.trim().parse::<Grid>().map_err(|e| Error::Parse {
                    line: i + 1,
                    msg: e.to_string(),
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Self::new(n, t, grids)
    }

    /// Sérialise la table, précédée d'un commentaire d'en-tête facultatif.
    pub fn to_text(&self, header: Option<&str>) -> String {
        let mut out = String::new();
        if let Some(h) = header {
            for line in h.lines() {
                out.push_str("# ");
                out.push_str(line);
                out.push('\n');
            }
        }
        for g in &self.grids {
            out.push_str(&g.to_string());
            out.push('\n');
        }
        out
    }

    pub fn n(&self) -> usize {
        self.n
    }

    pub fn t(&self) -> u8 {
        self.t
    }

    pub fn grids(&self) -> &[Grid] {
        &self.grids
    }

    pub fn len(&self) -> usize {
        self.grids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.grids.is_empty()
    }

    /// Table embarquée dans le binaire pour `(n, t)`.
    pub fn embedded(n: usize, t: u8) -> Result<&'static Table> {
        check_params(n, t)?;
        embedded_tables()
            .iter()
            .find(|tb| tb.n == n && tb.t == t)
            .ok_or(Error::NoTable { n, t })
    }
}

/// Toutes les tables embarquées, triées par `(n, t)`.
pub fn embedded_tables() -> &'static [Table] {
    static TABLES: OnceLock<Vec<Table>> = OnceLock::new();
    TABLES.get_or_init(|| {
        embedded::EMBEDDED
            .iter()
            .map(|&(n, t, text)| {
                Table::parse(n, t, text)
                    .unwrap_or_else(|e| panic!("table embarquée {n}_{t} illisible : {e}"))
            })
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_legacy_format_with_blank_lines_and_comments() {
        let text = "# en-tête\n01,02,03,04,05\n\n03,04,05,06,07\r\n  02,03,04,05,06  \n";
        let t = Table::parse(7, 3, text).unwrap();
        assert_eq!(t.len(), 3);
        assert_eq!(t.grids()[0].to_string(), "01,02,03,04,05");
        assert_eq!(t.grids()[1].to_string(), "02,03,04,05,06");
    }

    #[test]
    fn duplicates_are_removed() {
        // Régression : l'écriture en APPEND de la version Java pouvait dupliquer des grilles.
        let t = Table::parse(7, 3, "01,02,03,04,05\n01,02,03,04,05\n").unwrap();
        assert_eq!(t.len(), 1);
    }

    #[test]
    fn text_round_trip() {
        let t = Table::parse(9, 2, "01,02,03,04,05\n05,06,07,08,09\n").unwrap();
        let text = t.to_text(Some("ligne 1\nligne 2"));
        assert!(text.starts_with("# ligne 1\n# ligne 2\n"));
        assert_eq!(Table::parse(9, 2, &text).unwrap(), t);
    }

    #[test]
    fn errors_are_located() {
        let err = Table::parse(9, 2, "01,02,03,04,05\n01,02,x,04,05\n").unwrap_err();
        assert!(matches!(err, Error::Parse { line: 2, .. }));
        assert!(Table::parse(9, 2, "01,02,03,04,10\n").is_err());
        assert!(Table::parse(40, 2, "").is_err());
    }
}
