use std::fmt::Debug;
use std::{collections::BTreeMap, fmt::Display};

use crate::bucketers::Bucketer;

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
                let s = if bucket.min() == bucket.max() {
                    format!("{}", bucket.min())
                } else {
                    format!("{} - {} ", bucket.min(), bucket.max())
                };
                write!(f, "{s:<15} ")?;

                for _ in 0..((count as f64) / cols_per_count) as isize {
                    write!(f, "#")?;
                }
                writeln!(f, "  ({count})")?;
            }
        } else {
            writeln!(f, "Empty histogram")?;
        }

        if self.unknown > 0 {
            writeln!(f, "Unkown value count: {}", self.unknown)?;
        }

        Ok(())
    }
}
