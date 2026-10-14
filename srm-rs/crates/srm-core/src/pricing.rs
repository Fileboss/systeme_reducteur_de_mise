//! Prix des systèmes et suggestions selon la mise.

use serde::Serialize;

use crate::table::{Table, embedded_tables};

/// Prix d'une grille par défaut : celui de l'EuroMillions, jeu par défaut
/// (voir [`crate::game::Game::default_price`] pour les autres jeux).
pub const DEFAULT_GRID_PRICE: f64 = crate::game::Game::EuroMillions.default_price();

/// Marge par défaut autour de la mise visée (±15 %).
pub const DEFAULT_MARGIN: f64 = 1.15;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Offer {
    pub n: usize,
    pub t: u8,
    /// Grilles de numéros.
    pub grids: usize,
    /// Tickets joués : grilles × combinaisons de compléments.
    pub tickets: usize,
    pub price: f64,
}

impl Offer {
    /// `combos` : nombre de combinaisons de compléments jouées avec chaque grille (1 sans réduction).
    pub fn from_table(table: &Table, grid_price: f64, combos: usize) -> Self {
        let tickets = table.len() * combos.max(1);
        Self {
            n: table.n(),
            t: table.t(),
            grids: table.len(),
            tickets,
            price: tickets as f64 * grid_price,
        }
    }
}

/// Toutes les offres disponibles, triées par `(n, t)`.
pub fn catalog(grid_price: f64, combos: usize) -> Vec<Offer> {
    catalog_from(embedded_tables(), grid_price, combos)
}

pub fn catalog_from(tables: &[Table], grid_price: f64, combos: usize) -> Vec<Offer> {
    let mut offers: Vec<Offer> = tables
        .iter()
        .map(|t| Offer::from_table(t, grid_price, combos))
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
            tickets: grids,
            price: grids as f64 * 2.5,
        }
    }

    #[test]
    fn combos_multiply_tickets_and_price() {
        let table = Table::parse(7, 3, "01,02,03,04,05\n03,04,05,06,07\n").unwrap();
        let o = Offer::from_table(&table, 2.2, 3);
        assert_eq!((o.grids, o.tickets), (2, 6));
        assert!((o.price - 13.2).abs() < 1e-9);
        assert_eq!(Offer::from_table(&table, 2.5, 0).tickets, 2);
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
