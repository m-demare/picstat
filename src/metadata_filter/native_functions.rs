#![allow(clippy::needless_pass_by_value)]

use std::str::FromStr;

use cli_hist::bucketers::AproxF64;
use jiff::Timestamp;

use crate::{
    file_metadata::FileMetadata,
    metadata_filter::{FilterValue, value::FilterError},
};

type NativeFn = fn(&FileMetadata, Vec<FilterValue>) -> Result<FilterValue, FilterError>;

fn aperture(metadata: &FileMetadata, args: Vec<FilterValue>) -> Result<FilterValue, FilterError> {
    let aperture = metadata.aperture().map_or_else(
        || args.into_iter().next(),
        |ap| Some(FilterValue::Number(ap.aprox())),
    );
    aperture.ok_or(FilterError::EmptyField("aperture"))
}

fn instant(_metadata: &FileMetadata, args: Vec<FilterValue>) -> Result<FilterValue, FilterError> {
    args.first()
        .map_or_else(
            || Ok(Timestamp::now()),
            |ts| {
                let ts = ts.as_string().unwrap_or("");
                Timestamp::from_str(ts)
            },
        )
        .map(FilterValue::Instant)
        .map_err(FilterError::DateError)
}

pub fn lookup_fn(ident: &str) -> Option<NativeFn> {
    Some(match ident {
        "aperture" => aperture,
        "instant" => instant,
        _ => return None,
    })
}
