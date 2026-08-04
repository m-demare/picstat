use std::path::{Path, PathBuf};

use clap::Parser;

/// Rua language interpreter
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[arg()]
    pub(super) path: Option<PathBuf>,

    #[arg(short, long, value_parser = |s: &str| -> Result<String, std::convert::Infallible> { Ok(s.to_lowercase()) })]
    pub(super) extensions: Vec<String>,

    #[arg(short, long, default_value_t = false)]
    pub(super) recurse: bool,

    #[arg(short, long, default_value_t = false)]
    pub(super) stop_on_error: bool,

    #[arg(short = 'w', long, default_value_t = false)]
    pub(super) supress_warnings: bool,

    #[arg(long, default_value_t = '█')]
    pub(super) hist_char: char,
}

impl CliArgs {
    pub(super) fn should_analyze(&self, e: &Path) -> bool {
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
