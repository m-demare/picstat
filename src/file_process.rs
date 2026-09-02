use crate::Context;
use crate::cli::CliArgs;
use crate::file_metadata::FileMetadata;
use std::path::Path;

pub enum ParsedMetadata {
    Metadata(FileMetadata),
    Warning(String),
    Silent,
}

pub fn process_metadata(ctxt: &mut Context, metadata: ParsedMetadata) {
    let metadata = match metadata {
        ParsedMetadata::Metadata(metadata) => metadata,
        ParsedMetadata::Warning(w) => {
            ctxt.warn(w);
            return;
        }
        ParsedMetadata::Silent => return,
    };

    ctxt.iso_hist.insert_opt(metadata.iso());
    ctxt.lens_hist.insert_opt(metadata.lens().cloned());
    ctxt.shutter_speed_hist.insert_opt(metadata.shutter_speed());
    ctxt.aperture_hist.insert_opt(metadata.aperture());
    ctxt.focal_length_hist.insert_opt(metadata.focal_length());
    ctxt.camera_hist.insert_opt(metadata.camera().cloned());
}

pub fn parse_metadata(
    path: &Path,
    args: &CliArgs,
    ctxt: &Context,
) -> Result<ParsedMetadata, std::io::Error> {
    let metadata = match FileMetadata::from_exif(ctxt, path) {
        Ok(value) => value,
        Err(e) => {
            if args.stop_on_error {
                return Err(e);
            }
            if !args.suppress_warnings {
                return Ok(ParsedMetadata::Warning(format!(
                    "{e} - {}",
                    path.to_string_lossy()
                )));
            }
            return Ok(ParsedMetadata::Silent);
        }
    };
    Ok(ParsedMetadata::Metadata(metadata))
}
