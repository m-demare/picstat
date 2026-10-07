use std::{error::Error, sync::Arc};

use crate::metadata_filter::{
    MetadataFilter,
    lexer::{Lexer, Token},
    utils::{Precedence, consume, precedence_of_binary},
};

pub struct Parser<I: Iterator<Item = char>> {
    tokens: Lexer<I>,
    peeked_token: Option<Token>,
    last_tok_pos: usize,
}

type ParseResult = Result<super::MetadataFilter, ParseError>;

impl<I: Iterator<Item = char>> Parser<I> {
    pub(crate) fn new(mut tokens: Lexer<I>) -> Self {
        let peeked_token = tokens.next();
        let last_tok_pos = tokens.curr_pos();
        Self {
            tokens,
            peeked_token,
            last_tok_pos,
        }
    }

    const fn peek(&self) -> Option<&Token> {
        self.peeked_token.as_ref()
    }

    fn next(&mut self) -> Option<Token> {
        let retval = std::mem::replace(&mut self.peeked_token, self.tokens.next());
        self.last_tok_pos = self.tokens.curr_pos();
        retval
    }

    pub(crate) fn parse(&mut self) -> ParseResult {
        self.expression(Precedence::Lowest)
    }

    fn expression(&mut self, precedence: Precedence) -> ParseResult {
        let prefix = self.prefix_exp()?;
        self.infix_exp(prefix, precedence)
    }

    fn prefix_exp(&mut self) -> ParseResult {
        match self.next() {
            Some(Token::Boolean(b)) => Ok(MetadataFilter::Boolean(b)),
            Some(Token::Number(n)) => Ok(MetadataFilter::Number(n)),
            Some(Token::String(s)) => Ok(MetadataFilter::String(Arc::new(s))),
            Some(Token::Identifier(s)) => self.fn_call(s),
            Some(Token::Lparen) => self.group_exp(),
            Some(t @ Token::Not) => self.unary(t),

            Some(t) => Err(ParseError::UnexpectedToken(t, self.last_tok_pos)),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn fn_call(&mut self, ident: String) -> ParseResult {
        consume!(self; (Token::Lparen));
        let mut args = Vec::new();
        if self.peek() != Some(&Token::Rparen) {
            args.push(self.expression(Precedence::Lowest)?);
        }
        while Some(&Token::Comma) == self.peek() {
            self.next();
            args.push(self.expression(Precedence::Lowest)?);
        }
        consume!(self; (Token::Rparen));
        Ok(MetadataFilter::FnCall(ident, args))
    }

    fn unary(&mut self, t: Token) -> ParseResult {
        let inner = self.expression(Precedence::Prefix)?;
        match t {
            Token::Not => Ok(MetadataFilter::Not(inner.into())),
            t => unreachable!("Invalid unary {t:?}"),
        }
    }

    fn group_exp(&mut self) -> ParseResult {
        let res = self.expression(Precedence::Lowest)?;
        consume!(self; (Token::Rparen));
        Ok(res)
    }

    fn infix_exp(&mut self, mut lhs: MetadataFilter, precedence: Precedence) -> ParseResult {
        macro_rules! validate_precedence {
            ($t: expr) => {{
                let new_precedence = precedence_of_binary(&$t);
                if precedence >= new_precedence {
                    break Ok(lhs);
                }
                self.next();
                new_precedence
            }};
        }
        loop {
            match self.peek() {
                Some(t @ Token::Eq) => {
                    let new_precedence = validate_precedence!(t);
                    lhs = self.binary(lhs, MetadataFilter::Eq, new_precedence)?;
                }
                Some(t @ Token::NotEq) => {
                    let new_precedence = validate_precedence!(t);
                    lhs = self.binary(lhs, MetadataFilter::NotEq, new_precedence)?;
                }
                Some(t @ Token::Lt) => {
                    let new_precedence = validate_precedence!(t);
                    lhs = self.binary(lhs, MetadataFilter::Lt, new_precedence)?;
                }
                Some(t @ Token::Gt) => {
                    let new_precedence = validate_precedence!(t);
                    lhs = self.binary(lhs, MetadataFilter::Gt, new_precedence)?;
                }
                Some(t @ Token::Le) => {
                    let new_precedence = validate_precedence!(t);
                    lhs = self.binary(lhs, MetadataFilter::Le, new_precedence)?;
                }
                Some(t @ Token::Ge) => {
                    let new_precedence = validate_precedence!(t);
                    lhs = self.binary(lhs, MetadataFilter::Ge, new_precedence)?;
                }
                Some(t @ Token::Minus) => {
                    let new_precedence = validate_precedence!(t);
                    lhs = self.binary(lhs, MetadataFilter::Minus, new_precedence)?;
                }
                Some(t @ Token::Plus) => {
                    let new_precedence = validate_precedence!(t);
                    lhs = self.binary(lhs, MetadataFilter::Plus, new_precedence)?;
                }
                Some(t @ Token::Times) => {
                    let new_precedence = validate_precedence!(t);
                    lhs = self.binary(lhs, MetadataFilter::Times, new_precedence)?;
                }
                Some(t @ Token::Div) => {
                    let new_precedence = validate_precedence!(t);
                    lhs = self.binary(lhs, MetadataFilter::Div, new_precedence)?;
                }
                Some(t @ Token::And) => {
                    let new_precedence = validate_precedence!(t);
                    lhs = self.binary(lhs, MetadataFilter::And, new_precedence)?;
                }
                Some(t @ Token::Or) => {
                    let new_precedence = validate_precedence!(t);
                    lhs = self.binary(lhs, MetadataFilter::Or, new_precedence)?;
                }

                Some(_) | None => break Ok(lhs),
            }
        }
    }

    fn binary(
        &mut self,
        lhs: MetadataFilter,
        instr: fn(Box<MetadataFilter>, Box<MetadataFilter>) -> MetadataFilter,
        precedence: Precedence,
    ) -> ParseResult {
        let rhs = self.expression(precedence)?;
        Ok(instr(lhs.into(), rhs.into()))
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEOF,
    UnexpectedToken(Token, usize),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Encountered error when parsing filter: ")?;
        match self {
            Self::UnexpectedEOF => write!(f, "Unexpected end of expression"),
            Self::UnexpectedToken(Token::Error, offset) => {
                write!(f, "Invalid token at offset {offset}")
            }
            Self::UnexpectedToken(t, offset) => {
                write!(f, "Unexpected token {t:?} at offset {offset}")
            }
        }
    }
}

impl Error for ParseError {}
