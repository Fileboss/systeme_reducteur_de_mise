//! Propriétés vérifiées sur des sélections aléatoires de numéros.

use proptest::prelude::*;
use proptest::sample::subsequence;
use srm_core::Grid;
use srm_core::combinatorics::all_grids;
use srm_core::mapping::reduce;

fn selection() -> impl Strategy<Value = Vec<u8>> {
    (6usize..=16)
        .prop_flat_map(|n| subsequence((1u8..=50).collect::<Vec<_>>(), n))
        .prop_shuffle()
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    /// Après correspondance, tout tirage inclus dans les numéros choisis a au moins `t` bons
    /// numéros sur une des grilles : la garantie survit au changement de numéros.
    #[test]
    fn guarantee_holds_for_any_selection(numbers in selection(), t in 2u8..=4) {
        let grids = reduce(&numbers, t).unwrap();
        let mut sorted = numbers.clone();
        sorted.sort_unstable();
        for local in all_grids(sorted.len() as u8) {
            let draw = Grid::new(&Grid::from_mask(local).unwrap().numbers().map(|i| sorted[usize::from(i) - 1])).unwrap();
            prop_assert!(grids.iter().any(|g| g.common(draw) >= u32::from(t)), "tirage {draw} non couvert");
        }
        prop_assert!(grids.iter().all(|g| g.numbers().iter().all(|x| numbers.contains(x))));
    }

    #[test]
    fn grid_text_round_trip(numbers in subsequence((1u8..=50).collect::<Vec<_>>(), 5).prop_shuffle()) {
        let g = Grid::new(&numbers).unwrap();
        prop_assert_eq!(g.to_string().parse::<Grid>().unwrap(), g);
        let mut sorted = numbers.clone();
        sorted.sort_unstable();
        prop_assert_eq!(g.numbers().to_vec(), sorted);
    }
}
