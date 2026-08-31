use std::fmt::Debug;
use std::{collections::BTreeMap, fmt::Display};

use crate::bucketers::{Bucket, Bucketer};

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

    fn split_to_buckets(&self) -> Vec<(Bucket<V>, usize)> {
        self.bucketer.split(&self.items)
    }

    fn largest_bucket_size(buckets: &[(Bucket<V>, usize)]) -> Option<usize> {
        buckets.iter().map(|(_, count)| count).max().copied()
    }

    fn format_hist_lines(
        buckets: &[(Bucket<V>, usize)],
        largest_bucket_size: f64,
    ) -> (usize, Vec<(String, usize, usize)>) {
        let max_columns = 50_f64;
        let cols_per_count = largest_bucket_size / max_columns;

        let mut max_label_width = 14;
        let mut lines = Vec::new();

        for (bucket, count) in buckets {
            let label = if bucket.min() == bucket.max() {
                format!("{}", bucket.min())
            } else {
                format!("{} - {}", bucket.min(), bucket.max())
            };
            max_label_width = max_label_width.max(label.len());

            lines.push((label, ((*count as f64) / cols_per_count) as usize, *count));
        }
        (max_label_width, lines)
    }
}

impl<V: Ord + Eq + Debug + Display> Display for Histogram<V> {
    #[expect(clippy::cast_precision_loss)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buckets = self.split_to_buckets();
        if let Some(max) = Self::largest_bucket_size(&buckets) {
            let (max_label_width, lines) = Self::format_hist_lines(&buckets, max as f64);

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

#[cfg(test)]
mod tests {
    use crate::{bucketers::exact_match_bucketer::ExactMatchBucketer, histogram::Histogram};

    #[test]
    fn test_insert_different_elements() {
        let mut hist = Histogram::new(Box::new(ExactMatchBucketer::default()), '#');

        hist.insert(1);
        hist.insert(7);
        hist.insert(5);

        assert_eq!(hist.items.len(), 3);
        assert_eq!(hist.items.get(&1), Some(&1));
        assert_eq!(hist.items.get(&7), Some(&1));
        assert_eq!(hist.items.get(&5), Some(&1));
    }

    #[test]
    fn test_insert_same_element() {
        let mut hist = Histogram::new(Box::new(ExactMatchBucketer::default()), '#');

        hist.insert(1);
        hist.insert(1);
        hist.insert(1);

        assert_eq!(hist.items.len(), 1);
        assert_eq!(hist.items.get(&1), Some(&3));
    }

    #[test]
    fn test_insert_opt_some() {
        let mut hist = Histogram::new(Box::new(ExactMatchBucketer::default()), '#');

        hist.insert_opt(Some(3));
        hist.insert_opt(Some(7));
        hist.insert_opt(Some(3));

        assert_eq!(hist.items.len(), 2);
        assert_eq!(hist.items.get(&3), Some(&2));
        assert_eq!(hist.items.get(&7), Some(&1));
        assert_eq!(hist.unknown, 0)
    }

    #[test]
    fn test_insert_opt_none() {
        let mut hist = Histogram::<usize>::new(Box::new(ExactMatchBucketer::default()), '#');

        hist.insert_opt(None);
        hist.insert_opt(None);

        assert_eq!(hist.items.len(), 0);
        assert_eq!(hist.unknown, 2)
    }

    #[test]
    fn test_largest_bucket_size() {
        let mut hist = Histogram::new(Box::new(ExactMatchBucketer::default()), '#');

        hist.insert(1);
        hist.insert(2);
        hist.insert(1);

        let largest_bucket_size = Histogram::largest_bucket_size(&hist.split_to_buckets());
        assert_eq!(largest_bucket_size, Some(2));
    }
}
