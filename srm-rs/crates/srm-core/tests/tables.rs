//! Les tables embarquées tiennent leur promesse, et font mieux que la version Java de 2019.

use std::path::PathBuf;

use srm_core::cover::verify;
use srm_core::space::{MAX_N, MAX_T, MIN_N, MIN_T};
use srm_core::table::{Table, embedded_tables};

#[test]
fn every_supported_pair_has_a_table() {
    for t in MIN_T..=MAX_T {
        for n in MIN_N..=MAX_N {
            assert!(Table::embedded(n, t).is_ok(), "table {n}_{t} manquante");
        }
    }
}

/// Le test le plus important : chaque table embarquée est une couverture valide.
#[test]
fn every_embedded_table_is_a_valid_cover() {
    for table in embedded_tables() {
        verify(table.n(), table.t(), table.grids())
            .unwrap_or_else(|e| panic!("table {}_{} : {e}", table.n(), table.t()));
    }
}

fn legacy_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../SystReducV2/Ressources")
}

#[test]
fn legacy_java_tables_are_valid_and_never_smaller() {
    let dir = legacy_dir();
    let mut compared = 0;
    for table in embedded_tables() {
        let path = dir.join(format!("data_{}_{}.txt", table.n(), table.t()));
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let legacy = Table::parse(table.n(), table.t(), &text).unwrap();
        verify(legacy.n(), legacy.t(), legacy.grids()).unwrap();
        assert!(
            table.len() <= legacy.len(),
            "{}_{} : {} grilles contre {} en Java",
            table.n(),
            table.t(),
            table.len(),
            legacy.len()
        );
        compared += 1;
    }
    assert!(
        compared >= 50,
        "seulement {compared} tables Java trouvées dans {}",
        dir.display()
    );
}

/// Solution de M. Fehrenbach pour 20 numéros / garantie 3 (SystReducV2/src/verifAlgoFehrenbach.java),
/// en bitmasks : bit i ⇔ numéro i + 1, comme dans ce crate.
const TUTOR_20_3: [u64; 48] = [
    31, 992, 31744, 1015808, 533040, 51268, 67208, 147498, 139457, 69906, 264961, 790660, 37921,
    525634, 303368, 410640, 34962, 75812, 530504, 135940, 606721, 327792, 263686, 115072, 172546,
    134408, 20547, 24724, 657568, 33368, 77833, 524581, 197700, 283168, 525457, 796674, 574472,
    198673, 262346, 32940, 10576, 104960, 9378, 135320, 680192, 34068, 344076, 18569,
];

#[test]
fn tutor_solution_is_valid_and_ours_is_not_larger() {
    let grids: Vec<_> = TUTOR_20_3
        .iter()
        .map(|&m| srm_core::Grid::from_mask(m).unwrap())
        .collect();
    verify(20, 3, &grids).expect("la solution du tuteur doit couvrir tous les tirages");
    let ours = Table::embedded(20, 3).unwrap();
    assert!(ours.len() <= TUTOR_20_3.len(), "{} grilles", ours.len());
}
