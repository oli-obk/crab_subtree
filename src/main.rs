use clap::Parser;
use clap_verbosity_flag::Verbosity;
use color_eyre::eyre::{bail, Result};
use git2::Repository;
use std::path::PathBuf;
use tracing::metadata::LevelFilter;

#[macro_use]
extern crate tracing;

mod processor;

/// A greenfield implementation of what git subtree promises to do but fails to do on large repos
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the repository (will pick current dir otherwise)
    #[clap(long, value_parser)]
    path: Option<PathBuf>,

    /// Path to the subtree within the repository
    #[clap(long, value_parser)]
    prefix: PathBuf,

    /// Verbosity level
    #[clap(flatten)]
    verbose: Verbosity,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let Args {
        path,
        prefix,
        verbose,
    } = Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(convert_filter(verbose.log_level_filter()))
        .init();

    let path = match path {
        Some(path) => path,
        None => std::env::current_dir()?,
    };

    let repo = Repository::open(path)?;
    let mut walker = repo.revwalk()?;
    walker.push_head()?;
    for oid in walker {
        let oid = oid?;
        trace!(?oid);
        let commit = repo.find_commit(oid)?;
        let msg = commit.message().unwrap();
        for line in msg.lines() {
            if let Some(dir) = line.strip_prefix("git-subtree-dir: ") {
                debug!(?oid, %msg, "found git-subtree addition commit");
                if prefix.to_str() == Some(dir) {
                    return processor::process(oid, msg);
                }
            }
        }
    }
    bail!(
        "Did not find subtree addition commit for {}",
        prefix.display()
    );
}

fn convert_filter(filter: log::LevelFilter) -> LevelFilter {
    match filter {
        log::LevelFilter::Off => LevelFilter::OFF,
        log::LevelFilter::Error => LevelFilter::ERROR,
        log::LevelFilter::Warn => LevelFilter::WARN,
        log::LevelFilter::Info => LevelFilter::INFO,
        log::LevelFilter::Debug => LevelFilter::DEBUG,
        log::LevelFilter::Trace => LevelFilter::TRACE,
    }
}
