use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use anyhow::{Context, bail};
use clap::{Args, Parser, Subcommand};
use srm_core::extras::{ExtraMode, combo_count};
use srm_core::game::Game;
use srm_core::generate::{GenConfig, generate};
use srm_core::grid::parse_numbers;
use srm_core::pricing::{self, DEFAULT_MARGIN};
use srm_core::report::{self, Report, Settings};
use srm_core::space::{MAX_N, MAX_T, MIN_N, MIN_T};
use srm_core::{Table, cover};

use crate::format;

#[derive(Parser, Debug)]
#[command(
    name = "srm",
    version,
    about = "Système réducteur de mise pour l'EuroMillions et le Loto"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

/// Jeu et prix d'une grille.
#[derive(Args, Debug, Clone, Copy)]
pub struct GameArgs {
    /// Jeu : euromillions (5 parmi 50 + 2 étoiles parmi 12) ou loto (5 parmi 49 + 1 numéro chance parmi 10)
    #[arg(long, default_value = "euromillions")]
    pub jeu: Game,
    /// Prix d'une grille, en euros (défaut : 2,50 € à l'EuroMillions, 2,20 € au Loto)
    #[arg(long)]
    pub prix: Option<f64>,
}

impl GameArgs {
    pub fn grid_price(self) -> f64 {
        self.prix.unwrap_or(self.jeu.default_price())
    }
}

#[derive(Args, Debug, Clone, Copy)]
pub struct Money {
    #[command(flatten)]
    pub game: GameArgs,
    /// Taux de retour aux joueurs utilisé pour la perte attendue (0 à 1, approximatif)
    #[arg(long, default_value_t = srm_core::odds::DEFAULT_RETURN_RATE)]
    pub retour: f64,
}

impl Money {
    fn settings(self) -> Settings {
        Settings {
            game: self.game.jeu,
            grid_price: self.game.grid_price(),
            return_rate: self.retour,
        }
    }
}

/// Numéros complémentaires : étoiles (EuroMillions) ou numéros chance (Loto).
#[derive(Args, Debug, Clone, Default)]
pub struct ExtrasArgs {
    /// Vos étoiles (EuroMillions) ou numéros chance (Loto), par exemple 2,9. Sans --reduire-extra,
    /// exactement ceux d'une grille (2 étoiles ou 1 chance), joués sur toutes les grilles.
    #[arg(short = 'e', long, value_name = "NUMÉROS")]
    pub extra: Option<String>,
    /// Réduit aussi les compléments : vous en choisissez davantage, avec une garantie dédiée
    #[arg(long)]
    pub reduire_extra: bool,
    /// Compléments garantis en mode réduit (0 à 2 étoiles, 0 à 1 chance ; défaut : 1)
    #[arg(long, requires = "reduire_extra")]
    pub garantie_extra: Option<u8>,
}

impl ExtrasArgs {
    pub fn numbers(&self) -> anyhow::Result<Vec<u8>> {
        Ok(match &self.extra {
            Some(s) => parse_numbers(s)?,
            None => Vec::new(),
        })
    }

