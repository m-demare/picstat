use cli_hist::bucketers::AproxF64;

use crate::{
    file_metadata::FileMetadata,
    metadata_filter::{FilterValue, value::FilterError},
};

fn aperture(metadata: &FileMetadata, args: Vec<FilterValue>) -> Result<FilterValue, FilterError> {
    let aperture = metadata.aperture().map_or_else(
        || args.into_iter().next(),
        |ap| Some(FilterValue::Number(ap.aprox())),
    );
    aperture.ok_or(FilterError::EmptyField("aperture"))
}

pub fn lookup_fn(
    ident: &str,
) -> Option<fn(&FileMetadata, Vec<FilterValue>) -> Result<FilterValue, FilterError>> {
    match ident {
        "aperture" => Some(aperture),
        _ => None,
    }
}
