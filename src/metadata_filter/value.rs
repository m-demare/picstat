use std::error::Error;

pub(super) enum FilterValue {
    Boolean(bool),
    Number(f64),
}
impl FilterValue {
    pub fn as_bool(&self) -> Result<bool, FilterError> {
        match self {
            Self::Boolean(b) => Ok(*b),
            Self::Number(n) => Ok(!Self::eq_f64(*n, 0.0)),
        }
    }

    pub fn not(&self) -> Result<Self, FilterError> {
        Ok(Self::Boolean(!self.as_bool()?))
    }

    pub fn eq_f64(a: f64, b: f64) -> bool {
        const TOLERANCE: f64 = 0.000_001;
        (a - b).abs() < TOLERANCE
    }

    pub fn eq(&self, other: &Self) -> Result<bool, FilterError> {
        match (self, other) {
            (Self::Boolean(a), Self::Boolean(b)) => Ok(a == b),
            (Self::Number(a), Self::Number(b)) => Ok(Self::eq_f64(*a, *b)),
            (a, b) => Err(FilterError::TypeError(
                "matching types",
                format!("{} and {}", a.get_type_str(), b.get_type_str()),
            )),
        }
    }

    pub fn lt(&self, other: &Self) -> Result<bool, FilterError> {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Ok(a < b),
            (a, b) => Err(FilterError::TypeError(
                "numbers",
                format!("{} and {}", a.get_type_str(), b.get_type_str()),
            )),
        }
    }

    pub fn gt(&self, other: &Self) -> Result<bool, FilterError> {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Ok(a > b),
            (a, b) => Err(FilterError::TypeError(
                "numbers",
                format!("{} and {}", a.get_type_str(), b.get_type_str()),
            )),
        }
    }

    pub fn minus(&self, other: &Self) -> Result<Self, FilterError> {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Ok(Self::Number(a - b)),
            (a, b) => Err(FilterError::TypeError(
                "numbers",
                format!("{} and {}", a.get_type_str(), b.get_type_str()),
            )),
        }
    }

    pub fn plus(&self, other: &Self) -> Result<Self, FilterError> {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Ok(Self::Number(a + b)),
            (a, b) => Err(FilterError::TypeError(
                "numbers",
                format!("{} and {}", a.get_type_str(), b.get_type_str()),
            )),
        }
    }

    pub fn times(&self, other: &Self) -> Result<Self, FilterError> {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Ok(Self::Number(a * b)),
            (a, b) => Err(FilterError::TypeError(
                "numbers",
                format!("{} and {}", a.get_type_str(), b.get_type_str()),
            )),
        }
    }

    pub fn div(&self, other: &Self) -> Result<Self, FilterError> {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Ok(Self::Number(a / b)),
            (a, b) => Err(FilterError::TypeError(
                "numbers",
                format!("{} and {}", a.get_type_str(), b.get_type_str()),
            )),
        }
    }

    fn get_type_str(&self) -> String {
        match self {
            Self::Boolean(_) => "Boolean",
            Self::Number(_) => "Number",
        }
        .to_owned()
    }
}

#[derive(Debug)]
pub enum FilterError {
    TypeError(&'static str, String),
    UnknownIdentifier(String),
    EmptyField(&'static str),
}

impl std::fmt::Display for FilterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error applying file filter: ")?;
        match self {
            Self::TypeError(expected, got) => {
                write!(f, "Type error: expected {expected} and got {got}")
            }
            Self::UnknownIdentifier(id) => write!(f, "Unknown function identifier: {id}"),
            Self::EmptyField(id) => write!(f, "File has empty field {id}"),
        }
    }
}

impl Error for FilterError {}