    pub fn mode(&self) -> ExtraMode {
        if self.reduire_extra {
            ExtraMode::Reduced {
                t: self.garantie_extra.unwrap_or(1),
            }
        } else {
            ExtraMode::Fixed
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Donne les grilles à jouer pour vos numéros
    Play {
        /// Vos numéros (entre 6 et 30), séparés par des espaces ou des virgules
        #[arg(required = true, num_args = 1..)]
        numeros: Vec<String>,
        /// Nombre de bons numéros garantis (2 à 4)
        #[arg(short, long, default_value_t = 3)]
        garantie: u8,
        #[command(flatten)]
        extras: ExtrasArgs,
        #[command(flatten)]
        money: Money,
        /// Enregistre les tickets dans ce dossier
        #[arg(long, value_name = "DOSSIER", num_args = 0..=1, default_missing_value = "resultats")]
        save: Option<PathBuf>,
        /// Sortie JSON
        #[arg(long)]
        json: bool,
    },
    /// Suggère des systèmes selon votre mise
    Suggest {
        /// Mise visée, en euros
        #[arg(short, long)]
        mise: f64,
        /// Marge acceptée autour de la mise (1.15 = ±15 %)
        #[arg(long, default_value_t = DEFAULT_MARGIN)]
        marge: f64,
        #[command(flatten)]
        game: GameArgs,
        #[command(flatten)]
        extras: ExtrasArgs,
    },
    /// Compare le système à des grilles prises au hasard, sur tous les tirages possibles
    Stats {
        #[arg(required = true, num_args = 1..)]
        numeros: Vec<String>,
        #[arg(short, long, default_value_t = 3)]
        garantie: u8,
        #[command(flatten)]
        extras: ExtrasArgs,
        #[command(flatten)]
        money: Money,
        /// Graine des grilles aléatoires de comparaison
        #[arg(long, default_value_t = 1)]
        seed: u64,
    },
    /// Liste les tables disponibles et leur prix
    Tables {
        #[command(flatten)]
        game: GameArgs,
    },
    /// Calcule des tables (glouton puis recuit simulé)
    Generate {
        /// Taille du système (nombre de numéros)
        #[arg(short, long, required_unless_present = "all")]
        n: Option<usize>,
        /// Garantie
        #[arg(short, long, required_unless_present = "all")]
        t: Option<u8>,
        /// Toutes les tables (n de 6 à 30, garantie de 2 à 4)
        #[arg(long, conflicts_with_all = ["n", "t"])]
        all: bool,
        /// Durée de recherche par table, en secondes
        #[arg(long, default_value_t = 10.0)]
        budget: f64,
        #[arg(long, default_value_t = 1)]
        seed: u64,
        /// Exécutions parallèles indépendantes (par défaut : nombre de cœurs)
        #[arg(long)]
        runs: Option<usize>,
        /// Dossier des tables
        #[arg(long, default_value = default_tables_dir())]
        out: PathBuf,
        /// Écrase la table existante même si la nouvelle n'est pas plus petite
        #[arg(long)]
        force: bool,
        /// Repart de la table existante au lieu du glouton, pour prolonger une recherche
        #[arg(long)]
        resume: bool,
    },
    /// Vérifie qu'un fichier de grilles couvre bien tous les tirages de 1..n
    Verify {
        fichier: PathBuf,
        #[arg(short, long)]
        n: usize,
        #[arg(short, long)]
        t: u8,
    },
    /// Interface interactive dans le terminal
    Tui,
    /// Interface web de visualisation
    Serve {
        #[arg(long, default_value_t = 8080)]
        port: u16,
        /// Adresse d'écoute
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
    },
}

fn default_tables_dir() -> &'static str {
    concat!(env!("CARGO_MANIFEST_DIR"), "/../srm-core/tables")
}

pub fn parse_selection(args: &[String]) -> anyhow::Result<Vec<u8>> {
    Ok(parse_numbers(&args.join(" "))?)
}

fn build_report(
    numeros: &[String],
    garantie: u8,
    extras: &ExtrasArgs,
    money: Money,
) -> anyhow::Result<Report> {
    Ok(report::build(
        &parse_selection(numeros)?,
        garantie,
        &extras.numbers()?,
        extras.mode(),
        money.settings(),
    )?)
}

pub fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Command::Play {
            numeros,
            garantie,
            extras,
            money,
            save,
            json,
        } => {
            let r = build_report(&numeros, garantie, &extras, money)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&r)?);
            } else {
                for line in format::summary(&r) {
                    println!("{line}");
                }
                println!();
                let color = format::use_color();
                for (i, (g, e)) in r.tickets().enumerate() {
                    println!(
                        "{:>5}. {}",
                        i + 1,
                        format::ticket_colored(r.game, g, e, color)
                    );
                }
            }
            if let Some(dir) = save {
                let path = save_report(&r, &dir)?;
                eprintln!("Tickets enregistrés dans {}", path.display());
            }
        }
        Command::Suggest {
            mise,
            marge,
            game,
            extras,
        } => {
            let picked = extras.numbers()?;
            let combos = if picked.is_empty() {
                1
            } else {
                srm_core::extras::normalize_extras(game.jeu, &picked)?;
                combo_count(game.jeu, picked.len(), extras.mode())?
            };
            let offers =
                pricing::suggest(&pricing::catalog(game.grid_price(), combos), mise, marge);
            if offers.is_empty() {
                println!("Aucun système dans cette gamme de prix.");
            }
            for o in offers {
                println!("{}", format::offer(&o));
            }
        }
        Command::Stats {
            numeros,
            garantie,
            extras,
            money,
            seed,
        } => {
            let r = build_report(&numeros, garantie, &extras, money)?;
            for line in format::summary(&r) {
                println!("{line}");
            }
            println!();
            for line in format::comparison(&report::compare(&r, seed)?) {
                println!("{line}");
            }
        }
        Command::Tables { game } => {
            println!(
                "{} : {} par grille",
                game.jeu,
                format::euros(game.grid_price())
            );
            println!("{:>3} {:>3} {:>7} {:>12}", "n", "t", "grilles", "prix");
            for o in pricing::catalog(game.grid_price(), 1) {
                println!(
                    "{:>3} {:>3} {:>7} {:>12}",
                    o.n,
                    o.t,
                    o.grids,
                    format::euros(o.price)
                );
            }
        }
        Command::Generate {
            n,
            t,
            all,
            budget,
            seed,
            runs,
            out,
            force,
            resume,
        } => {
            let cfg = GenConfig {
                seed,
                runs: runs.unwrap_or_else(rayon_threads),
                max_moves: u64::MAX,
                time_limit: Some(Duration::from_secs_f64(budget)),
                ..Default::default()
            };
            let params: Vec<(usize, u8)> = if all {
                (MIN_T..=MAX_T)
                    .flat_map(|t| (MIN_N..=MAX_N).map(move |n| (n, t)))
                    .collect()
            } else {
                vec![(n.unwrap(), t.unwrap())]
            };
            fs::create_dir_all(&out)?;
            for (n, t) in params {
                generate_one(n, t, &cfg, &out, force, resume)?;
            }
        }
        Command::Verify { fichier, n, t } => {
            let text = fs::read_to_string(&fichier)
                .with_context(|| format!("lecture de {}", fichier.display()))?;
            let table = Table::parse(n, t, &text)?;
            match cover::verify(n, t, table.grids()) {
                Ok(()) => println!(
                    "OK : {} grilles, tous les tirages de 1..{n} sont couverts à {t} bons numéros ou plus",
                    table.len()
                ),
                Err(e) => bail!(
                    "{e} ({} des tirages couverts)",
                    format::percent(cover::coverage_ratio(n, t, table.grids()))
                ),
            }
        }
        Command::Tui => crate::tui::run()?,
        Command::Serve { port, host } => crate::web::serve(&host, port)?,
    }
    Ok(())
}

