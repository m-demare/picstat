use std::collections::BTreeMap;

pub mod exact_match_bucketer;
pub mod log_bucketer;

#[derive(Debug)]
pub struct Bucket<V: Ord + Eq> {
    min: V,
    max: V,
}

impl<V: Ord + Eq> Bucket<V> {
    pub const fn min(&self) -> &V {
        &self.min
    }

    pub const fn max(&self) -> &V {
        &self.max
    }
}

pub trait Bucketer<V: Ord + Eq> {
    fn split(&self, hist: &BTreeMap<V, usize>, target_buckets: u8) -> Vec<(Bucket<V>, usize)>;
}
