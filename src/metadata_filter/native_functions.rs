use cli_hist::bucketers::AproxF64;

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

pub fn lookup_fn(ident: &str) -> Option<NativeFn> {
    match ident {
        "aperture" => Some(aperture),
        _ => None,
    }
}