fn rayon_threads() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}

fn generate_one(
    n: usize,
    t: u8,
    cfg: &GenConfig,
    out: &Path,
    force: bool,
    resume: bool,
) -> anyhow::Result<()> {
    let path = out.join(format!("{n}_{t}.txt"));
    let existing = fs::read_to_string(&path)
        .ok()
        .and_then(|text| Table::parse(n, t, &text).ok());
    let start = Instant::now();
    let origin = if resume && existing.is_some() {
        "reprise"
    } else {
        "glouton"
    };
    let cfg = GenConfig {
        initial: existing
            .as_ref()
            .filter(|_| resume)
            .map(|tb| tb.grids().to_vec()),
        ..cfg.clone()
    };
    let generated = generate(n, t, &cfg)?;
    let size = generated.table.len();
    let status = match &existing {
        Some(old) if old.len() <= size && !force => format!("conservée ({} grilles)", old.len()),
        _ => {
            let header = format!(
                "srm : couverture de 1..{n}, garantie {t}\n{size} grilles (départ {origin} : {}), graine {}, vérifiée par force brute",
                generated.start_size, generated.seed
            );
            fs::write(&path, generated.table.to_text(Some(&header)))?;
            match existing {
                Some(old) => format!("écrite ({} → {size} grilles)", old.len()),
                None => "écrite".to_string(),
            }
        }
    };
    println!(
        "{n:>2}/{t} : {origin} {:>5}, final {size:>5} en {:.1} s, {status}",
        generated.start_size,
        start.elapsed().as_secs_f64()
    );
    Ok(())
}

/// Enregistre les tickets dans `dir/AAAA-MM-JJ_HH-MM-SS.txt` (jamais en ajout).
pub fn save_report(r: &Report, dir: &Path) -> anyhow::Result<PathBuf> {
    fs::create_dir_all(dir)?;
    let path = dir.join(format!(
        "{}.txt",
        chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")
    ));
    let mut text: String = format::summary(r)
        .into_iter()
        .map(|l| format!("# {l}\n"))
        .collect();
    for (g, e) in r.tickets() {
        text.push_str(&format::ticket(r.game, g, e));
        text.push('\n');
    }
    fs::write(&path, text)?;
    Ok(path)
}
