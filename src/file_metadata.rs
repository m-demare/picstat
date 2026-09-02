use std::{io, path::Path};

use nom_exif::{EntryValue, ExifTag};

use crate::{
    context::Context,
    types::{Aperture, Camera, FocalLength, Lens, ShutterSpeed},
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
    pub fn from_exif(ctxt: &Context, path: &Path) -> Result<Self, io::Error> {
        let exif = wrap_exif_err(nom_exif::read_exif(path))?;
        Ok(Self::parse_exif(&exif, ctxt))
    }

    fn parse_exif(exif: &nom_exif::Exif, ctxt: &Context) -> Self {
        let iso = exif
            .get(ExifTag::ISOSpeedRatings)
            .and_then(EntryValue::try_as_integer)
            .and_then(|v| v.try_into().ok());
        let aperture = exif
            .get(ExifTag::FNumber)
            .and_then(EntryValue::as_urational)
            .map(Aperture::from);
        let shutter_speed = exif
            .get(ExifTag::ExposureTime)
            .and_then(EntryValue::as_urational)
            .map(ShutterSpeed::from);
        let focal_length = exif
            .get(ExifTag::FocalLength)
            .and_then(EntryValue::as_urational)
            .map(FocalLength::from);
        let lens = exif
            .get(ExifTag::LensModel)
            .and_then(EntryValue::as_str)
            .map(|v| Lens::from(ctxt.string_interner.insert_or_get(v)));
        let camera = exif
            .get(ExifTag::Model)
            .and_then(EntryValue::as_str)
            .map(|v| Camera::from(ctxt.string_interner.insert_or_get(v)));

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

fn wrap_exif_err<T>(res: Result<T, nom_exif::Error>) -> io::Result<T> {
    res.map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}
