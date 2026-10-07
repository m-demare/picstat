#![allow(clippy::needless_pass_by_value)]

use chrono::{Local, NaiveDateTime};
use cli_hist::bucketers::AproxF64;

use crate::{
    file_metadata::FileMetadata,
    metadata_filter::{FilterValue, value::FilterError},
};

type NativeFn = fn(&FileMetadata, Vec<FilterValue>) -> Result<FilterValue, FilterError>;

numeric_prop!(iso);
numeric_prop!(aperture);
numeric_prop!(shutter_speed);
numeric_prop!(focal_length);
string_prop!(lens);
string_prop!(camera);

fn photo_datetime(
    metadata: &FileMetadata,
    args: Vec<FilterValue>,
) -> Result<FilterValue, FilterError> {
    let dt = metadata.datetime().map_or_else(
        || args.into_iter().next(),
        |dt| Some(FilterValue::Instant(dt.into_naive())),
    );
    dt.ok_or(FilterError::EmptyField("datetime"))
}

fn instant(_metadata: &FileMetadata, args: Vec<FilterValue>) -> Result<FilterValue, FilterError> {
    args.first()
        .map_or_else(
            || Ok(Local::now().naive_local()),
            |ts| {
                let ts = ts.as_string().unwrap_or("");
                NaiveDateTime::parse_from_str(ts, "%Y-%m-%d")
            },
        )
        .map(FilterValue::Instant)
        .map_err(FilterError::DateError)
}

pub fn lookup_fn(ident: &str) -> Option<NativeFn> {
    Some(match ident {
        "iso" => iso,
        "aperture" => aperture,
        "shutter_speed" => shutter_speed,
        "focal_length" => focal_length,
        "lens" => lens,
        "camera" => camera,
        "photo_datetime" => photo_datetime,
        "instant" => instant,
        _ => return None,
    })
}

macro_rules! numeric_prop {
    ($name: ident) => {
        fn $name(
            metadata: &FileMetadata,
            args: Vec<FilterValue>,
        ) -> Result<FilterValue, FilterError> {
            let val = metadata.$name().map_or_else(
                || args.into_iter().next(),
                |val| Some(FilterValue::Number(val.aprox())),
            );
            val.ok_or(FilterError::EmptyField(stringify!($name)))
        }
    };
}

macro_rules! string_prop {
    ($name: ident) => {
        fn $name(
            metadata: &FileMetadata,
            args: Vec<FilterValue>,
        ) -> Result<FilterValue, FilterError> {
            let val = metadata.$name().map_or_else(
                || args.into_iter().next(),
                |val| Some(FilterValue::String(val.clone().into())),
            );
            val.ok_or(FilterError::EmptyField(stringify!($name)))
        }
    };
}

use numeric_prop;
use string_prop;
