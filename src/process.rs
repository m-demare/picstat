use crate::Context;
use crate::cli::CliArgs;
use crate::file_metadata::FileMetadata;
use std::path::PathBuf;

pub fn process_dir(dir: std::fs::ReadDir, args: &CliArgs, ctxt: &mut Context) -> std::io::Result<()> {
    for entry in dir {
        let path = entry?.path();
        if path.is_file() {
            process_file(&path, args, ctxt)?;
        } else if path.is_dir() && args.recurse {
            process_dir(std::fs::read_dir(path)?, args, ctxt)?;
        }
    }
    Ok(())
}

pub fn process_file(path: &PathBuf, args: &CliArgs, ctxt: &mut Context) -> std::io::Result<()> {
    if !args.should_analyze(path) {
        return Ok(());
    }

    let file = std::fs::File::open(path)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = match exifreader.read_from_container(&mut bufreader) {
        Ok(exif) => exif,
        Err(e) => {
            if args.stop_on_error {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
            }
            if args.show_warnings {
                eprintln!("{e} - {}", path.to_string_lossy());
            }
            return Ok(());
        }
    };

    let metadata = FileMetadata::new(&exif);

    ctxt.iso_hist.insert_opt(metadata.iso());

    Ok(())
}


