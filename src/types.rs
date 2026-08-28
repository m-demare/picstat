use std::{fmt::Display, rc::Rc};

use dhist::bucketers::log_bucketer::AproxF64;
use nom_exif::URational;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rational {
    pub num: u32,
    pub denom: u32,
}

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_f64())
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Rational {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_f64()
            .partial_cmp(&other.to_f64())
            .expect("Rationals can't be NaN")
    }
}

impl Rational {
    pub fn new(num: u32, denom: u32) -> Self {
        Self::safe_new(num, denom).unwrap_or_else(|| panic!("Invalid rational {num}/{denom}"))
    }

    fn safe_new(num: u32, denom: u32) -> Option<Self> {
        match denom {
            0 => None,
            _ => Self { num, denom }.into(),
        }
    }

    pub fn to_f64(self) -> f64 {
        f64::from(self.num) / f64::from(self.denom)
    }
}

impl From<URational> for Rational {
    fn from(value: URational) -> Self {
        Self::new(value.numerator(), value.denominator())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Aperture(Rational);

impl Display for Aperture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "f/{}", self.0)
    }
}

impl<T: Into<Rational>> From<T> for Aperture {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl Aperture {
    pub fn to_f64(self) -> f64 {
        self.0.to_f64()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ShutterSpeed(Rational);

impl Display for ShutterSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.to_f64() > 1.0 {
            write!(f, "{} s", self.0)
        } else {
            write!(f, "{}/{} s", self.0.num, self.0.denom)
        }
    }
}

impl<T: Into<Rational>> From<T> for ShutterSpeed {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl ShutterSpeed {
    pub fn to_f64(self) -> f64 {
        self.0.to_f64()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FocalLength(Rational);

impl Display for FocalLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}mm", self.0)
    }
}

impl<T: Into<Rational>> From<T> for FocalLength {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl FocalLength {
    pub fn to_f64(self) -> f64 {
        self.0.to_f64()
    }
}

#[derive(Debug, Clone, PartialOrd, Ord)]
pub struct Lens(Rc<String>);

impl Display for Lens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for Lens {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for Lens {}

impl From<Rc<String>> for Lens {
    fn from(value: Rc<String>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, PartialOrd, Ord)]
pub struct Camera(Rc<String>);

impl Display for Camera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for Camera {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for Camera {}

impl From<Rc<String>> for Camera {
    fn from(value: Rc<String>) -> Self {
        Self(value)
    }
}

impl AproxF64 for Aperture {
    fn aprox(&self) -> f64 {
        self.to_f64()
    }
}

impl AproxF64 for ShutterSpeed {
    fn aprox(&self) -> f64 {
        self.to_f64()
    }
}

impl AproxF64 for FocalLength {
    fn aprox(&self) -> f64 {
        self.to_f64()
    }
}
