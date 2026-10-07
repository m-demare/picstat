use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Boolean(bool),
    Number(f64),
    Identifier(String),
    String(String),
    Error,
    Eq,
    NotEq,
    Lt,
    Gt,
    Le,
    Ge,
    Minus,
    Plus,
    Times,
    Div,
    And,
    Or,
    Not,
    Lparen,
    Rparen,
    Comma,
}

pub struct Lexer<I: Iterator<Item = char>> {
    input: Peekable<I>,
    pos: usize,
}

impl<I: Iterator<Item = char>> Lexer<I> {
    fn peek(&mut self) -> Option<char> {
        self.input.peek().copied()
    }

    fn next(&mut self) -> Option<char> {
        self.pos += 1;
        self.input.next()
    }

    fn read_number(&mut self) -> Token {
        let mut whole = 0f64;
        while let Some(ch) = self.peek()
            && let Some(n) = ch.to_digit(10)
        {
            self.next();
            whole *= 10.0;
            whole += f64::from(n);
        }

        let mut decimal = 0f64;
        let mut decimal_multiplier = 1f64;
        if self.peek() == Some('.') {
            self.next();

            while let Some(ch) = self.peek()
                && let Some(n) = ch.to_digit(10)
            {
                self.next();
                decimal_multiplier /= 10.0;
                decimal = f64::mul_add(f64::from(n), decimal_multiplier, decimal);
            }
        }

        Token::Number(whole + decimal)
    }

    fn read_identifier(&mut self) -> Token {
        let mut res = String::new();
        while let Some(ch) = self.peek()
            && ch.is_ascii_alphabetic()
        {
            self.next();
            res.push(ch);
        }

        match res.as_str() {
            "" => Token::Error,
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),
            _ => Token::Identifier(res),
        }
    }

    fn read_string(&mut self) -> Token {
        let mut res = String::new();
        self.next();
        while let Some(ch) = self.peek()
            && ch != '"'
        {
            self.next();
            res.push(ch);
        }

        match self.next() {
            Some('"') => Token::String(res),
            Some(_) => unreachable!("Error parsing string"),
            None => Token::Error,
        }
    }

    fn read_comparator(&mut self, ch: char) -> Token {
        self.next();
        let eq = self.peek() == Some('=');
        if eq {
            self.next();
        }

        match (ch, eq) {
            ('<', false) => Token::Lt,
            ('<', true) => Token::Le,
            ('>', false) => Token::Gt,
            ('>', true) => Token::Ge,

            ('!', false) => Token::Not,
            ('!', true) => Token::NotEq,
            ('=', true) => Token::Eq,

            _ => Token::Error,
        }
    }

    const fn single_char_token(ch: char) -> Token {
        match ch {
            '=' => Token::Eq,
            '<' => Token::Lt,
            '>' => Token::Gt,
            '-' => Token::Minus,
            '+' => Token::Plus,
            '*' => Token::Times,
            '/' => Token::Div,
            '&' => Token::And,
            '|' => Token::Or,
            '!' => Token::Not,
            '(' => Token::Lparen,
            ')' => Token::Rparen,
            ',' => Token::Comma,
            _ => Token::Error,
        }
    }

    pub const fn curr_pos(&self) -> usize {
        self.pos
    }
}

impl<I: Iterator<Item = char>> From<I> for Lexer<I> {
    fn from(input: I) -> Self {
        Self {
            input: input.peekable(),
            pos: 0,
        }
    }
}

impl<I: Iterator<Item = char>> Iterator for Lexer<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while self.peek()?.is_whitespace() {
            self.next();
        }

        Some(match self.peek()? {
            '0'..='9' => self.read_number(),
            'a'..='z' => self.read_identifier(),
            '"' => self.read_string(),
            ch @ ('<' | '>' | '!' | '=') => self.read_comparator(ch),
            ch => {
                self.next();
                Self::single_char_token(ch)
            }
        })
    }
}
