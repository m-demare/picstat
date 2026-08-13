use crate::Context;
use crate::cli::CliArgs;
use crate::file_metadata::FileMetadata;
use std::path::Path;

pub fn process_dir(
    dir: std::fs::ReadDir,
    args: &CliArgs,
    ctxt: &mut Context,
) -> std::io::Result<()> {
    ctxt.analysed_dirs += 1;
    for entry in dir {
        let path = entry?.path();
        if path.is_file() {
            process_file(&path, args, ctxt)?;
        } else if path.is_dir() && args.recursive {
            process_dir(std::fs::read_dir(path)?, args, ctxt)?;
        }
    }
    Ok(())
}

pub fn process_file(path: &Path, args: &CliArgs, ctxt: &mut Context) -> std::io::Result<()> {
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
