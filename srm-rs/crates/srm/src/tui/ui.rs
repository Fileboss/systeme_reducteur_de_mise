//! Rendu de l'interface terminal à partir de l'état [`App`].

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph, Wrap};
use srm_core::game::Game;

use super::app::{App, COLS, Field, Focus, Screen};
use crate::format;

const ACCENT: Color = Color::Cyan;
const PICKED: Color = Color::Yellow;
/// Couleur des compléments (étoiles, numéro chance), partout dans la TUI.
const EXTRA: Color = Color::Magenta;

pub fn draw(frame: &mut Frame, app: &App) {
    match app.screen {
        Screen::Warning => draw_warning(frame),
        Screen::Main => draw_main(frame, app),
    }
}

fn draw_warning(frame: &mut Frame) {
    let text = vec![
        Line::from("Avertissement".bold()),
        Line::from(""),
        Line::from("Les jeux d'argent sont interdits aux mineurs."),
        Line::from("Un système réducteur ne change pas l'espérance de gain :"),
        Line::from("en moyenne, on perd environ la moitié de ce que l'on mise."),
        Line::from("Jouer comporte des risques : endettement, dépendance…"),
        Line::from("Appelez le 09 74 75 13 13 (Joueurs Info Service, appel non surtaxé)."),
        Line::from(""),
        Line::from("Avez-vous plus de 18 ans et souhaitez-vous continuer ? (o/n)".fg(ACCENT)),
    ];
    let area = centered(frame.area(), 72, 11);
    frame.render_widget(
        Paragraph::new(text)
            .centered()
            .wrap(Wrap { trim: true })
            .block(Block::bordered()),
        area,
    );
}

fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let [v] = Layout::vertical([Constraint::Length(height)])
        .flex(ratatui::layout::Flex::Center)
        .areas(area);
    let [h] = Layout::horizontal([Constraint::Length(width)])
        .flex(ratatui::layout::Flex::Center)
        .areas(v);
    h
}

fn draw_main(frame: &mut Frame, app: &App) {
    let [top, summary, bottom, footer] = Layout::vertical([
        Constraint::Length(9),
        Constraint::Length(11),
        Constraint::Min(6),
        Constraint::Length(1),
    ])
    .areas(frame.area());
    let [numbers, params] =
        Layout::horizontal([Constraint::Length(34), Constraint::Min(30)]).areas(top);
    let [grids, side] =
        Layout::horizontal([Constraint::Length(34), Constraint::Min(30)]).areas(bottom);

    draw_numbers(frame, app, numbers);
    draw_params(frame, app, params);
    draw_summary(frame, app, summary);
    draw_grids(frame, app, grids);
    draw_side(frame, app, side);
    draw_footer(frame, app, footer);
}

fn focus_block(title: String, focused: bool) -> Block<'static> {
    let block = Block::bordered().title(title);
    if focused {
        block.border_style(Style::new().fg(ACCENT))
    } else {
        block
    }
}

/// Une case de numéro ; `picked` donne la couleur de fond si elle est choisie.
fn cell(n: u8, picked: Option<Color>, under_cursor: bool) -> Span<'static> {
    let mut style = Style::new();
    if let Some(bg) = picked {
        style = style.fg(Color::Black).bg(bg);
    }
    if under_cursor {
        style = style.add_modifier(Modifier::REVERSED | Modifier::BOLD);
    }
    Span::styled(format!("{n:02}"), style)
}

fn draw_numbers(frame: &mut Frame, app: &App, area: Rect) {
    let game = app.game();
    let pool = game.pool();
    let mut lines: Vec<Line> = (0..pool.div_ceil(COLS))
        .map(|row| {
            let spans: Vec<Span> = (1..=COLS)
                .map(|col| row * COLS + col)
                .filter(|&n| n <= pool)
                .map(|n| {
                    cell(
                        n,
                        app.selected.contains(&n).then_some(PICKED),
                        n == app.cursor && app.focus == Focus::Numbers,
                    )
                })
                .flat_map(|s| [s, Span::raw(" ")])
                .collect();
            Line::from(spans)
        })
        .collect();
    // Ligne des compléments : étoiles ou numéros chance.
    lines.push(Line::from(""));
    let mut extras: Vec<Span> = vec![match game {
        Game::EuroMillions => "★ ".fg(EXTRA),
        Game::Loto => "chance ".fg(EXTRA),
    }];
    extras.extend(
        (1..=game.extra_pool())
            .map(|e| {
                cell(
                    e,
                    app.extras.contains(&e).then_some(EXTRA),
                    e == app.extra_cursor && app.focus == Focus::Extras,
                )
            })
            .flat_map(|s| [s, Span::raw(" ")]),
    );
    lines.push(Line::from(extras));
    let title = format!(
        " Numéros ({}) · {} ({}) ",
        app.selected.len(),
        game.extra_plural(),
        app.extras.len()
    );
    frame.render_widget(
        Paragraph::new(lines).block(focus_block(
            title,
            matches!(app.focus, Focus::Numbers | Focus::Extras),
        )),
        area,
    );
}

