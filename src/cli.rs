use std::path::{Path, PathBuf};

use clap::Parser;

/// Get stats on your camera settings
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Directory containing the images. Defaults to current directory
    #[arg()]
    pub(super) path: Option<PathBuf>,

    /// File extensions to analyse ( e.g. -e jpg -e cr2 )
    #[arg(short, long, value_parser = |s: &str| -> Result<String, std::convert::Infallible> { Ok(s.to_lowercase()) })]
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
}

impl CliArgs {
    pub(super) fn should_analyse(&self, e: &Path) -> bool {
        if self.extensions.is_empty() {
            return true;
        }
        match e.extension() {
            Some(ext) => self
                .extensions
                .iter()
                .any(|e| **e == *ext.to_ascii_lowercase()),
            None => false,
        }
    }
}
