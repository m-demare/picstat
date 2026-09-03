//! # cli-hist
//!
//! A simple library designed to compute and draw histograms.
//!
//! ## Example
//!
//! ```
//! use cli_hist::histogram::Histogram;
//! use cli_hist::bucketers::linear_bucketer::LinearBucketer;
//! let mut hist = Histogram::new('█');
//!
//! hist.insert(1);
//! hist.insert(2);
//! hist.insert(4);
//!
//! println!("{}", hist.bucket(&LinearBucketer::new(2)));
//!
//! // Example output:
//!
//! // 1 - 2          ██████████████████████████████████████████████████  (2)
//! // 4              █████████████████████████                           (1)
//! ```

pub mod bucketed_histogram;
pub mod bucketers;
pub mod histogram;
