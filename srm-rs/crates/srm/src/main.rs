mod cli;
mod format;
mod tui;
mod web;

use clap::Parser;

fn main() -> anyhow::Result<()> {
    cli::run(cli::Cli::parse())
}