fn draw_params(frame: &mut Frame, app: &App, area: Rect) {
    let budget = app.budget.map_or("—".to_string(), format::euros);
    let game = app.game();
    let extras_mode = if app.reduce_extras {
        format!(
            "réduits, garantie {} {}",
            app.extra_t,
            format::plural(game, app.extra_t.into())
        )
    } else {
        format!(
            "fixes ({} {})",
            game.extras_per_grid(),
            format::plural(game, game.extras_per_grid().into())
        )
    };
    let lines = vec![
        Line::from(vec![
            "Jeu          ".into(),
            game.name().bold(),
            "  (j)".dark_gray(),
        ]),
        Line::from(vec![
            "Garantie     ".into(),
            format!("{} bons numéros", app.t).bold(),
            "  (+/-)".dark_gray(),
        ]),
        Line::from(vec![
            "Compléments  ".into(),
            extras_mode.bold(),
            if app.reduce_extras {
                "  (r, [ ])"
            } else {
                "  (r)"
            }
            .dark_gray(),
        ]),
        Line::from(vec![
            "Mise visée   ".into(),
            budget.bold(),
            "  (m)".dark_gray(),
        ]),
        Line::from(vec![
            "Prix grille  ".into(),
            format::euros(app.settings.grid_price).bold(),
            "  (p)".dark_gray(),
        ]),
    ];
    frame.render_widget(
        Paragraph::new(lines).block(Block::bordered().title(" Paramètres ")),
        area,
    );
}

fn draw_summary(frame: &mut Frame, app: &App, area: Rect) {
    let lines: Vec<Line> = match &app.report {
        Ok(r) => format::summary(r).into_iter().map(Line::from).collect(),
        Err(e) => vec![Line::from(e.clone().fg(Color::Yellow))],
    };
    frame.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .block(Block::bordered().title(" Résumé ")),
        area,
    );
}

fn draw_grids(frame: &mut Frame, app: &App, area: Rect) {
    // Seuls les tickets visibles sont mis en forme : il peut y en avoir des centaines de milliers.
    let visible = usize::from(area.height.saturating_sub(2));
    let (title, lines) = match &app.report {
        Ok(r) => (
            format!(" Tickets ({}) ", r.tickets),
            r.tickets()
                .enumerate()
                .skip(app.scroll)
                .take(visible)
                .map(|(i, (g, e))| {
                    let mut spans = vec![Span::raw(format!("{:>5}. {g}", i + 1))];
                    if let Some(e) = e {
                        spans.push(Span::raw(if r.game == Game::Loto { " + " } else { " " }));
                        spans.push(format::extras_label(r.game, e).fg(EXTRA).bold());
                    }
                    Line::from(spans)
                })
                .collect(),
        ),
        Err(_) => (" Tickets ".to_string(), Vec::new()),
    };
    frame.render_widget(
        Paragraph::new(lines).block(focus_block(title, app.focus == Focus::Grids)),
        area,
    );
}

fn draw_side(frame: &mut Frame, app: &App, area: Rect) {
    if let Some(c) = &app.comparison {
        let lines: Vec<Line> = format::comparison(c).into_iter().map(Line::from).collect();
        frame.render_widget(
            Paragraph::new(lines)
                .wrap(Wrap { trim: false })
                .block(Block::bordered().title(" Système contre hasard ")),
            area,
        );
        return;
    }
    let lines: Vec<Line> = match app.budget {
        None => vec![
            Line::from("Saisissez une mise (m) pour voir les systèmes"),
            Line::from("compatibles, ou « x » pour comparer le système"),
            Line::from("à des grilles prises au hasard."),
        ],
        Some(_) if app.suggestions.is_empty() => {
            vec![Line::from("Aucun système dans cette gamme de prix.")]
        }
        Some(_) => app
            .suggestions
            .iter()
            .map(|o| Line::from(format::offer(o)))
            .collect(),
    };
    frame.render_widget(
        Paragraph::new(lines).block(Block::bordered().title(" Suggestions (a : appliquer) ")),
        area,
    );
}

