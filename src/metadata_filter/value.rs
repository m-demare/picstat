use std::error::Error;

pub(super) enum FilterValue {
    Boolean(bool),
    Number(f64),
}
impl FilterValue {
    pub fn as_bool(&self) -> Result<bool, FilterError> {
        match self {
            FilterValue::Boolean(b) => Ok(*b),
            FilterValue::Number(n) => Ok(!Self::eq_f64(*n, 0.0)),
        }
    }

    pub fn as_number(&self) -> Result<f64, FilterError> {
        match self {
            FilterValue::Number(n) => Ok(*n),
            v => Err(FilterError::TypeError("number", v.get_type_str())),
        }
    }

    pub fn not(&self) -> Result<FilterValue, FilterError> {
        Ok(FilterValue::Boolean(!self.as_bool()?))
    }

    pub fn eq_f64(a: f64, b: f64) -> bool {
        const TOLERANCE: f64 = 0.000001;
        (a - b).abs() < TOLERANCE
    }

    pub fn eq(&self, other: &FilterValue) -> Result<bool, FilterError> {
        match (self, other) {
            (FilterValue::Boolean(a), FilterValue::Boolean(b)) => Ok(a == b),
            (FilterValue::Number(a), FilterValue::Number(b)) => Ok(Self::eq_f64(*a, *b)),
            (a, b) => Err(FilterError::TypeError(
                "matching types",
                format!("{} and {}", a.get_type_str(), b.get_type_str()),
            )),
        }
    }

    pub fn lt(&self, other: &FilterValue) -> Result<bool, FilterError> {
        match (self, other) {
            (FilterValue::Number(a), FilterValue::Number(b)) => Ok(a < b),
            (a, b) => Err(FilterError::TypeError(
                "numbers",
                format!("{} and {}", a.get_type_str(), b.get_type_str()),
            )),
        }
    }

    pub fn gt(&self, other: &FilterValue) -> Result<bool, FilterError> {
        match (self, other) {
            (FilterValue::Number(a), FilterValue::Number(b)) => Ok(a > b),
            (a, b) => Err(FilterError::TypeError(
                "numbers",
                format!("{} and {}", a.get_type_str(), b.get_type_str()),
            )),
        }
    }

    pub fn minus(&self, other: &FilterValue) -> Result<FilterValue, FilterError> {
        match (self, other) {
            (FilterValue::Number(a), FilterValue::Number(b)) => Ok(FilterValue::Number(a - b)),
            (a, b) => Err(FilterError::TypeError(
                "numbers",
                format!("{} and {}", a.get_type_str(), b.get_type_str()),
            )),
        }
    }

    pub fn plus(&self, other: &FilterValue) -> Result<FilterValue, FilterError> {
        match (self, other) {
            (FilterValue::Number(a), FilterValue::Number(b)) => Ok(FilterValue::Number(a + b)),
            (a, b) => Err(FilterError::TypeError(
                "numbers",
                format!("{} and {}", a.get_type_str(), b.get_type_str()),
            )),
        }
    }

    pub fn times(&self, other: &FilterValue) -> Result<FilterValue, FilterError> {
        match (self, other) {
            (FilterValue::Number(a), FilterValue::Number(b)) => Ok(FilterValue::Number(a * b)),
            (a, b) => Err(FilterError::TypeError(
                "numbers",
                format!("{} and {}", a.get_type_str(), b.get_type_str()),
            )),
        }
    }

    pub fn div(&self, other: &FilterValue) -> Result<FilterValue, FilterError> {
        match (self, other) {
            (FilterValue::Number(a), FilterValue::Number(b)) => Ok(FilterValue::Number(a / b)),
            (a, b) => Err(FilterError::TypeError(
                "numbers",
                format!("{} and {}", a.get_type_str(), b.get_type_str()),
            )),
        }
    }

    fn get_type_str(&self) -> String {
        match self {
            FilterValue::Boolean(_) => "Boolean",
            FilterValue::Number(_) => "Number",
        }
        .to_owned()
    }
}

#[derive(Debug)]
pub enum FilterError {
    TypeError(&'static str, String),
    UnknownValue(String),
}

impl std::fmt::Display for FilterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error applying file filter: ")?;
        match self {
            FilterError::TypeError(expected, got) => {
                write!(f, "Type error: expected {expected} and got {got}")
            }
            FilterError::UnknownValue(v) => todo!(),
        }
    }
}

impl Error for FilterError {}
