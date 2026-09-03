use std::{collections::BTreeMap, fmt::Display};

use crate::{bucketed_histogram::BucketedHistogram, bucketers::Bucketer};

/// Struct representing the histogram.
pub struct Histogram<V: Ord + Eq + Display> {
    items: BTreeMap<V, usize>,
    unknown: usize,
    hist_char: char,
}

impl<V: Ord + Eq + Display> Histogram<V> {
    /// Create a new histogram, using `hist_char` as the character to render the histograms.
    pub fn new(hist_char: char) -> Self {
        Self {
            items: BTreeMap::default(),
            unknown: 0,
            hist_char,
        }
    }

    /// Insert new data to the histogram.
    pub fn insert(&mut self, v: V) {
        *self.items.entry(v).or_default() += 1;
    }

    /// Insert new data to the histogram.
    /// If the value is None, it'll be inserted as an "unknown" value, that gets drawn separately
    /// from the histogram.
    pub fn insert_opt(&mut self, v: Option<V>) {
        match v {
            Some(v) => self.insert(v),
            None => self.unknown += 1,
        }
    }

    /// Bucket the histogram.
    /// This method returns a BucketedHistogram, that can be displayed.
    /// Different Bucketer's can be used, depending on the datatypes and the specific needs.
    /// E.g.: ExactMatchBucketer, LogBucketer, LinearBucketer
    /// # Example
    /// ```
    /// use cli_hist::histogram::Histogram;
    /// use cli_hist::bucketers::linear_bucketer::LinearBucketer;
    /// let mut hist = Histogram::new('█');
    ///
    /// hist.insert(1);
    /// println!("{}", hist.bucket(&LinearBucketer::new(10)));
    /// // hist gets split into buckets according to LinearBucketer, and displayed
    /// ```
    pub fn bucket<'a, 'b>(&'a self, bucketer: &'b dyn Bucketer<V>) -> BucketedHistogram<'a, 'b, V> {
        BucketedHistogram::new(self, bucketer)
    }

    pub(crate) fn char(&self) -> char {
        self.hist_char
    }

    pub(crate) fn items(&self) -> &BTreeMap<V, usize> {
        &self.items
    }

    pub(crate) fn unknown(&self) -> usize {
        self.unknown
    }
}

#[cfg(test)]
mod tests {
    use crate::histogram::Histogram;

    #[test]
    fn test_insert_different_elements() {
        let mut hist = Histogram::new('#');

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
        let mut hist = Histogram::new('#');

        hist.insert(1);
        hist.insert(1);
        hist.insert(1);

        assert_eq!(hist.items.len(), 1);
        assert_eq!(hist.items.get(&1), Some(&3));
    }

    #[test]
    fn test_insert_opt_some() {
        let mut hist = Histogram::new('#');

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
        let mut hist = Histogram::<usize>::new('#');

        hist.insert_opt(None);
        hist.insert_opt(None);

        assert_eq!(hist.items.len(), 0);
        assert_eq!(hist.unknown, 2)
    }
}
