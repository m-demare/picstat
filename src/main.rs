#![warn(clippy::pedantic, clippy::nursery, clippy::unwrap_used, clippy::perf)]
#![deny(unused_must_use)]
#![deny(clippy::mod_module_files)]
#![allow(clippy::option_if_let_else)]

use std::path::PathBuf;

use clap::Parser;
use cli::CliArgs;

use crate::{context::Context, process::process_dir};

mod cli;
mod file_metadata;
mod process;
mod histogram;
mod context;
mod bucketers;

fn main() -> std::io::Result<()> {
    let args = CliArgs::parse();

    let curr_dir = &PathBuf::from(".");
    let path = args.path.as_ref().unwrap_or(curr_dir);
    let dir = std::fs::read_dir(path)?;

    let mut ctxt = Context::new();
    process_dir(dir, &args, &mut ctxt)?;

    println!("{}", ctxt.iso_hist);

    Ok(())
}

