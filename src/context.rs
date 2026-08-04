use crate::{
    bucketers::{exact_match_bucketer::ExactMatchBucketer, log_bucketer::LogBucketer},
    histogram::Histogram,
    string_interner::StringInterner,
    types::{Aperture, FocalLength, Lens, ShutterSpeed},
};

pub struct Context {
    pub(crate) lens_interner: StringInterner,
    pub(crate) iso_hist: Histogram<u32>,
    pub(crate) lens_hist: Histogram<Lens>,
    pub(crate) shutter_speed_hist: Histogram<ShutterSpeed>,
    pub(crate) aperture_hist: Histogram<Aperture>,
    pub(crate) focal_length_hist: Histogram<FocalLength>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            lens_interner: StringInterner::default(),
            iso_hist: Histogram::new(Box::new(LogBucketer::new())),
            lens_hist: Histogram::new(Box::new(ExactMatchBucketer::new())),
            shutter_speed_hist: Histogram::new(Box::new(LogBucketer::new())),
            aperture_hist: Histogram::new(Box::new(LogBucketer::new())),
            focal_length_hist: Histogram::new(Box::new(LogBucketer::new())),
        }
    }
}
