use crate::{bucketers::log_bucketer::{LogBucketer, clip_iso}, histogram::Histogram};

pub struct Context {
    pub(crate) lens_interner: u32, // TODO
    pub(crate) iso_hist: Histogram<u32>,
}

impl Context {
    pub fn new() -> Self {
        Self { lens_interner: 0, iso_hist: Histogram::new(Box::new(LogBucketer::new(&clip_iso))) }
    }
}

