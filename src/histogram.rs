use std::fmt::Debug;
use std::{collections::BTreeMap, fmt::Display};

pub struct Histogram<V: Ord + Eq + Debug + Display> {
    items: BTreeMap<V, usize>,
    unknown: usize,
    bucketer: Box<dyn Bucketer<V>>,
}

impl<V: Ord + Eq + Debug + Display> Histogram<V> {
    pub(crate) fn new(bucketer: Box<dyn Bucketer<V>>) -> Self {
        Self {
            items: BTreeMap::default(),
            unknown: 0,
            bucketer,
        }
    }

    pub(crate) fn insert(&mut self, v: V) {
        *self.items.entry(v).or_default() += 1;
    }

    pub(crate) fn insert_opt(&mut self, v: Option<V>) {
        match v {
            Some(v) => self.insert(v),
            None => self.unknown += 1,
        }
    }
}

impl<V: Ord + Eq + Debug + Display> Display for Histogram<V> {
    #[expect(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buckets = self.bucketer.split(&self.items, 10);
        if let Some(&max) = buckets.iter().map(|(_, count)| count).max() {
            let max_columns = 50_f64;
            let cols_per_count = max as f64 / max_columns;

            for (bucket, count) in buckets {
                let s = format!("{} - {} ", bucket.min, bucket.max);
                write!(f, "{s:<15}")?;

                for _ in 0..((count as f64) / cols_per_count) as isize {
                    write!(f, "#")?;
                }
                writeln!(f, "  ({count})")?;
            }
        } else {
            writeln!(f, "Empty histogram")?;
        }

        writeln!(f)?;
        writeln!(f, "Unkown value count: {}", self.unknown)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct Bucket<V: Ord + Eq> {
    min: V,
    max: V,
}

pub trait Bucketer<V: Ord + Eq> {
    fn split(&self, hist: &BTreeMap<V, usize>, target_buckets: u8) -> Vec<(Bucket<V>, usize)>;
}

#[derive(Default)]
pub struct LogBucketer {}

impl Bucketer<u32> for LogBucketer {
    fn split(&self, hist: &BTreeMap<u32, usize>, target_buckets: u8) -> Vec<(Bucket<u32>, usize)> {
        if hist.is_empty() {
            return vec![];
        }

        let min = *hist.first_key_value().expect("not empty").0;
        let max = *hist.last_key_value().expect("not empty").0;

        let minlog = f64::from(min).log2();
        let maxlog = f64::from(max).log2();

        let step = (maxlog - minlog) / f64::from(target_buckets);

        let mut limits = (1..target_buckets)
            .map(|n| f64::from(n).mul_add(step, minlog))
            .map(f64::exp2)
            .map(Self::clip_iso)
            .rev()
            .collect::<Vec<_>>();
        limits.dedup();

        let mut bucket_start = min;
        let mut bucket_end = min;
        let mut current_limit = limits.pop();
        let mut current_count = 0;

        let mut buckets = Vec::new();

        for (key, count) in hist {
            if let Some(cl) = current_limit && *key > cl {
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

        buckets
    }
}

impl LogBucketer {
    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn clip_iso(n: f64) -> u32 {
        ((n / 100.0).log2().round().exp2() * 100.0) as u32
    }
}
