//! Mise en forme commune (CLI, TUI, fichiers de résultats).

use srm_core::report::{Comparison, Report};

pub fn euros(x: f64) -> String {
    format!("{x:.2} €").replace('.', ",")
}

pub fn percent(p: f64) -> String {
    if p > 0.0 && p < 0.0001 {
        format!("{:.4} %", p * 100.0).replace('.', ",")
    } else {
        format!("{:.2} %", p * 100.0).replace('.', ",")
    }
}

/// « 1 sur 137 », plus parlant qu'un pourcentage pour les petites probabilités.
pub fn one_in(p: f64) -> String {
    if p <= 0.0 {
        "jamais".into()
    } else {
        format!("1 sur {}", (1.0 / p).round() as u64)
    }
}

pub fn numbers(ns: &[u8]) -> String {
    ns.iter()
        .map(|n| format!("{n:02}"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Résumé chiffré d'un rapport, ligne par ligne.
pub fn summary(r: &Report) -> Vec<String> {
    vec![
        format!("Numéros ({}) : {}", r.numbers.len(), numbers(&r.numbers)),
        format!(
            "Garantie     : {} bons numéros si les 5 gagnants sont parmi vos {} numéros",
            r.t,
            r.numbers.len()
        ),
        format!("Grilles      : {}", r.grids.len()),
        format!("Coût         : {}", euros(r.cost)),
        format!(
            "La garantie s'applique dans {} des tirages ({})",
            percent(r.prob_guarantee),
            one_in(r.prob_guarantee)
        ),
        format!(
            "Perte moyenne attendue : {} (le système ne change pas l'espérance de gain)",
            euros(r.expected_loss)
        ),
    ]
}

/// Tableau texte de la comparaison système / hasard, en cumulé : part des tirages où au moins
/// une grille a `k` bons numéros ou plus. Chaque ligne se lit seule, contrairement à « exactement k ».
pub fn comparison(c: &Comparison) -> Vec<String> {
    let mut out = vec![
        "Part des 2 118 760 tirages où au moins une grille a… :".to_string(),
        format!("{:>16}  {:>12}  {:>12}", "", "système", "hasard"),
    ];
    for k in (1..=5).rev() {
        out.push(format!(
            "{:>16}  {:>12}  {:>12}",
            format!("≥ {k} bon{}", if k > 1 { "s" } else { "" }),
            percent(c.system.prob_at_least(k)),
            percent(c.random.prob_at_least(k))
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn french_number_format() {
        assert_eq!(euros(150.0), "150,00 €");
        assert_eq!(percent(0.007317), "0,73 %");
        assert_eq!(percent(0.0000012), "0,0001 %");
        assert_eq!(one_in(0.007317), "1 sur 137");
        assert_eq!(numbers(&[3, 12]), "03 12");
    }

    #[test]
    fn comparison_is_cumulative() {
        use srm_core::odds::Distribution;
        let c = Comparison {
            system: Distribution {
                counts: [10, 40, 30, 15, 4, 1],
            },
            random: Distribution {
                counts: [10, 50, 25, 11, 3, 1],
            },
        };
        let lines = comparison(&c);
        assert_eq!(lines.len(), 2 + 5);
        // ≥ 2 : système 50 %, hasard 40 % ; ≥ 1 : 90 % des deux côtés.
        let ge2 = lines.iter().find(|l| l.contains("≥ 2")).unwrap();
        assert!(ge2.contains("50,00 %") && ge2.contains("40,00 %"), "{ge2}");
        let ge1 = lines.iter().find(|l| l.contains("≥ 1")).unwrap();
        assert_eq!(ge1.matches("90,00 %").count(), 2, "{ge1}");
    }
}
