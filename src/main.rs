mod lib;

use anyhow::Result;
use colored::Colorize;
use std::{path::PathBuf, process::exit};
use structopt::clap;
use structopt::StructOpt;

use crate::lib::lint_files;

#[derive(Debug, StructOpt)]
#[structopt(about, global_setting = clap::AppSettings::ColoredHelp)]
struct Opt {
    /// Target files
    #[structopt()]
    pub files: Vec<PathBuf>,
}

fn run() -> Result<()> {
    let opt = Opt::from_args();
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
