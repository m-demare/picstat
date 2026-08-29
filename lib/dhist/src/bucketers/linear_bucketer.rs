use std::{collections::BTreeMap, convert::identity};

use crate::bucketers::{
    AproxF64, Bucket, Bucketer,
    aprox_bucketers::{buckets_by_limits, compute_buckets_limits},
};

pub struct LinearBucketer {
    target_buckets: u8,
}

impl LinearBucketer {
    pub const fn new(target_buckets: u8) -> Self {
        Self { target_buckets }
    }
}

impl Default for LinearBucketer {
    fn default() -> Self {
        Self::new(10)
    }
}

impl<T: AproxF64 + Ord + Eq + Copy> Bucketer<T> for LinearBucketer {
    fn split(&self, hist: &BTreeMap<T, usize>) -> Vec<(Bucket<T>, usize)> {
        if hist.is_empty() {
            return vec![];
        }

        let (min, limits) = compute_buckets_limits(hist, self.target_buckets, identity, identity);

        buckets_by_limits(hist, *min, limits)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::bucketers::{Bucket, Bucketer, linear_bucketer::LinearBucketer};

    #[test]
    fn test_same_element_goes_to_same_bucket() {
        let mut hist = BTreeMap::new();
        hist.insert(10, 3);
        let bucketer = LinearBucketer::new(100);

        let res = bucketer.split(&hist);

        dbg!(&res);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].1, 3);
        assert_eq!(res[0].0.min, 10);
        assert_eq!(res[0].0.max, 10);
    }

    #[test]
    fn test_different_elements_go_to_different_buckets() {
        let mut hist = BTreeMap::new();
        hist.insert(10, 3);
        hist.insert(20, 5);
        let bucketer = LinearBucketer::new(100);

        let res = bucketer.split(&hist);

        assert_eq!(res.len(), 2);
        assert!(res.contains(&(Bucket { min: 10, max: 10 }, 3)));
        assert!(res.contains(&(Bucket { min: 20, max: 20 }, 5)));
    }

    #[test]
    fn test_buckets_are_ordered() {
        let mut hist = BTreeMap::new();
        hist.insert(10, 3);
        hist.insert(1000, 2);
        hist.insert(100, 5);
        let bucketer = LinearBucketer::new(100);

        let res = bucketer.split(&hist);

        assert_eq!(res.len(), 3);
        assert_eq!(res[0].0.min, 10);
        assert_eq!(res[1].0.min, 100);
        assert_eq!(res[2].0.min, 1000);
    }

    #[test]
    fn test_close_items_are_bucketed_toghether() {
        let mut hist = BTreeMap::new();
        hist.insert(10, 3);
        hist.insert(20, 2);
        hist.insert(100, 7);
        let bucketer = LinearBucketer::new(2);

        let res = bucketer.split(&hist);

        assert_eq!(res.len(), 2);
        assert_eq!(res[0].0.min, 10);
        assert_eq!(res[0].0.max, 20);
        assert_eq!(res[0].1, 5);
        assert_eq!(res[1].0.min, 100);
        assert_eq!(res[1].0.max, 100);
        assert_eq!(res[1].1, 7);
    }

    #[test]
    fn test_close_items_are_bucketed_toghether_2() {
        let mut hist = BTreeMap::new();
        hist.insert(10, 3);
        hist.insert(90, 2);
        hist.insert(100, 7);
        let bucketer = LinearBucketer::new(2);

        let res = bucketer.split(&hist);

        assert_eq!(res.len(), 2);
        assert_eq!(res[0].0.min, 10);
        assert_eq!(res[0].0.max, 10);
        assert_eq!(res[0].1, 3);
        assert_eq!(res[1].0.min, 90);
        assert_eq!(res[1].0.max, 100);
        assert_eq!(res[1].1, 9);
    }

    #[test]
    fn test_items_are_bucketed_linearly() {
        let mut hist = BTreeMap::new();
        (1..=10).for_each(|i| {
            hist.insert(i, 1);
        });
        let bucketer = LinearBucketer::new(5);

        let res = bucketer.split(&hist);

        assert_eq!(res.len(), 5);
        assert_eq!(res[0].0.min, 1);
        assert_eq!(res[0].0.max, 2);
        assert_eq!(res[0].1, 2);

        assert_eq!(res[1].0.min, 3);
        assert_eq!(res[1].0.max, 4);
        assert_eq!(res[1].1, 2);

        assert_eq!(res[2].0.min, 5);
        assert_eq!(res[2].0.max, 6);
        assert_eq!(res[2].1, 2);

        assert_eq!(res[3].0.min, 7);
        assert_eq!(res[3].0.max, 8);
        assert_eq!(res[3].1, 2);

        assert_eq!(res[4].0.min, 9);
        assert_eq!(res[4].0.max, 10);
        assert_eq!(res[4].1, 2);
    }
}
