use std::collections::BTreeMap;

use crate::bucketers::{Bucket, Bucketer};

pub struct ExactMatchBucketer {}

impl ExactMatchBucketer {
    pub const fn new() -> Self {
        Self {}
    }
}

impl<T: Ord + Eq + Clone> Bucketer<T> for ExactMatchBucketer {
    fn split(&self, hist: &BTreeMap<T, usize>, _target_buckets: u8) -> Vec<(Bucket<T>, usize)> {
        hist.iter()
            .map(|(key, count)| {
                (
                    Bucket {
                        min: key.clone(),
                        max: key.clone(),
                    },
                    *count,
                )
            })
            .collect()
    }
}
