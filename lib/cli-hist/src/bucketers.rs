use std::collections::BTreeMap;

mod aprox_bucketers;
pub mod exact_match_bucketer;
pub mod linear_bucketer;
pub mod log_bucketer;

/// Struct that represents histogram's buckets as ranges of values.
#[derive(Debug, PartialEq, Eq)]
pub struct Bucket<V: Ord + Eq> {
    min: V,
    max: V,
}

impl<V: Ord + Eq> Bucket<V> {
    /// Create new bucket.
    pub const fn new(min: V, max: V) -> Self {
        Self { min, max }
    }

    /// Minimum value in range.
    pub const fn min(&self) -> &V {
        &self.min
    }

    /// Maximum value in range.
    pub const fn max(&self) -> &V {
        &self.max
    }
}

/// Bucketer trait
pub trait Bucketer<V: Ord + Eq> {
    /// Method that takes a histogram, and returns a Vec of Buckets, along with the number of values
    /// in each bucket.
    fn split(&self, hist: &BTreeMap<V, usize>) -> Vec<(Bucket<V>, usize)>;
}

/// Trait used by some bucketers, that work with ranges of values.
/// Takes a value, and gives it an approximate f64 representation, that the bucketer works with.
/// The approximation doesn't need to be exact, nor bijective.
pub trait AproxF64 {
    fn aprox(&self) -> f64;
}
