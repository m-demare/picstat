use std::path::{Path, PathBuf};

use clap::Parser;

use crate::metadata_filter::MetadataFilter;

/// Get stats on your camera settings
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Directory containing the images. Defaults to current directory
    #[arg()]
    pub(super) path: Option<PathBuf>,

    /// File extensions to analyse ( e.g. -e jpg -e cr2 )
    #[arg(short, long)]
    pub(super) extensions: Vec<String>,

    /// Analyse subdirectories recursively
    #[arg(short, long, default_value_t = false)]
    pub(super) recursive: bool,

    /// Exit upon first file analysis error
    #[arg(short, long, default_value_t = false)]
    pub(super) stop_on_error: bool,

    /// Suppress warnings for parsing failures
    #[arg(short = 'w', long, default_value_t = false)]
    pub(super) suppress_warnings: bool,

    /// Character to be used for the histograms
    #[arg(long, default_value_t = '█')]
    pub(super) hist_char: char,

    /// Filter to apply to the files. Include the photo metadata in the histograms if it returns true.
    /// Example:
    /// --filter='photo_datetime() < instant("2026-08-01") & iso() > 640'
    /// support basic comparison and arithmetics (+ - * / < <= == != etc)
    /// decimal numbers, booleans, and strings
    /// Full list of callable functions at src/metadata_filter/native_functions.rs
    #[arg(short, long, default_value = "true", verbatim_doc_comment)]
    pub(super) filter: MetadataFilter,
}

impl CliArgs {
    pub(super) fn should_analyse(&self, path: &Path) -> bool {
        if self.extensions.is_empty() {
            return true;
        }
        match path.extension() {
            Some(ext) => self.extensions.iter().any(|e| ext.eq_ignore_ascii_case(e)),
            None => false,
        }
    }
}
