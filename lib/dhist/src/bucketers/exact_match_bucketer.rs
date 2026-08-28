use std::collections::BTreeMap;

use crate::bucketers::{Bucket, Bucketer};

pub struct ExactMatchBucketer {}

impl ExactMatchBucketer {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Default for ExactMatchBucketer {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + Eq + Clone> Bucketer<T> for ExactMatchBucketer {
    fn split(&self, hist: &BTreeMap<T, usize>, _target_buckets: u8) -> Vec<(Bucket<T>, usize)> {
        hist.iter()
            .map(|(key, count)| (Bucket::new(key.clone(), key.clone()), *count))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::bucketers::{Bucket, Bucketer, exact_match_bucketer::ExactMatchBucketer};

    #[test]
    fn test_same_element_goes_to_same_bucket() {
        let mut hist = BTreeMap::new();
        hist.insert("hi", 3);
        let bucketer = ExactMatchBucketer::new();

        let res = bucketer.split(&hist, 100);

        assert_eq!(res.len(), 1);
        assert_eq!(res[0].1, 3);
        assert_eq!(res[0].0.min, "hi");
        assert_eq!(res[0].0.max, "hi");
    }

    #[test]
    fn test_different_elements_go_to_different_buckets() {
        let mut hist = BTreeMap::new();
        hist.insert("hi", 3);
        hist.insert("bye", 5);
        hist.insert("howdy", 2);
        let bucketer = ExactMatchBucketer::new();

        let res = bucketer.split(&hist, 100);

        assert_eq!(res.len(), 3);
        assert!(res.contains(&(
            Bucket {
                min: "hi",
                max: "hi"
            },
            3
        )));
        assert!(res.contains(&(
            Bucket {
                min: "bye",
                max: "bye"
            },
            5
        )));
        assert!(res.contains(&(
            Bucket {
                min: "howdy",
                max: "howdy"
            },
            2
        )));
    }

    #[test]
    fn test_buckets_are_ordered() {
        let mut hist = BTreeMap::new();
        hist.insert("hi", 3);
        hist.insert("bye", 5);
        hist.insert("howdy", 2);
        let bucketer = ExactMatchBucketer::new();

        let res = bucketer.split(&hist, 100);

        assert_eq!(res.len(), 3);
        assert_eq!(res[0].0.min, "bye");
        assert_eq!(res[1].0.min, "hi");
        assert_eq!(res[2].0.min, "howdy");
    }
}
