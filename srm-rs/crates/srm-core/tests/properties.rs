//! Propriétés vérifiées sur des sélections aléatoires de numéros.

use proptest::prelude::*;
use proptest::sample::subsequence;
use srm_core::Grid;
use srm_core::combinatorics::all_grids;
use srm_core::extras::{ExtraMode, ExtraSet};
use srm_core::game::Game;
use srm_core::mapping::reduce;
use srm_core::report::{Settings, build};

fn selection() -> impl Strategy<Value = Vec<u8>> {
    (6usize..=16)
        .prop_flat_map(|n| subsequence((1u8..=50).collect::<Vec<_>>(), n))
        .prop_shuffle()
}

/// Un jeu, des numéros (6 à 12) dans son pool, des compléments et un mode cohérents.
fn joint_case() -> impl Strategy<Value = (Game, Vec<u8>, Vec<u8>, ExtraMode)> {
    prop_oneof![Just(Game::EuroMillions), Just(Game::Loto)].prop_flat_map(|game| {
        let per = usize::from(game.extras_per_grid());
        let numbers = (6usize..=12).prop_flat_map(move |n| {
            subsequence((1..=game.pool()).collect::<Vec<_>>(), n).prop_shuffle()
        });
        let fixed = subsequence((1..=game.extra_pool()).collect::<Vec<_>>(), per)
            .prop_map(|e| (e, ExtraMode::Fixed));
        let reduced = (per..=6, 0..=game.extras_per_grid()).prop_flat_map(move |(m, t)| {
            subsequence((1..=game.extra_pool()).collect::<Vec<_>>(), m)
                .prop_map(move |e| (e, ExtraMode::Reduced { t }))
        });
        (Just(game), numbers, prop_oneof![fixed, reduced]).prop_map(|(g, n, (e, m))| (g, n, e, m))
    })
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

    /// Garantie jointe : si les 5 numéros et les compléments gagnants sont parmi ceux choisis,
    /// un même ticket a au moins `t` bons numéros et au moins `s` bons compléments.
    #[test]
    fn joint_guarantee_holds_with_extras(
        (game, numbers, extras, mode) in joint_case(),
        t in 2u8..=4,
    ) {
        let r = build(&numbers, t, &extras, mode, Settings::for_game(game)).unwrap();
        let s = u32::from(mode.guarantee(game));
        let tickets: Vec<(Grid, ExtraSet)> = r.tickets().map(|(g, e)| (g, e.unwrap())).collect();
        prop_assert_eq!(tickets.len(), r.tickets);

        let per = usize::from(game.extras_per_grid());
        let mut extra_draws = Vec::new();
        for mask in 0u16..(1 << r.extras.len()) {
            if mask.count_ones() as usize == per {
                let pick: Vec<u8> = (0..r.extras.len()).filter(|i| mask & (1 << i) != 0).map(|i| r.extras[i]).collect();
                extra_draws.push(ExtraSet::from_numbers(&pick));
            }
        }
        for local in all_grids(r.numbers.len() as u8) {
            let draw = Grid::new(&Grid::from_mask(local).unwrap().numbers().map(|i| r.numbers[usize::from(i) - 1])).unwrap();
            for &e in &extra_draws {
                prop_assert!(
                    tickets.iter().any(|(g, x)| g.common(draw) >= u32::from(t) && x.common(e) >= s),
                    "tirage {draw} + {e} non couvert"
                );
            }
        }
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
