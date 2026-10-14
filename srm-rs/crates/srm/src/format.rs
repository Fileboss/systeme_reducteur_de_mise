//! Mise en forme commune (CLI, TUI, fichiers de résultats).

use srm_core::Grid;
use srm_core::extras::{ExtraMode, ExtraSet};
use srm_core::game::Game;
use srm_core::pricing::Offer;
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
        format!("1 sur {}", thousands((1.0 / p).round() as u64))
    }
}

pub fn numbers(ns: &[u8]) -> String {
    ns.iter()
        .map(|n| format!("{n:02}"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Description courte des règles : « EuroMillions (5 numéros parmi 50, 2 étoiles parmi 12) ».
pub fn game_rules(game: Game) -> String {
    format!(
        "{} (5 numéros parmi {}, {} {} parmi {})",
        game.name(),
        game.pool(),
        game.extras_per_grid(),
        plural(game, game.extras_per_grid().into()),
        game.extra_pool()
    )
}

/// « étoile », « étoiles », « numéro chance » ou « numéros chance ».
pub fn plural(game: Game, count: usize) -> &'static str {
    if count > 1 {
        game.extra_plural()
    } else {
        game.extra_name()
    }
}

/// Compléments d'un ticket : « ★ 02 09 » ou « chance 07 ».
pub fn extras_label(game: Game, extras: ExtraSet) -> String {
    match game {
        Game::EuroMillions => format!("★ {extras}"),
        Game::Loto => format!("chance {extras}"),
    }
}

/// Un ticket : la grille, suivie de ses compléments.
pub fn ticket(game: Game, grid: Grid, extras: Option<ExtraSet>) -> String {
    match (extras, game) {
        (None, _) => grid.to_string(),
        (Some(e), Game::EuroMillions) => format!("{grid} {}", extras_label(game, e)),
        (Some(e), Game::Loto) => format!("{grid} + {}", extras_label(game, e)),
    }
}

/// Code ANSI de la couleur des compléments dans le terminal (magenta), et remise à zéro.
const ANSI_EXTRA: &str = "\x1b[1;35m";
const ANSI_RESET: &str = "\x1b[0m";

/// Comme [`ticket`], avec les compléments en couleur si `color` est vrai.
pub fn ticket_colored(game: Game, grid: Grid, extras: Option<ExtraSet>, color: bool) -> String {
    match extras {
        Some(e) if color => {
            let sep = if game == Game::Loto { " + " } else { " " };
            format!(
                "{grid}{sep}{ANSI_EXTRA}{}{ANSI_RESET}",
                extras_label(game, e)
            )
        }
        _ => ticket(game, grid, extras),
    }
}

/// Couleurs seulement vers un vrai terminal, et jamais si `NO_COLOR` est défini (convention no-color.org).
pub fn use_color() -> bool {
    use std::io::IsTerminal;
    std::env::var_os("NO_COLOR").is_none() && std::io::stdout().is_terminal()
}

/// Une offre de système, pour les suggestions.
pub fn offer(o: &Offer) -> String {
    let tickets = if o.tickets == o.grids {
        String::new()
    } else {
        format!(" ({} tickets)", o.tickets)
    };
    format!(
        "{:>2} numéros, garantie {} : {:>5} grilles{tickets}, {}",
        o.n,
        o.t,
        o.grids,
        euros(o.price)
    )
}

/// Résumé chiffré d'un rapport, ligne par ligne.
pub fn summary(r: &Report) -> Vec<String> {
    let game = r.game;
    let mut out = vec![
        format!("Numéros ({}) : {}", r.numbers.len(), numbers(&r.numbers)),
        format!("Jeu          : {}", game_rules(game)),
    ];
    let many = r.extras.len() > 1;
    let label = format!(
        "{} ({})",
        capitalize(plural(game, r.extras.len())),
        r.extras.len()
    );
    match r.extra_mode {
        None => out.push(format!(
            "{}au choix, non {} en compte dans le système",
            field(&capitalize(game.extra_plural())),
            match game {
                Game::EuroMillions => "prises",
                Game::Loto => "pris",
            }
        )),
        Some(ExtraMode::Fixed) => out.push(format!(
            "{}{} — {}, joué{} sur toutes les grilles",
            field(&label),
            numbers(&r.extras),
            agree(game, many, "fixe"),
            agree(game, many, "")
        )),
        Some(ExtraMode::Reduced { .. }) => out.push(format!(
            "{}{} — {}, {} combinaison{}",
            field(&label),
            numbers(&r.extras),
            agree(game, many, "réduit"),
            r.extra_sets.len(),
            if r.extra_sets.len() > 1 { "s" } else { "" }
        )),
    }
    out.push(format!(
        "{}{} bons numéros si les 5 gagnants sont parmi vos {} numéros",
        field("Garantie"),
        r.t,
        r.numbers.len()
    ));
    if let Some(s) = r.extra_t.filter(|&s| s > 0) {
        out.push(format!(
            "{}et {} si {}",
            field(""),
            extras_won(game, s),
            match game {
                Game::EuroMillions => "les 2 étoiles gagnantes sont parmi les vôtres",
                Game::Loto => "le numéro chance gagnant est parmi les vôtres",
            }
        ));
    }
    if r.extra_sets.is_empty() {
        out.push(format!("{}{}", field("Grilles"), r.grids.len()));
    } else {
        out.push(format!(
            "{}{} ({} grilles × {} combinaison{} {})",
            field("Tickets"),
            r.tickets,
            r.grids.len(),
            r.extra_sets.len(),
            if r.extra_sets.len() > 1 { "s" } else { "" },
            match game {
                Game::EuroMillions => "d'étoiles",
                Game::Loto => "de numéros chance",
            }
        ));
    }
    out.push(format!("{}{}", field("Coût"), euros(r.cost)));
    out.push(format!(
        "La garantie s'applique dans {} des tirages ({})",
        percent(r.prob_guarantee),
        one_in(r.prob_guarantee)
    ));
    out.push(format!(
        "Perte moyenne attendue : {} (le système ne change pas l'espérance de gain)",
        euros(r.expected_loss)
    ));
    out
}

/// Étiquette alignée du résumé : « Coût         : ». Toujours au moins une espace avant les deux-points.
fn field(label: &str) -> String {
    if label.is_empty() {
        " ".repeat(15)
    } else {
        format!("{label:<12} : ")
    }
}

/// Accorde un adjectif (ou un participe, passé en « » pour « joué ») avec les compléments :
/// les étoiles sont féminines, les numéros chance masculins.
fn agree(game: Game, many: bool, adj: &str) -> String {
    let fem = game == Game::EuroMillions;
    match (adj.ends_with('e'), fem, many) {
        (true, _, true) => format!("{adj}s"),
        (true, _, false) => adj.to_string(),
        (false, true, true) => format!("{adj}es"),
        (false, true, false) => format!("{adj}e"),
        (false, false, true) => format!("{adj}s"),
        (false, false, false) => adj.to_string(),
    }
}

/// Ce que garantit le système sur les compléments : « au moins 1 bonne étoile », etc.
fn extras_won(game: Game, s: u8) -> String {
    match (game, s) {
        (Game::EuroMillions, 1) => "au moins 1 bonne étoile".into(),
        (Game::EuroMillions, _) => "les 2 bonnes étoiles".into(),
        (Game::Loto, _) => "le bon numéro chance".into(),
    }
}

/// 2118760 → « 2 118 760 ».
pub fn thousands(x: u64) -> String {
    let s = x.to_string();
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(' ');
        }
        out.push(ch);
    }
    out
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

/// Tableau texte de la comparaison système / hasard, en cumulé : part des tirages où au moins
/// une grille a `k` bons numéros ou plus. Chaque ligne se lit seule, contrairement à « exactement k ».
pub fn comparison(c: &Comparison) -> Vec<String> {
    let mut out = vec![
        format!(
            "Part des {} tirages de numéros où au moins une grille a… :",
            thousands(c.system.total())
        ),
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
        assert_eq!(thousands(2_118_760), "2 118 760");
        assert_eq!(thousands(999), "999");
    }

    #[test]
    fn french_agreement() {
        assert_eq!(agree(Game::EuroMillions, true, "fixe"), "fixes");
        assert_eq!(agree(Game::EuroMillions, true, "réduit"), "réduites");
        assert_eq!(agree(Game::EuroMillions, true, ""), "es");
        assert_eq!(agree(Game::Loto, false, "réduit"), "réduit");
        assert_eq!(agree(Game::Loto, true, "réduit"), "réduits");
        assert_eq!(agree(Game::Loto, false, ""), "");
        assert_eq!(field("Coût"), "Coût         : ");
        assert_eq!(field("Numéros chance (3)"), "Numéros chance (3) : ");
        assert_eq!(one_in(1.0 / 176_563.0), "1 sur 176 563");
    }

    #[test]
    fn tickets_and_rules() {
        let g = Grid::new(&[1, 2, 3, 4, 5]).unwrap();
        let e = ExtraSet::from_numbers(&[9, 2]);
        assert_eq!(
            ticket(Game::EuroMillions, g, Some(e)),
            "01,02,03,04,05 ★ 02 09"
        );
        assert_eq!(
            ticket(Game::Loto, g, Some(ExtraSet::from_numbers(&[7]))),
            "01,02,03,04,05 + chance 07"
        );
        assert_eq!(ticket(Game::Loto, g, None), "01,02,03,04,05");
        assert_eq!(
            ticket_colored(Game::EuroMillions, g, Some(e), true),
            "01,02,03,04,05 \x1b[1;35m★ 02 09\x1b[0m"
        );
        assert_eq!(
            ticket_colored(Game::EuroMillions, g, Some(e), false),
            "01,02,03,04,05 ★ 02 09"
        );
        assert_eq!(ticket_colored(Game::Loto, g, None, true), "01,02,03,04,05");
        assert_eq!(
            game_rules(Game::Loto),
            "Loto (5 numéros parmi 49, 1 numéro chance parmi 10)"
        );
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
