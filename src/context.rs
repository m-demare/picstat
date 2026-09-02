use std::sync::atomic::{AtomicU32, Ordering};

use indicatif::ProgressBar;

use crate::{
    cli::CliArgs,
    progress_bar::build_progress_bar,
    string_interner::StringInterner,
    types::{Aperture, Camera, FocalLength, Lens, ShutterSpeed},
};

use dhist::{
    bucketers::{exact_match_bucketer::ExactMatchBucketer, log_bucketer::LogBucketer},
    histogram::Histogram,
};

pub struct Context {
    pub(crate) string_interner: StringInterner,

    pub(crate) iso_hist: Histogram<u32>,
    pub(crate) shutter_speed_hist: Histogram<ShutterSpeed>,
    pub(crate) aperture_hist: Histogram<Aperture>,
    pub(crate) focal_length_hist: Histogram<FocalLength>,
    pub(crate) lens_hist: Histogram<Lens>,
    pub(crate) camera_hist: Histogram<Camera>,

    analysed_files: AtomicU32,
    analysed_dirs: AtomicU32,

    progress_bar: ProgressBar,
    warnings: Vec<String>,
}

impl Context {
    pub fn new(args: &CliArgs) -> Self {
        Self {
            string_interner: StringInterner::default(),

            iso_hist: Histogram::new(args.hist_char),
            shutter_speed_hist: Histogram::new(args.hist_char),
            aperture_hist: Histogram::new(args.hist_char),
            focal_length_hist: Histogram::new(args.hist_char),
            lens_hist: Histogram::new(args.hist_char),
            camera_hist: Histogram::new(args.hist_char),

            analysed_files: 0.into(),
            analysed_dirs: 0.into(),

            progress_bar: ProgressBar::new(0),
            warnings: Vec::default(),
        }
    }

    pub fn analysed_files(&self) -> u32 {
        self.analysed_files.load(Ordering::Relaxed)
    }

    pub fn analysed_dirs(&self, recursive: bool) -> u32 {
        if recursive {
            self.analysed_dirs.load(Ordering::Relaxed)
        } else {
            1
        }
    }

    pub fn analyse_file(&self) {
        self.progress_bar.inc(1);
        self.analysed_files.fetch_add(1, Ordering::Relaxed);
    }

    pub fn analyse_dir(&self) {
        self.analysed_dirs.fetch_add(1, Ordering::Relaxed);
    }

    pub fn initialize_progress_bar(&mut self, args: &CliArgs, total_file_count: usize) {
        self.progress_bar = build_progress_bar(args, total_file_count);
    }

    pub fn finish_analysis(&self, args: &CliArgs) {
        self.progress_bar.finish();

        println!();
        self.warnings.iter().for_each(|w| println!("{w}"));
        println!();

        println!(
            "Analysed {} files in {} directories",
            self.analysed_files(),
            self.analysed_dirs(args.recursive)
        );
        println!();
    }

    pub fn print_stats(&self) {
        let lb = &LogBucketer::default();
        let eb = &ExactMatchBucketer::default();

        println!("Focal length");
        println!("{}", self.focal_length_hist.bucket(lb));
        println!("Aperture");
        println!("{}", self.aperture_hist.bucket(lb));
        println!("Shutter speed");
        println!("{}", self.shutter_speed_hist.bucket(lb));
        println!("ISO");
        println!("{}", self.iso_hist.bucket(lb));
        println!("Lens");
        println!("{}", self.lens_hist.bucket(eb));
        println!("Camera body");
        println!("{}", self.camera_hist.bucket(eb));
    }

    pub fn warn(&mut self, s: String) {
        self.warnings.push(s);
    }
}
