use std::collections::BTreeMap;

use crate::{
    bucketers::{Bucket, Bucketer},
    types::Rational,
};

pub struct LogBucketer {}

impl LogBucketer {
    pub const fn new() -> Self {
        Self {}
    }

    fn compute_limits<T: AproxF64 + Ord + Eq + Copy>(
        hist: &BTreeMap<T, usize>,
        target_buckets: u8,
    ) -> (T, Vec<f64>) {
        let min = *hist.first_key_value().expect("not empty").0;
        let max = *hist.last_key_value().expect("not empty").0;

        let minlog = min.aprox().log2();
        let maxlog = max.aprox().log2();

        let step = (maxlog - minlog) / f64::from(target_buckets);

        let mut limits = (1..target_buckets)
            .map(|n| f64::from(n).mul_add(step, minlog))
            .map(f64::exp2)
            .rev()
            .collect::<Vec<_>>();
        limits.dedup();
        (min, limits)
    }
}

// #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
// pub fn clip_iso(n: f64) -> u32 {
//     ((n / 100.0).log2().round().exp2() * 100.0) as u32
// }

impl<T: AproxF64 + Ord + Eq + Copy> Bucketer<T> for LogBucketer {
    fn split(&self, hist: &BTreeMap<T, usize>, target_buckets: u8) -> Vec<(Bucket<T>, usize)> {
        if hist.is_empty() {
            return vec![];
        }

        let (min, mut limits) = Self::compute_limits(hist, target_buckets);

        let mut bucket_start = min;
        let mut bucket_end = min;
        let mut current_limit = limits.pop();
        let mut current_count = 0;

        let mut buckets = Vec::new();

        for (key, count) in hist {
            if let Some(cl) = current_limit
                && key.aprox() > cl
            {
                buckets.push((
                    Bucket {
                        min: bucket_start,
                        max: bucket_end,
                    },
                    current_count,
                ));
                bucket_start = *key;
                current_count = 0;
                current_limit = limits.pop();
            }
            bucket_end = *key;
            current_count += count;
        }

        buckets.push((
            Bucket {
                min: bucket_start,
                max: bucket_end,
            },
            current_count,
        ));

        buckets
    }
}

pub trait AproxF64 {
    fn aprox(&self) -> f64;
}

impl AproxF64 for u32 {
    fn aprox(&self) -> f64 {
        f64::from(*self)
    }
}

impl AproxF64 for Rational {
    fn aprox(&self) -> f64 {
        self.to_f64()
    }
}
