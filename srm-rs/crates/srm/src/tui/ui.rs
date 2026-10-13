//! Rendu de l'interface terminal à partir de l'état [`App`].

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph, Wrap};
use srm_core::combinatorics::MAX_NUMBER;

use super::app::{App, COLS, Field, Focus, Screen};
use crate::format;

const ACCENT: Color = Color::Cyan;
const PICKED: Color = Color::Yellow;

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
        Constraint::Length(7),
        Constraint::Length(8),
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

fn draw_numbers(frame: &mut Frame, app: &App, area: Rect) {
    let lines: Vec<Line> = (0..MAX_NUMBER / COLS)
        .map(|row| {
            let spans: Vec<Span> = (1..=COLS)
                .map(|col| {
                    let n = row * COLS + col;
                    let mut style = Style::new();
                    if app.selected.contains(&n) {
                        style = style.fg(Color::Black).bg(PICKED);
                    }
                    if n == app.cursor && app.focus == Focus::Numbers {
                        style = style.add_modifier(Modifier::REVERSED | Modifier::BOLD);
                    }
                    Span::styled(format!("{n:02}"), style)
                })
                .flat_map(|s| [s, Span::raw(" ")])
                .collect();
            Line::from(spans)
        })
        .collect();
    let title = format!(" Numéros ({}) ", app.selected.len());
    frame.render_widget(
        Paragraph::new(lines).block(focus_block(title, app.focus == Focus::Numbers)),
        area,
    );
}

fn draw_params(frame: &mut Frame, app: &App, area: Rect) {
    let budget = app.budget.map_or("—".to_string(), format::euros);
    let lines = vec![
        Line::from(vec![
            "Garantie     ".into(),
            format!("{} bons numéros", app.t).bold(),
            "  (+/-)".dark_gray(),
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
        Line::from(vec![
            "Taux retour  ".into(),
            format::percent(app.settings.return_rate).into(),
            "  (approx.)".dark_gray(),
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
    let (title, lines) = match &app.report {
        Ok(r) => (
            format!(" Grilles ({}) ", r.grids.len()),
            r.grids
                .iter()
                .enumerate()
                .map(|(i, g)| Line::from(format!("{:>4}. {g}", i + 1)))
                .collect(),
        ),
        Err(_) => (" Grilles ".to_string(), Vec::new()),
    };
    frame.render_widget(
        Paragraph::new(lines)
            .scroll((app.scroll as u16, 0))
            .block(focus_block(title, app.focus == Focus::Grids)),
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
            .map(|o| {
                Line::from(format!(
                    "{:>2} numéros, garantie {} : {:>4} grilles, {}",
                    o.n,
                    o.t,
                    o.grids,
                    format::euros(o.price)
                ))
            })
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
            "←↑↓→ déplacer  espace choisir  tab grilles  x comparer  s enregistrer  c vider  q quitter"
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
        assert!(screen.contains("Grilles ("));
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
}
