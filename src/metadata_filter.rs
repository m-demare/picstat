use std::{str::FromStr, sync::Arc};

use crate::{
    file_metadata::FileMetadata,
    metadata_filter::{
        lexer::Lexer,
        native_functions::lookup_fn,
        parser::{ParseError, Parser},
        value::{FilterError, FilterValue},
    },
};

mod lexer;
mod native_functions;
mod parser;
mod utils;
pub mod value;

#[derive(Clone, Debug)]
pub enum MetadataFilter {
    Boolean(bool),
    Number(f64),
    String(Arc<String>),
    Not(Box<Self>),
    Eq(Box<Self>, Box<Self>),
    NotEq(Box<Self>, Box<Self>),
    Lt(Box<Self>, Box<Self>),
    Gt(Box<Self>, Box<Self>),
    Le(Box<Self>, Box<Self>),
    Ge(Box<Self>, Box<Self>),
    Minus(Box<Self>, Box<Self>),
    Plus(Box<Self>, Box<Self>),
    Times(Box<Self>, Box<Self>),
    Div(Box<Self>, Box<Self>),
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    FnCall(String, Vec<Self>),
}

impl MetadataFilter {
    pub fn accepts(&self, metadata: &FileMetadata) -> Result<bool, FilterError> {
        self.eval(metadata)?.as_bool()
    }

    fn eval(&self, metadata: &FileMetadata) -> Result<FilterValue, FilterError> {
        macro_rules! comparison {
            ($a: ident, $b: ident, $fn: ident) => {
                $a.eval(metadata)?
                    .$fn(&$b.eval(metadata)?)
                    .map(FilterValue::Boolean)
            };
        }
        macro_rules! arithm {
            ($a: ident, $b: ident, $fn: ident) => {
                $a.eval(metadata)?.$fn(&$b.eval(metadata)?)
            };
        }
        match self {
            Self::Boolean(b) => Ok(FilterValue::Boolean(*b)),
            Self::Number(n) => Ok(FilterValue::Number(*n)),
            Self::String(s) => Ok(FilterValue::String(s.clone())),
            Self::Not(a) => a.eval(metadata)?.not(),
            Self::Eq(a, b) => comparison!(a, b, eq),
            Self::NotEq(a, b) => comparison!(a, b, neq),
            Self::Lt(a, b) => comparison!(a, b, lt),
            Self::Gt(a, b) => comparison!(a, b, gt),
            Self::Le(a, b) => comparison!(a, b, le),
            Self::Ge(a, b) => comparison!(a, b, ge),
            Self::Minus(a, b) => arithm!(a, b, minus),
            Self::Plus(a, b) => arithm!(a, b, plus),
            Self::Times(a, b) => arithm!(a, b, times),
            Self::Div(a, b) => arithm!(a, b, div),
            Self::And(a, b) => Self::bool_op(&a.eval(metadata)?, &b.eval(metadata)?, |a, b| a && b),
            Self::Or(a, b) => Self::bool_op(&a.eval(metadata)?, &b.eval(metadata)?, |a, b| a || b),
            Self::FnCall(ident, args) => Self::fn_call(ident, args, metadata),
        }
    }

    fn bool_op(
        a: &FilterValue,
        b: &FilterValue,
        f: impl Fn(bool, bool) -> bool,
    ) -> Result<FilterValue, FilterError> {
        Ok(FilterValue::Boolean(f(a.as_bool()?, b.as_bool()?)))
    }

    fn fn_call(
        ident: &str,
        args: &[Self],
        metadata: &FileMetadata,
    ) -> Result<FilterValue, FilterError> {
        let func =
            lookup_fn(ident).ok_or_else(|| FilterError::UnknownIdentifier(ident.to_owned()))?;
        let args = args
            .iter()
            .map(|a| a.eval(metadata))
            .collect::<Result<Vec<_>, _>>()?;
        func(metadata, args)
    }
}

impl FromStr for MetadataFilter {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokenizer = Lexer::from(s.chars());
        Parser::new(tokenizer).parse()
    }
}
