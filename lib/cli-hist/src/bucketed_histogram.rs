use std::fmt::Display;

use crate::{
    bucketers::{Bucket, Bucketer},
    histogram::Histogram,
};

/// Struct representing a histogram that's already been bucketed, ready to display. See
/// [`crate::histogram::Histogram::bucket`]
pub struct BucketedHistogram<'a, 'b, V: Ord + Eq + Display> {
    hist: &'a Histogram<V>,
    bucketer: &'b dyn Bucketer<V>,
}

impl<'a, 'b, V: Ord + Eq + Display> BucketedHistogram<'a, 'b, V> {
    pub(super) fn new(hist: &'a Histogram<V>, bucketer: &'b dyn Bucketer<V>) -> Self {
        Self { hist, bucketer }
    }

    fn split_to_buckets(&self) -> Vec<(Bucket<V>, usize)> {
        self.bucketer.split(self.hist.items())
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

impl<'a, 'b, V: Ord + Eq + Display> Display for BucketedHistogram<'a, 'b, V> {
    #[expect(clippy::cast_precision_loss)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buckets = self.split_to_buckets();
        if let Some(max) = Self::largest_bucket_size(&buckets) {
            let (max_label_width, lines) = Self::format_hist_lines(&buckets, max as f64);

            for (label, hist_cols, count) in lines {
                let hist = std::iter::repeat_n(self.hist.char(), hist_cols).collect::<String>();
                writeln!(f, "{label:<max_label_width$} {hist}  ({count})")?;
            }
        } else {
            writeln!(f, "Empty histogram")?;
        }

        if self.hist.unknown() > 0 {
            writeln!(f, "Unkown value count: {}", self.hist.unknown())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bucketed_histogram::BucketedHistogram, bucketers::exact_match_bucketer::ExactMatchBucketer,
        histogram::Histogram,
    };

    #[test]
    fn test_largest_bucket_size() {
        let mut hist = Histogram::new('#');

        hist.insert(1);
        hist.insert(2);
        hist.insert(1);

        let buckets = hist.bucket(&ExactMatchBucketer::new()).split_to_buckets();
        let largest_bucket_size = BucketedHistogram::largest_bucket_size(&buckets);
        assert_eq!(largest_bucket_size, Some(2));
    }
}
