use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use eoflint::lint_files;
use std::{path::PathBuf, process::exit};

#[derive(Debug, Parser)]
#[command(about, version)]
struct Opt {
    /// Target files
    pub files: Vec<PathBuf>,
}

fn run() -> Result<()> {
    let opt = Opt::parse();
    if !lint_files(opt.files)? {
        exit(1);
    }
    Ok(())
}

fn main() {
    env_logger::init();
    if let Err(e) = run() {
        eprintln!("{} {:#}", "error:".red().bold(), e);
        exit(2);
    }
}
