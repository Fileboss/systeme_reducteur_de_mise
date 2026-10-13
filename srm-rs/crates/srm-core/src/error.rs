use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("numéro invalide : {0} (doit être entre 1 et 50)")]
    InvalidNumber(i64),
    #[error("numéro en double : {0}")]
    DuplicateNumber(u8),
    #[error("grille invalide : {0}")]
    InvalidGrid(String),
    #[error(
        "paramètres non supportés : {n} numéros, garantie {t} (attendu : 6 ≤ n ≤ 30, 2 ≤ t ≤ 4)"
    )]
    UnsupportedParams { n: usize, t: u8 },
    #[error("aucune table pour {n} numéros et garantie {t}")]
    NoTable { n: usize, t: u8 },
    #[error("ligne {line} : {msg}")]
    Parse { line: usize, msg: String },
    #[error("couverture invalide : le tirage {0} n'est pas couvert")]
    NotCovered(String),
}

pub type Result<T> = std::result::Result<T, Error>;
