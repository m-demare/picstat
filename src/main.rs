#![warn(clippy::pedantic, clippy::nursery, clippy::unwrap_used, clippy::perf)]
#![deny(unused_must_use)]
#![deny(clippy::mod_module_files)]
#![allow(clippy::option_if_let_else)]

use std::path::PathBuf;

use clap::Parser;
use cli::CliArgs;

use crate::{context::Context, process::process_dir};

mod cli;
mod context;
mod file_metadata;
mod process;
mod progress_bar;
mod string_interner;
mod types;

fn main() -> std::io::Result<()> {
    let args = CliArgs::parse();

    let curr_dir = &PathBuf::from(".");
    let path = args.path.as_ref().unwrap_or(curr_dir);

    let mut ctxt = Context::new(&args);
    process_dir(path, &args, &mut ctxt)?;

    ctxt.progress_bar.finish();

    println!();
    ctxt.warnings.iter().for_each(|w| println!("{w}"));
    println!();

    println!(
        "Analysed {} files in {} directories",
        ctxt.analysed_files,
        if args.recursive {
            ctxt.analysed_dirs
        } else {
            1
        }
    );
    println!();

    println!("Focal length");
    println!("{}", ctxt.focal_length_hist);
    println!("Aperture");
    println!("{}", ctxt.aperture_hist);
    println!("Shutter speed");
    println!("{}", ctxt.shutter_speed_hist);
    println!("ISO");
    println!("{}", ctxt.iso_hist);
    println!("Lens");
    println!("{}", ctxt.lens_hist);
    println!("Camera body");
    println!("{}", ctxt.camera_hist);

    Ok(())
}
