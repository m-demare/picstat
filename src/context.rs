use crate::histogram::{Histogram, LogBucketer};

pub struct Context {
    pub(crate) lens_interner: u32, // TODO
    pub(crate) iso_hist: Histogram<u32>,
}

impl Context {
    pub fn new() -> Self {
        Self { lens_interner: 0, iso_hist: Histogram::new(Box::new(LogBucketer::default())) }
    }
}

