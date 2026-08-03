#![warn(clippy::pedantic, clippy::nursery, clippy::unwrap_used, clippy::perf)]
#![deny(unused_must_use)]
#![deny(clippy::mod_module_files)]
#![allow(clippy::option_if_let_else)]

use std::path::PathBuf;

use clap::Parser;
use cli::CliArgs;

use crate::{context::Context, process::process_dir};

mod bucketers;
mod cli;
mod context;
mod file_metadata;
mod histogram;
mod process;
mod types;

fn main() -> std::io::Result<()> {
    let args = CliArgs::parse();

    let curr_dir = &PathBuf::from(".");
    let path = args.path.as_ref().unwrap_or(curr_dir);
    let dir = std::fs::read_dir(path)?;

    let mut ctxt = Context::new();
    process_dir(dir, &args, &mut ctxt)?;

    println!("ISO");
    println!("{}", ctxt.iso_hist);
    println!();
    println!("Lens");
    println!("{}", ctxt.lens_hist);
    println!();
    println!("Shutter speed");
    println!("{}", ctxt.shutter_speed_hist);
    println!();
    println!("Aperture");
    println!("{}", ctxt.aperture_hist);
    println!();
    println!("Focal length");
    println!("{}", ctxt.focal_length_hist);

    Ok(())
}