fn draw_footer(frame: &mut Frame, app: &App, area: Rect) {
    let line = if let Some(input) = &app.input {
        let label = match input.field {
            Field::Budget => "Mise visée",
            Field::GridPrice => "Prix d'une grille",
        };
        Line::from(vec![
            format!("{label} (€) : ").fg(ACCENT),
            input.buffer.clone().bold(),
            "▏  Entrée : valider, Échap : annuler".dark_gray(),
        ])
    } else if let Some(msg) = &app.message {
        Line::from(msg.clone().fg(Color::Yellow))
    } else {
        Line::from(
            "←↑↓→ déplacer  espace choisir  e compléments  j jeu  tab tickets  x comparer  s enregistrer  c vider  q quitter"
                .dark_gray(),
        )
    };
    frame.render_widget(Paragraph::new(line), area);
}

#[cfg(test)]
mod tests {
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    use super::*;

    fn render(app: &App) -> String {
        let mut terminal = Terminal::new(TestBackend::new(100, 40)).unwrap();
        terminal.draw(|f| draw(f, app)).unwrap();
        let buf = terminal.backend().buffer().clone();
        buf.content().iter().map(|c| c.symbol()).collect()
    }

    #[test]
    fn renders_warning_and_main_screens() {
        let mut app = App::default();
        assert!(render(&app).contains("18 ans"));
        app.screen = Screen::Main;
        for n in 1..=12 {
            app.toggle(n);
        }
        app.budget = Some(100.0);
        app.refresh();
        let screen = render(&app);
        assert!(screen.contains("Numéros (12)"));
        assert!(screen.contains("Tickets ("));
        assert!(screen.contains("Garantie"));
    }

    #[test]
    fn renders_on_tiny_terminal_without_panicking() {
        let mut app = App {
            screen: Screen::Main,
            ..Default::default()
        };
        let mut terminal = Terminal::new(TestBackend::new(20, 5)).unwrap();
        terminal.draw(|f| draw(f, &app)).unwrap();
        // Sans sélection valide, la comparaison renvoie un message au lieu de paniquer.
        app.compute_comparison();
        assert!(app.comparison.is_none() && app.message.is_some());
    }

    #[test]
    fn renders_comparison_panel() {
        let mut app = App {
            screen: Screen::Main,
            ..Default::default()
        };
        for n in 1..=12 {
            app.toggle(n);
        }
        app.compute_comparison();
        assert!(render(&app).contains("Système contre hasard"));
    }

    #[test]
    fn ticket_extras_are_colored() {
        let mut app = App {
            screen: Screen::Main,
            ..Default::default()
        };
        for n in 1..=12 {
            app.toggle(n);
        }
        app.toggle_extra(2);
        app.toggle_extra(9);
        let mut terminal = Terminal::new(TestBackend::new(110, 40)).unwrap();
        terminal.draw(|f| draw(f, &app)).unwrap();
        let buf = terminal.backend().buffer().clone();
        let width = usize::from(buf.area.width);
        let cells = buf.content();
        // Retrouver « ★ 02 09 » dans la liste des tickets et vérifier la couleur de chaque caractère.
        let text: Vec<&str> = cells.iter().map(|c| c.symbol()).collect();
        let star = (0..cells.len())
            .find(|&i| {
                text[i] == "★" && text.get(i + 2) == Some(&"0") && text.get(i + 3) == Some(&"2")
            })
            .expect("compléments du premier ticket");
        assert!(
            star / width > 20,
            "dans la liste des tickets, pas dans le sélecteur"
        );
        for c in &cells[star..star + 7] {
            assert_eq!(c.fg, EXTRA, "« {} » devrait être en couleur", c.symbol());
        }
        // Les numéros de la grille restent sans couleur.
        assert_ne!(cells[star - 2].fg, EXTRA);
    }
}
