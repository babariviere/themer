use std::iter::Peekable;
use std::str::Chars;
use token::Token;

pub struct Lexer<'a> {
    buf: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(buf: &'a str) -> Self {
        Lexer {
            buf: buf.chars().peekable(),
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.buf.peek().cloned()
    }

    fn eat(&mut self) -> Option<char> {
        self.buf.next()
    }

    pub fn next_token(&mut self) -> Option<Token> {
        while self.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
            self.eat();
        }
        match self.peek()? {
            c if is_start_of_path(c) => self.lex_path(),
            c if c.is_alphabetic() => self.lex_ident(),
            c if c.is_numeric() => self.lex_number(),
            '"' => self.lex_str(),
            '#' => self.lex_hex(),
            '{' => self.lex_simple(Token::LBrace),
            '}' => self.lex_simple(Token::RBrace),
            '(' => self.lex_simple(Token::LParen),
            ')' => self.lex_simple(Token::RParen),
            ',' => self.lex_simple(Token::Comma),
            _ => None,
        }
    }

    fn lex_path(&mut self) -> Option<Token> {
        let mut path = String::new();
        // TODO: allow for escaped whitespace

        while let Some(c) = self.eat() {
            if c.is_whitespace() {
                break;
            }
            if c == '\\' && self.peek() == Some(' ') {
                path.push(self.eat().unwrap());
            } else {
                path.push(c);
            }
        }
        Some(Token::Path(path))
    }

    fn lex_ident(&mut self) -> Option<Token> {
        let mut ident = String::new();
        while self
            .peek()
            .map(|c| c.is_alphanumeric() || c == '_')
            .unwrap_or(false)
        {
            ident.push(self.eat().unwrap());
        }
        Some(Token::Ident(ident))
    }

    fn lex_number(&mut self) -> Option<Token> {
        let mut number = String::new();
        while self.peek().map(|c| c.is_numeric()).unwrap_or(false) {
            number.push(self.eat().unwrap());
        }
        Some(Token::Number(number))
    }

    fn lex_hex(&mut self) -> Option<Token> {
        let mut hex = String::new();
        if self.eat()? != '#' {
            return None;
        }
        while self.peek().map(|c| is_hex(c)).unwrap_or(false) {
            hex.push(self.eat().unwrap());
        }
        Some(Token::Hex(hex))
    }

    fn lex_str(&mut self) -> Option<Token> {
        let mut s = String::new();
        if self.eat()? != '"' {
            return None;
        }
        while let Some(c) = self.eat() {
            if c == '"' {
                break;
            }
            if c == '\\' && self.peek() == Some('"') {
                self.eat();
                s.push('"');
                continue;
            }
            s.push(c);
        }
        if let Some(fc) = s.chars().next() {
            if is_start_of_path(fc) {
                return Some(Token::Path(s));
            }
        }
        Some(Token::Str(s))
    }

    fn lex_simple(&mut self, token: Token) -> Option<Token> {
        self.eat()?;
        Some(token)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

fn is_hex(c: char) -> bool {
    c.is_numeric() || 'a' <= c && c <= 'f' || 'A' <= c && c <= 'F'
}

fn is_start_of_path(c: char) -> bool {
    c == '~' || c == '/' || c == '.'
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    fn eval(buf: &str, tokens: &[Token]) {
        let mut lexer = Lexer::new(buf);
        for token in tokens.into_iter() {
            assert_eq!(lexer.next_token().as_ref(), Some(token));
        }
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn simple() {
        eval(
            "colors { cursor black }",
            &[
                Token::Ident("colors".into()),
                Token::LBrace,
                Token::Ident("cursor".into()),
                Token::Ident("black".into()),
                Token::RBrace,
            ],
        );
    }

    #[test]
    fn complex() {
        eval(
            "colors {
                cursor black
                foreground #ffff00
            }
            desktop {
                method \"feh\"
                file /path/to/background
                path \"/another/path\"
            }
            ",
            &[
                Token::Ident("colors".into()),
                Token::LBrace,
                Token::Ident("cursor".into()),
                Token::Ident("black".into()),
                Token::Ident("foreground".into()),
                Token::Hex("ffff00".into()),
                Token::RBrace,
                Token::Ident("desktop".into()),
                Token::LBrace,
                Token::Ident("method".into()),
                Token::Str("feh".into()),
                Token::Ident("file".into()),
                Token::Path("/path/to/background".into()),
                Token::Ident("path".into()),
                Token::Path("/another/path".into()),
                Token::RBrace,
            ],
        );
    }
}
