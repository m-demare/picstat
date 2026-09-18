#[derive(Debug, PartialEq, PartialOrd, Eq, Clone, Copy)]
pub(super) enum Precedence {
    Lowest,
    Or,
    And,
    Comparator, // >, <, =
    Sum,        // +, -
    Product,    // *, /
    Prefix,     // !, - (unary)
}

pub(super) fn precedence_of_binary(op: &Token) -> Precedence {
    match op {
        Token::Or => Precedence::Or,
        Token::And => Precedence::And,
        Token::Eq | Token::Lt | Token::Gt => Precedence::Comparator,
        Token::Plus | Token::Minus => Precedence::Sum,
        Token::Times | Token::Div => Precedence::Product,
        _ => unreachable!("Invalid binary"),
    }
}

macro_rules! consume {
    ($compiler: expr, $allow_eof: expr, $($args: tt),+) => {
        match $compiler.next() {
            $(
                Some$args => {},
            )+
            Some(t) => return Err(ParseError::UnexpectedToken(t, $compiler.last_tok_pos)),
            None if $allow_eof => {},
            None => return Err(ParseError::UnexpectedEOF),
        }
    };
    ($compiler: expr; $($args: tt),+) => {
        consume!($compiler, false, $($args),+)
    };
    ($compiler: expr; allow_eof, $($args: tt),+) => {
        consume!($compiler, true, $($args),+)
    };
}

pub(super) use consume;

use crate::metadata_filter::lexer::Token;
