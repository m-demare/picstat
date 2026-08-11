use exif::{In, Tag, Value};

use crate::{
    context::Context,
    types::{Aperture, Camera, FocalLength, Lens, Rational, ShutterSpeed},
};

#[derive(Debug)]
pub struct FileMetadata {
    iso: Option<u32>,
    aperture: Option<Aperture>,
    shutter_speed: Option<ShutterSpeed>,
    focal_length: Option<FocalLength>,
    lens: Option<Lens>,
    camera: Option<Camera>,
}

impl FileMetadata {
    pub fn new(exif: &exif::Exif, ctxt: &mut Context) -> Self {
        let mut iso = None;
        let mut shutter_speed = None;
        let mut aperture = None;
        let mut focal_length = None;
        let mut lens = None;
        let mut camera = None;

        if let Some(field) = exif.get_field(Tag::ISOSpeed, In::PRIMARY) {
            iso = field
                .value
                .get_uint(0)
                .or_else(|| todo!("ISOSpeed {:?}", field.value));
        }
        if let Some(field) = exif.get_field(Tag::PhotographicSensitivity, In::PRIMARY) {
            iso = field
                .value
                .get_uint(0)
                .or_else(|| todo!("PhotographicSensitivity {:?}", field.value));
        }
        if let Some(field) = exif.get_field(Tag::ExposureTime, In::PRIMARY) {
            shutter_speed = field
                .value
                .get_rational(0)
                .map_or_else(
                    || todo!("ExposureTime {:?}", field.value),
                    ShutterSpeed::from,
                )
                .into();
        }
        if let Some(field) = exif.get_field(Tag::FNumber, In::PRIMARY) {
            aperture = field
                .value
                .get_rational(0)
                .map_or_else(|| todo!("FNumber {:?}", field.value), Aperture::from)
                .into();
        }
        if let Some(field) = exif.get_field(Tag::FocalLength, In::PRIMARY) {
            focal_length = field
                .value
                .get_rational(0)
                .map_or_else(|| todo!("FocalLength {:?}", field.value), FocalLength::from)
                .into();
        }
        if let Some(field) = exif.get_field(Tag::LensModel, In::PRIMARY) {
            lens = field.value.get_string(0).map_or_else(
                || todo!("LensModel {:?}", field.value),
                |v| Lens::from(ctxt.string_interner.insert_or_get(v)).into(),
            );
        }
        if let Some(field) = exif.get_field(Tag::Model, In::PRIMARY) {
            camera = field.value.get_string(0).map_or_else(
                || todo!("Model {:?}", field.value),
                |v| Camera::from(ctxt.string_interner.insert_or_get(v)).into(),
            );
        }

        Self {
            iso,
            aperture,
            shutter_speed,
            focal_length,
            lens,
            camera,
        }
    }

    pub const fn iso(&self) -> Option<u32> {
        self.iso
    }

    pub const fn aperture(&self) -> Option<Aperture> {
        self.aperture
    }

    pub const fn shutter_speed(&self) -> Option<ShutterSpeed> {
        self.shutter_speed
    }

    pub const fn focal_length(&self) -> Option<FocalLength> {
        self.focal_length
    }

    pub const fn lens(&self) -> Option<&Lens> {
        self.lens.as_ref()
    }

    pub const fn camera(&self) -> Option<&Camera> {
        self.camera.as_ref()
    }
}

trait GetRational {
    fn get_rational(&self, index: usize) -> Option<Rational>;
}

impl GetRational for Value {
    fn get_rational(&self, index: usize) -> Option<Rational> {
        if let Self::Rational(r) = self {
            if let Some(exif::Rational { num, denom }) = r.get(index) {
                Some(Rational::new(*num, *denom))
            } else {
                None
            }
        } else {
            None
        }
    }
}

trait GetString {
    fn get_string(&self, index: usize) -> Option<&[u8]>;
}

impl GetString for Value {
    fn get_string(&self, index: usize) -> Option<&[u8]> {
        if let Self::Ascii(r) = self {
            r.get(index).map(|v| &**v)
        } else {
            None
        }
    }
}
