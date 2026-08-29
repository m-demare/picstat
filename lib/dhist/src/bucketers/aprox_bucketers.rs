use std::collections::BTreeMap;

use crate::bucketers::{AproxF64, Bucket};

impl<T: Copy> AproxF64 for T
where
    f64: From<T>,
{
    fn aprox(&self) -> f64 {
        f64::from(*self)
    }
}

pub(super) fn compute_buckets_limits<T: Ord + AproxF64>(
    hist: &BTreeMap<T, usize>,
    target_buckets: u8,
    f: impl Fn(f64) -> f64,
    f_inv: impl Fn(f64) -> f64,
) -> (&T, Vec<f64>) {
    let min = hist.first_key_value().expect("not empty").0;
    let max = hist.last_key_value().expect("not empty").0;

    if min == max {
        return (min, Vec::new());
    }

    let minlog = f(min.aprox());
    let maxlog = f(max.aprox());

    let step = (maxlog - minlog) / f64::from(target_buckets);

    let mut limits = (1..target_buckets)
        .map(|n| f64::from(n).mul_add(step, minlog))
        .map(f_inv)
        .rev()
        .collect::<Vec<_>>();
    limits.dedup();
    (min, limits)
}

pub(super) fn buckets_by_limits<T: AproxF64 + Ord + Eq + Copy>(
    hist: &BTreeMap<T, usize>,
    min: T,
    mut limits: Vec<f64>,
) -> Vec<(Bucket<T>, usize)> {
    let mut bucket_start = min;
    let mut bucket_end = min;
    let mut current_limit = limits.pop();
    let mut current_count = 0;

    let mut buckets = Vec::new();

    for (key, count) in hist {
        if let Some(cl) = current_limit
            && key.aprox() > cl
        {
            buckets.push((Bucket::new(bucket_start, bucket_end), current_count));
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
