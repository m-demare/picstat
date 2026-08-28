use std::collections::BTreeMap;

pub mod exact_match_bucketer;
pub mod log_bucketer;

#[derive(Debug, PartialEq, Eq)]
pub struct Bucket<V: Ord + Eq> {
    min: V,
    max: V,
}

impl<V: Ord + Eq> Bucket<V> {
    pub const fn new(min: V, max: V) -> Self {
        Self { min, max }
    }

    pub const fn min(&self) -> &V {
        &self.min
    }

    pub const fn max(&self) -> &V {
        &self.max
    }
}

pub trait Bucketer<V: Ord + Eq> {
    fn split(&self, hist: &BTreeMap<V, usize>) -> Vec<(Bucket<V>, usize)>;
}
