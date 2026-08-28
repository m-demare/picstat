use std::fmt::Debug;
use std::{collections::BTreeMap, fmt::Display};

use crate::bucketers::Bucketer;

pub struct Histogram<V: Ord + Eq + Debug + Display> {
    items: BTreeMap<V, usize>,
    unknown: usize,
    bucketer: Box<dyn Bucketer<V>>,
    hist_char: char,
}

impl<V: Ord + Eq + Debug + Display> Histogram<V> {
    pub fn new(bucketer: Box<dyn Bucketer<V>>, hist_char: char) -> Self {
        Self {
            items: BTreeMap::default(),
            unknown: 0,
            bucketer,
            hist_char,
        }
    }

    pub fn insert(&mut self, v: V) {
        *self.items.entry(v).or_default() += 1;
    }

    pub fn insert_opt(&mut self, v: Option<V>) {
        match v {
            Some(v) => self.insert(v),
            None => self.unknown += 1,
        }
    }
}

impl<V: Ord + Eq + Debug + Display> Display for Histogram<V> {
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss,
        clippy::cast_sign_loss
    )]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buckets = self.bucketer.split(&self.items, 10);
        if let Some(&max) = buckets.iter().map(|(_, count)| count).max() {
            let max_columns = 50_f64;
            let cols_per_count = max as f64 / max_columns;
            let mut max_label_width = 14;
            let mut lines = Vec::new();

            for (bucket, count) in buckets {
                let label = if bucket.min() == bucket.max() {
                    format!("{}", bucket.min())
                } else {
                    format!("{} - {}", bucket.min(), bucket.max())
                };
                max_label_width = max_label_width.max(label.len());

                lines.push((label, ((count as f64) / cols_per_count) as usize, count));
            }

            for (label, hist_cols, count) in lines {
                let hist = std::iter::repeat_n(self.hist_char, hist_cols).collect::<String>();
                writeln!(f, "{label:<max_label_width$} {hist}  ({count})")?;
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
