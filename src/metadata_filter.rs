use std::str::FromStr;

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
pub(crate) mod value;

#[derive(Clone, Debug)]
pub enum MetadataFilter {
    Boolean(bool),
    Number(f64),
    Not(Box<MetadataFilter>),
    Eq(Box<MetadataFilter>, Box<MetadataFilter>),
    Lt(Box<MetadataFilter>, Box<MetadataFilter>),
    Gt(Box<MetadataFilter>, Box<MetadataFilter>),
    Minus(Box<MetadataFilter>, Box<MetadataFilter>),
    Plus(Box<MetadataFilter>, Box<MetadataFilter>),
    Times(Box<MetadataFilter>, Box<MetadataFilter>),
    Div(Box<MetadataFilter>, Box<MetadataFilter>),
    And(Box<MetadataFilter>, Box<MetadataFilter>),
    Or(Box<MetadataFilter>, Box<MetadataFilter>),
    FnCall(String, Vec<MetadataFilter>),
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
            MetadataFilter::Boolean(b) => Ok(FilterValue::Boolean(*b)),
            MetadataFilter::Number(n) => Ok(FilterValue::Number(*n)),
            MetadataFilter::Not(a) => a.eval(metadata)?.not(),
            MetadataFilter::Eq(a, b) => comparison!(a, b, eq),
            MetadataFilter::Lt(a, b) => comparison!(a, b, lt),
            MetadataFilter::Gt(a, b) => comparison!(a, b, gt),
            MetadataFilter::Minus(a, b) => arithm!(a, b, minus),
            MetadataFilter::Plus(a, b) => arithm!(a, b, plus),
            MetadataFilter::Times(a, b) => arithm!(a, b, times),
            MetadataFilter::Div(a, b) => arithm!(a, b, div),
            MetadataFilter::And(a, b) => {
                self.bool_op(a.eval(metadata)?, b.eval(metadata)?, |a, b| a && b)
            }
            MetadataFilter::Or(a, b) => {
                self.bool_op(a.eval(metadata)?, b.eval(metadata)?, |a, b| a || b)
            }
            MetadataFilter::FnCall(ident, args) => self.fn_call(ident, args, metadata),
        }
    }

    fn bool_op(
        &self,
        a: FilterValue,
        b: FilterValue,
        f: impl Fn(bool, bool) -> bool,
    ) -> Result<FilterValue, FilterError> {
        Ok(FilterValue::Boolean(f(a.as_bool()?, b.as_bool()?)))
    }

    fn fn_call(
        &self,
        ident: &str,
        args: &[MetadataFilter],
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
