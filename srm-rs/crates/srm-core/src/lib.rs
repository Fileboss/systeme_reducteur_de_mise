//! Cœur du système réducteur de mise pour le Loto (5 numéros parmi 50).
//!
//! - [`generate`] calcule des couvertures minimales de `{1..n}` (glouton puis recuit simulé) ;
//! - [`table`] embarque les couvertures pré-calculées ;
//! - [`mapping`] les applique aux numéros choisis par le joueur ;
//! - [`cover`] vérifie une couverture par force brute ;
//! - [`odds`] et [`pricing`] chiffrent ce que le système garantit et ce qu'il coûte.

pub mod combinatorics;
pub mod cover;
pub mod error;
pub mod generate;
pub mod grid;
pub mod mapping;
pub mod odds;
pub mod pricing;
pub mod report;
pub mod space;
pub mod table;

pub use error::{Error, Result};
pub use grid::Grid;
pub use table::Table;
