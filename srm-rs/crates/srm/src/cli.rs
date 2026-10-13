use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use anyhow::{Context, bail};
use clap::{Args, Parser, Subcommand};
use srm_core::generate::{GenConfig, generate};
use srm_core::grid::parse_numbers;
use srm_core::pricing::{self, DEFAULT_GRID_PRICE, DEFAULT_MARGIN};
use srm_core::report::{self, Report, Settings};
use srm_core::space::{MAX_N, MAX_T, MIN_N, MIN_T};
use srm_core::{Table, cover};

use crate::format;

#[derive(Parser, Debug)]
#[command(
    name = "srm",
    version,
    about = "Système réducteur de mise pour le Loto (5 numéros parmi 50)"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Args, Debug, Clone, Copy)]
pub struct Money {
    /// Prix d'une grille, en euros
    #[arg(long, default_value_t = DEFAULT_GRID_PRICE)]
    pub prix: f64,
    /// Taux de retour aux joueurs utilisé pour la perte attendue (0 à 1, approximatif)
    #[arg(long, default_value_t = srm_core::odds::DEFAULT_RETURN_RATE)]
    pub retour: f64,
}

impl Money {
    fn settings(self) -> Settings {
        Settings {
            grid_price: self.prix,
            return_rate: self.retour,
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
        money: Money,
        /// Enregistre les grilles dans ce dossier
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
        #[arg(long, default_value_t = DEFAULT_GRID_PRICE)]
        prix: f64,
    },
    /// Compare le système à des grilles prises au hasard, sur tous les tirages possibles
    Stats {
        #[arg(required = true, num_args = 1..)]
        numeros: Vec<String>,
        #[arg(short, long, default_value_t = 3)]
        garantie: u8,
        #[command(flatten)]
        money: Money,
        /// Graine des grilles aléatoires de comparaison
        #[arg(long, default_value_t = 1)]
        seed: u64,
    },
    /// Liste les tables disponibles et leur prix
    Tables {
        #[arg(long, default_value_t = DEFAULT_GRID_PRICE)]
        prix: f64,
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

pub fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Command::Play {
            numeros,
            garantie,
            money,
            save,
            json,
        } => {
            let r = report::build(&parse_selection(&numeros)?, garantie, money.settings())?;
            if json {
                println!("{}", serde_json::to_string_pretty(&r)?);
            } else {
                for line in format::summary(&r) {
                    println!("{line}");
                }
                println!();
                for (i, g) in r.grids.iter().enumerate() {
                    println!("{:>4}. {g}", i + 1);
                }
            }
            if let Some(dir) = save {
                let path = save_report(&r, &dir)?;
                eprintln!("Grilles enregistrées dans {}", path.display());
            }
        }
        Command::Suggest { mise, marge, prix } => {
            let offers = pricing::suggest(&pricing::catalog(prix), mise, marge);
            if offers.is_empty() {
                println!("Aucun système dans cette gamme de prix.");
            }
            for o in offers {
                println!(
                    "{:>2} numéros, garantie {} : {:>5} grilles, {}",
                    o.n,
                    o.t,
                    o.grids,
                    format::euros(o.price)
                );
            }
        }
        Command::Stats {
            numeros,
            garantie,
            money,
            seed,
        } => {
            let r = report::build(&parse_selection(&numeros)?, garantie, money.settings())?;
            for line in format::summary(&r) {
                println!("{line}");
            }
            println!();
            for line in format::comparison(&report::compare(&r, seed)?) {
                println!("{line}");
            }
        }
        Command::Tables { prix } => {
            println!("{:>3} {:>3} {:>7} {:>12}", "n", "t", "grilles", "prix");
            for o in pricing::catalog(prix) {
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
                generate_one(n, t, &cfg, &out, force)?;
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

fn generate_one(n: usize, t: u8, cfg: &GenConfig, out: &Path, force: bool) -> anyhow::Result<()> {
    let path = out.join(format!("{n}_{t}.txt"));
    let existing = fs::read_to_string(&path)
        .ok()
        .and_then(|text| Table::parse(n, t, &text).ok());
    let start = Instant::now();
    let generated = generate(n, t, cfg)?;
    let size = generated.table.len();
    let status = match &existing {
        Some(old) if old.len() <= size && !force => format!("conservée ({} grilles)", old.len()),
        _ => {
            let header = format!(
                "srm : couverture de 1..{n}, garantie {t}\n{size} grilles (glouton : {}), graine {}, vérifiée par force brute",
                generated.greedy_size, generated.seed
            );
            fs::write(&path, generated.table.to_text(Some(&header)))?;
            match existing {
                Some(old) => format!("écrite ({} → {size} grilles)", old.len()),
                None => "écrite".to_string(),
            }
        }
    };
    println!(
        "{n:>2}/{t} : glouton {:>5}, final {size:>5} en {:.1} s, {status}",
        generated.greedy_size,
        start.elapsed().as_secs_f64()
    );
    Ok(())
}

/// Enregistre les grilles dans `dir/AAAA-MM-JJ_HH-MM-SS.txt` (jamais en ajout).
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
    for g in &r.grids {
        text.push_str(&g.to_string());
        text.push('\n');
    }
    fs::write(&path, text)?;
    Ok(path)
}
