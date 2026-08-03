use std::fmt::Display;

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
