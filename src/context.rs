use crate::histogram::Histogram;

pub struct Context {
    lens_interner: u32, // TODO
    iso_hist: Histogram<usize>,
}

impl Context {
    pub fn new() -> Self {
        Self { lens_interner: 0, iso_hist: Histogram::new() }
    }
}

