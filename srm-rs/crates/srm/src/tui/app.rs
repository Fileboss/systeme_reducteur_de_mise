//! État de l'interface terminal et réaction aux touches, sans aucun rendu.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use srm_core::combinatorics::MAX_NUMBER;
use srm_core::pricing::{self, DEFAULT_MARGIN, Offer};
use srm_core::report::{self, Comparison, Report, Settings};
use srm_core::space::{MAX_N, MAX_T, MIN_N, MIN_T};

/// Nombre de colonnes de la grille des numéros (5 lignes × 10 colonnes).
pub const COLS: u8 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Warning,
    Main,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Numbers,
    Grids,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Field {
    Budget,
    GridPrice,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input {
    pub field: Field,
    pub buffer: String,
}

pub struct App {
    pub screen: Screen,
    pub focus: Focus,
    /// Numéro sous le curseur, de 1 à 50.
    pub cursor: u8,
    pub selected: BTreeSet<u8>,
    pub t: u8,
    pub budget: Option<f64>,
    pub settings: Settings,
    pub input: Option<Input>,
    pub report: Result<Report, String>,
    pub comparison: Option<Comparison>,
    pub suggestions: Vec<Offer>,
    pub scroll: usize,
    pub message: Option<String>,
    pub save_dir: PathBuf,
    pub quit: bool,
}

impl Default for App {
    fn default() -> Self {
        let mut app = Self {
            screen: Screen::Warning,
            focus: Focus::Numbers,
            cursor: 1,
            selected: BTreeSet::new(),
            t: 3,
            budget: None,
            settings: Settings::default(),
            input: None,
            report: Err(String::new()),
            comparison: None,
            suggestions: Vec::new(),
            scroll: 0,
            message: None,
            save_dir: PathBuf::from("resultats"),
            quit: false,
        };
        app.refresh();
        app
    }
}

impl App {
    pub fn selection(&self) -> Vec<u8> {
        self.selected.iter().copied().collect()
    }

    /// Recalcule le rapport et les suggestions après tout changement.
    pub fn refresh(&mut self) {
        let n = self.selected.len();
        self.comparison = None;
        self.scroll = 0;
        self.report = if !(MIN_N..=MAX_N).contains(&n) {
            Err(format!(
                "Choisissez entre {MIN_N} et {MAX_N} numéros ({n} sélectionnés)"
            ))
        } else {
            report::build(&self.selection(), self.t, self.settings).map_err(|e| e.to_string())
        };
        self.suggestions = match self.budget {
            Some(b) => pricing::suggest(
                &pricing::catalog(self.settings.grid_price),
                b,
                DEFAULT_MARGIN,
            ),
            None => Vec::new(),
        };
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            self.quit = true;
            return;
        }
        match self.screen {
            Screen::Warning => match key.code {
                KeyCode::Char('o' | 'O') => self.screen = Screen::Main,
                _ => self.quit = true,
            },
            Screen::Main if self.input.is_some() => self.on_input_key(key.code),
            Screen::Main => self.on_main_key(key.code),
        }
    }

    fn on_main_key(&mut self, code: KeyCode) {
        self.message = None;
        match code {
            KeyCode::Char('q') | KeyCode::Esc => self.quit = true,
            KeyCode::Tab => {
                self.focus = match self.focus {
                    Focus::Numbers => Focus::Grids,
                    Focus::Grids => Focus::Numbers,
                }
            }
            KeyCode::Char('+') | KeyCode::Char('=') => self.set_t(self.t + 1),
            KeyCode::Char('-') => self.set_t(self.t.saturating_sub(1)),
            KeyCode::Char('m') => self.start_input(Field::Budget),
            KeyCode::Char('p') => self.start_input(Field::GridPrice),
            KeyCode::Char('c') => {
                self.selected.clear();
                self.refresh();
                self.message = Some("Sélection vidée".into());
            }
            KeyCode::Char('a') => self.apply_first_suggestion(),
            KeyCode::Char('x') => self.compute_comparison(),
            KeyCode::Char('s') => self.save(),
            _ => match self.focus {
                Focus::Numbers => self.on_numbers_key(code),
                Focus::Grids => self.on_grids_key(code),
            },
        }
    }

    fn on_numbers_key(&mut self, code: KeyCode) {
        let c = self.cursor;
        self.cursor = match code {
            KeyCode::Left if !(c - 1).is_multiple_of(COLS) => c - 1,
            KeyCode::Right if (c - 1) % COLS < COLS - 1 => c + 1,
            KeyCode::Up if c > COLS => c - COLS,
            KeyCode::Down if c + COLS <= MAX_NUMBER => c + COLS,
            KeyCode::Char(' ') | KeyCode::Enter => {
                self.toggle(c);
                c
            }
            _ => c,
        };
    }

    fn on_grids_key(&mut self, code: KeyCode) {
        let len = self.report.as_ref().map(|r| r.grids.len()).unwrap_or(0);
        let max = len.saturating_sub(1);
        self.scroll = match code {
            KeyCode::Up => self.scroll.saturating_sub(1),
            KeyCode::Down => (self.scroll + 1).min(max),
            KeyCode::PageUp => self.scroll.saturating_sub(20),
            KeyCode::PageDown => (self.scroll + 20).min(max),
            KeyCode::Home => 0,
            KeyCode::End => max,
            _ => self.scroll,
        };
    }

    pub fn toggle(&mut self, number: u8) {
        if !self.selected.remove(&number) {
            if self.selected.len() >= MAX_N {
                self.message = Some(format!("{MAX_N} numéros au maximum"));
                return;
            }
            self.selected.insert(number);
        }
        self.refresh();
    }

    pub fn set_t(&mut self, t: u8) {
        let t = t.clamp(MIN_T, MAX_T);
        if t != self.t {
            self.t = t;
            self.refresh();
        }
    }

    fn start_input(&mut self, field: Field) {
        self.input = Some(Input {
            field,
            buffer: String::new(),
        });
    }

    fn on_input_key(&mut self, code: KeyCode) {
        let Some(input) = self.input.as_mut() else {
            return;
        };
        match code {
            KeyCode::Esc => self.input = None,
            KeyCode::Backspace => {
                input.buffer.pop();
            }
            KeyCode::Char(ch) if ch.is_ascii_digit() || ch == ',' || ch == '.' => {
                input.buffer.push(ch);
            }
            KeyCode::Enter => {
                let input = self.input.take().unwrap();
                match input.buffer.replace(',', ".").parse::<f64>() {
                    Ok(v) if v > 0.0 && v.is_finite() => {
                        match input.field {
                            Field::Budget => self.budget = Some(v),
                            Field::GridPrice => self.settings.grid_price = v,
                        }
                        self.refresh();
                    }
                    _ => self.message = Some(format!("Montant invalide : « {} »", input.buffer)),
                }
            }
            _ => {}
        }
    }

    /// Ajuste la garantie et complète la sélection pour coller à la première suggestion.
    fn apply_first_suggestion(&mut self) {
        let Some(o) = self.suggestions.first().copied() else {
            self.message = Some("Aucune suggestion : saisissez une mise avec « m »".into());
            return;
        };
        self.t = o.t;
        if self.selected.len() > o.n {
            self.message = Some(format!(
                "Suggestion : {} numéros, garantie {} — retirez {} numéro(s)",
                o.n,
                o.t,
                self.selected.len() - o.n
            ));
        } else {
            self.message = Some(format!(
                "Suggestion : {} numéros, garantie {} — encore {} numéro(s) à choisir",
                o.n,
                o.t,
                o.n - self.selected.len()
            ));
        }
        self.refresh();
    }

    pub fn compute_comparison(&mut self) {
        match &self.report {
            Ok(r) => match report::compare(r, 1) {
                Ok(c) => self.comparison = Some(c),
                Err(e) => self.message = Some(e.to_string()),
            },
            Err(e) => self.message = Some(e.clone()),
        }
    }

    fn save(&mut self) {
        self.message = Some(match &self.report {
            Ok(r) => match crate::cli::save_report(r, &self.save_dir) {
                Ok(path) => format!("Grilles enregistrées dans {}", path.display()),
                Err(e) => format!("Échec de l'enregistrement : {e}"),
            },
            Err(e) => e.clone(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn press(app: &mut App, code: KeyCode) {
        app.on_key(KeyEvent::new(code, KeyModifiers::NONE));
    }

    fn main_app() -> App {
        let mut app = App::default();
        press(&mut app, KeyCode::Char('o'));
        app
    }

    #[test]
    fn warning_must_be_accepted() {
        let mut app = App::default();
        press(&mut app, KeyCode::Char('n'));
        assert!(app.quit);
        assert_eq!(main_app().screen, Screen::Main);
    }

    #[test]
    fn cursor_moves_and_stays_in_bounds() {
        let mut app = main_app();
        press(&mut app, KeyCode::Left);
        press(&mut app, KeyCode::Up);
        assert_eq!(app.cursor, 1);
        press(&mut app, KeyCode::Right);
        press(&mut app, KeyCode::Down);
        assert_eq!(app.cursor, 12);
        for _ in 0..20 {
            press(&mut app, KeyCode::Down);
            press(&mut app, KeyCode::Right);
        }
        assert_eq!(app.cursor, 50);
        // Pas de passage d'une ligne à l'autre avec ← →.
        app.cursor = 11;
        press(&mut app, KeyCode::Left);
        assert_eq!(app.cursor, 11);
    }

    #[test]
    fn toggling_numbers_updates_report() {
        let mut app = main_app();
        assert!(app.report.is_err());
        for n in 1..=12u8 {
            app.cursor = n;
            press(&mut app, KeyCode::Char(' '));
        }
        assert_eq!(app.selected.len(), 12);
        let grids = app.report.as_ref().map(|r| r.grids.len());
        assert!(grids.is_ok(), "{:?}", app.report.as_ref().err());
        press(&mut app, KeyCode::Char(' '));
        assert_eq!(app.selected.len(), 11);
        press(&mut app, KeyCode::Char('c'));
        assert!(app.selected.is_empty());
    }

    #[test]
    fn selection_is_capped() {
        let mut app = main_app();
        for n in 1..=31 {
            app.toggle(n);
        }
        assert_eq!(app.selected.len(), MAX_N);
        assert!(app.message.is_some());
    }

    #[test]
    fn guarantee_is_clamped() {
        let mut app = main_app();
        for _ in 0..5 {
            press(&mut app, KeyCode::Char('+'));
        }
        assert_eq!(app.t, MAX_T);
        for _ in 0..5 {
            press(&mut app, KeyCode::Char('-'));
        }
        assert_eq!(app.t, MIN_T);
    }

    #[test]
    fn budget_input_accepts_comma_and_rejects_garbage() {
        let mut app = main_app();
        press(&mut app, KeyCode::Char('m'));
        for ch in "150,5".chars() {
            press(&mut app, KeyCode::Char(ch));
        }
        press(&mut app, KeyCode::Char('z'));
        press(&mut app, KeyCode::Enter);
        assert_eq!(app.budget, Some(150.5));
        assert!(app.input.is_none());

        press(&mut app, KeyCode::Char('p'));
        press(&mut app, KeyCode::Enter);
        assert!(app.message.as_deref().unwrap().contains("invalide"));
        assert_eq!(
            app.settings.grid_price,
            srm_core::pricing::DEFAULT_GRID_PRICE
        );
    }

    #[test]
    fn grids_scroll_is_bounded() {
        let mut app = main_app();
        for n in 1..=20 {
            app.toggle(n);
        }
        press(&mut app, KeyCode::Tab);
        press(&mut app, KeyCode::End);
        let len = app.report.as_ref().unwrap().grids.len();
        assert_eq!(app.scroll, len - 1);
        press(&mut app, KeyCode::Down);
        assert_eq!(app.scroll, len - 1);
        press(&mut app, KeyCode::Home);
        assert_eq!(app.scroll, 0);
    }
}
