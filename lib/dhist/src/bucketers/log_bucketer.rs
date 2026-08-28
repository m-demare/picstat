use std::collections::BTreeMap;

use crate::bucketers::{Bucket, Bucketer};

pub struct LogBucketer {
    target_buckets: u8
}

impl LogBucketer {
    pub const fn new(target_buckets: u8) -> Self {
        Self { target_buckets }
    }

    fn compute_limits<T: AproxF64 + Ord + Eq + Copy>(
        hist: &BTreeMap<T, usize>,
        target_buckets: u8,
    ) -> (T, Vec<f64>) {
        let min = *hist.first_key_value().expect("not empty").0;
        let max = *hist.last_key_value().expect("not empty").0;

        if min == max {
            return (min, Vec::new());
        }

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

impl Default for LogBucketer {
    fn default() -> Self {
        Self::new(10)
    }
}

impl<T: AproxF64 + Ord + Eq + Copy> Bucketer<T> for LogBucketer {
    fn split(&self, hist: &BTreeMap<T, usize>) -> Vec<(Bucket<T>, usize)> {
        if hist.is_empty() {
            return vec![];
        }

        let (min, mut limits) = Self::compute_limits(hist, self.target_buckets);

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
}

pub trait AproxF64 {
    fn aprox(&self) -> f64;
}

impl AproxF64 for u32 {
    fn aprox(&self) -> f64 {
        f64::from(*self)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::bucketers::{Bucket, Bucketer, log_bucketer::LogBucketer};

    #[test]
    fn test_same_element_goes_to_same_bucket() {
        let mut hist = BTreeMap::new();
        hist.insert(10, 3);
        let bucketer = LogBucketer::new(100);

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
        let bucketer = LogBucketer::new(100);

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
        let bucketer = LogBucketer::new(100);

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
        let bucketer = LogBucketer::new(2);

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
        let bucketer = LogBucketer::new(2);

        let res = bucketer.split(&hist);

        assert_eq!(res.len(), 2);
        assert_eq!(res[0].0.min, 10);
        assert_eq!(res[0].0.max, 10);
        assert_eq!(res[0].1, 3);
        assert_eq!(res[1].0.min, 90);
        assert_eq!(res[1].0.max, 100);
        assert_eq!(res[1].1, 9);
    }
}
