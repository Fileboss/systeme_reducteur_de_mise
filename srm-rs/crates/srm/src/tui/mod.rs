//! Interface interactive dans le terminal (ratatui).

pub mod app;
mod ui;

use ratatui::crossterm::event::{self, Event, KeyEventKind};

use app::App;

pub fn run() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    let result = (|| -> anyhow::Result<()> {
        let mut app = App::default();
        while !app.quit {
            terminal.draw(|f| ui::draw(f, &app))?;
            if let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press
            {
                app.on_key(key);
            }
        }
        Ok(())
    })();
    ratatui::restore();
    result
}
