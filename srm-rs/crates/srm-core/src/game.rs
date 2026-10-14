//! Règles des jeux supportés : taille du pool de numéros et numéro complémentaire.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Game {
    /// 5 numéros parmi 50 et 2 étoiles parmi 12 (le jeu modélisé par la version de 2019).
    #[default]
    EuroMillions,
    /// 5 numéros parmi 49 et 1 numéro chance parmi 10.
    Loto,
}

impl Game {
    pub const ALL: [Game; 2] = [Game::EuroMillions, Game::Loto];

    /// Plus grand numéro principal.
    pub const fn pool(self) -> u8 {
        match self {
            Game::EuroMillions => 50,
            Game::Loto => 49,
        }
    }

    /// Nombre de compléments sur une grille.
    pub const fn extras_per_grid(self) -> u8 {
        match self {
            Game::EuroMillions => 2,
            Game::Loto => 1,
        }
    }

    /// Plus grand complément.
    pub const fn extra_pool(self) -> u8 {
        match self {
            Game::EuroMillions => 12,
            Game::Loto => 10,
        }
    }

    /// Prix d'une grille par défaut, paramétrable partout.
    pub const fn default_price(self) -> f64 {
        match self {
            Game::EuroMillions => 2.5,
            Game::Loto => 2.2,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Game::EuroMillions => "EuroMillions",
            Game::Loto => "Loto",
        }
    }

    pub const fn id(self) -> &'static str {
        match self {
            Game::EuroMillions => "euromillions",
            Game::Loto => "loto",
        }
    }

    /// « étoile » ou « numéro chance ».
    pub const fn extra_name(self) -> &'static str {
        match self {
            Game::EuroMillions => "étoile",
            Game::Loto => "numéro chance",
        }
    }

    pub const fn extra_plural(self) -> &'static str {
        match self {
            Game::EuroMillions => "étoiles",
            Game::Loto => "numéros chance",
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.trim().to_lowercase().as_str() {
            "euromillions" | "euromillion" | "euro" | "em" => Ok(Game::EuroMillions),
            "loto" => Ok(Game::Loto),
            other => Err(Error::Extras(format!(
                "jeu inconnu : « {other} » (euromillions ou loto)"
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rules() {
        assert_eq!(Game::default(), Game::EuroMillions);
        assert_eq!(
            (
                Game::EuroMillions.pool(),
                Game::EuroMillions.extras_per_grid(),
                Game::EuroMillions.extra_pool()
            ),
            (50, 2, 12)
        );
        assert_eq!(
            (
                Game::Loto.pool(),
                Game::Loto.extras_per_grid(),
                Game::Loto.extra_pool()
            ),
            (49, 1, 10)
        );
        assert_eq!(Game::Loto.default_price(), 2.2);
    }

    #[test]
    fn parsing_and_display() {
        assert_eq!("EuroMillions".parse::<Game>().unwrap(), Game::EuroMillions);
        assert_eq!(" loto ".parse::<Game>().unwrap(), Game::Loto);
        assert!("keno".parse::<Game>().is_err());
        for g in Game::ALL {
            assert_eq!(g.id().parse::<Game>().unwrap(), g);
            assert_eq!(g.to_string(), g.name());
        }
    }
}
