#![warn(clippy::pedantic, clippy::nursery, clippy::unwrap_used, clippy::perf)]
#![deny(unused_must_use)]
#![deny(clippy::mod_module_files)]
#![allow(clippy::option_if_let_else)]

use std::path::PathBuf;

use clap::Parser;
use cli::CliArgs;

use crate::{context::Context, file_walking::process_dir};

mod cli;
mod context;
mod file_metadata;
mod file_process;
mod file_walking;
mod progress_bar;
mod string_interner;
mod types;

fn main() -> std::io::Result<()> {
    let args = CliArgs::parse();

    let curr_dir = &PathBuf::from(".");
    let path = args.path.as_ref().unwrap_or(curr_dir);

    let mut ctxt = Context::new(&args);
    process_dir(path, &args, &mut ctxt)?;

    ctxt.print_stats();

    Ok(())
}
