use walkdir::WalkDir;

use crate::Context;
use crate::cli::CliArgs;
use crate::file_metadata::FileMetadata;
use std::path::Path;

pub fn process_dir(dir: &Path, args: &CliArgs, ctxt: &mut Context) -> std::io::Result<()> {
    let walker = build_walker(dir, args);

    walk(walker, args, ctxt)
}

fn walk(walker: WalkDir, args: &CliArgs, ctxt: &mut Context) -> std::io::Result<()> {
    for f in walker {
        let f = f?;
        if f.file_type().is_file() {
            process_file(f.path(), args, ctxt)?;
        } else if f.file_type().is_dir() {
            ctxt.analysed_dirs += 1;
        }
    }
    Ok(())
}

fn process_file(path: &Path, args: &CliArgs, ctxt: &mut Context) -> std::io::Result<()> {
    if !args.should_analyse(path) {
        return Ok(());
    }

    let metadata = match FileMetadata::from_exif(ctxt, path) {
        Ok(value) => value,
        Err(e) => {
            if args.stop_on_error {
                return Err(e);
            }
            if !args.suppress_warnings {
                eprintln!("{e} - {}", path.to_string_lossy());
            }
            return Ok(());
        }
    };

    ctxt.analysed_files += 1;
    ctxt.iso_hist.insert_opt(metadata.iso());
    ctxt.lens_hist.insert_opt(metadata.lens().cloned());
    ctxt.shutter_speed_hist.insert_opt(metadata.shutter_speed());
    ctxt.aperture_hist.insert_opt(metadata.aperture());
    ctxt.focal_length_hist.insert_opt(metadata.focal_length());
    ctxt.camera_hist.insert_opt(metadata.camera().cloned());

    Ok(())
}

fn build_walker(dir: &Path, args: &CliArgs) -> WalkDir {
    let mut walker = WalkDir::new(dir);

    if !args.recursive {
        walker = walker.max_depth(1);
    }
    walker
}
