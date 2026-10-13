//! Prix des systèmes et suggestions selon la mise.

use serde::Serialize;

use crate::table::{Table, embedded_tables};

/// Prix d'une grille par défaut (celui de la version de 2019, paramétrable partout).
pub const DEFAULT_GRID_PRICE: f64 = 2.5;

/// Marge par défaut autour de la mise visée (±15 %).
pub const DEFAULT_MARGIN: f64 = 1.15;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Offer {
    pub n: usize,
    pub t: u8,
    pub grids: usize,
    pub price: f64,
}

impl Offer {
    pub fn from_table(table: &Table, grid_price: f64) -> Self {
        Self {
            n: table.n(),
            t: table.t(),
            grids: table.len(),
            price: table.len() as f64 * grid_price,
        }
    }
}

/// Toutes les offres disponibles, triées par `(n, t)`.
pub fn catalog(grid_price: f64) -> Vec<Offer> {
    catalog_from(embedded_tables(), grid_price)
}

pub fn catalog_from(tables: &[Table], grid_price: f64) -> Vec<Offer> {
    let mut offers: Vec<Offer> = tables
        .iter()
        .map(|t| Offer::from_table(t, grid_price))
        .collect();
    offers.sort_by_key(|o| (o.n, o.t));
    offers
}

/// Offres dont le prix est dans `[mise / marge, mise × marge]`, de la plus proche à la plus éloignée de la mise.
pub fn suggest(offers: &[Offer], budget: f64, margin: f64) -> Vec<Offer> {
    let margin = margin.max(1.0);
    let (lo, hi) = (budget / margin, budget * margin);
    let mut out: Vec<Offer> = offers
        .iter()
        .copied()
        .filter(|o| o.price >= lo && o.price <= hi)
        .collect();
    out.sort_by(|a, b| {
        (a.price - budget)
            .abs()
            .total_cmp(&(b.price - budget).abs())
            .then((b.n, b.t).cmp(&(a.n, a.t)))
    });
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn offer(n: usize, t: u8, grids: usize) -> Offer {
        Offer {
            n,
            t,
            grids,
            price: grids as f64 * 2.5,
        }
    }

    #[test]
    fn suggest_filters_by_margin_and_sorts_by_distance() {
        let offers = [
            offer(15, 3, 21),
            offer(20, 3, 48),
            offer(16, 3, 40),
            offer(25, 2, 18),
            offer(30, 3, 200),
        ];
        let s = suggest(&offers, 108.0, 1.15);
        // 108 / 1.15 ≈ 93,9 ; 108 × 1.15 = 124,2 → 100 € (40 grilles) puis 120 € (48 grilles).
        assert_eq!(s.iter().map(|o| o.grids).collect::<Vec<_>>(), vec![40, 48]);
    }

    #[test]
    fn suggest_has_no_duplicates() {
        // Régression : recupererParametres ajoutait deux fois la première ligne du fichier de prix.
        let offers = [offer(11, 2, 2), offer(12, 2, 3)];
        assert_eq!(suggest(&offers, 5.0, 1.15).len(), 1);
    }

    #[test]
    fn margin_below_one_is_clamped() {
        let offers = [offer(11, 2, 2)];
        assert_eq!(suggest(&offers, 5.0, 0.5).len(), 1);
    }
}
