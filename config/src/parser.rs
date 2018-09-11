use super::{Config, Section, Value};
use lexer::Lexer;
use std::collections::HashMap;
use std::iter::Peekable;
use token::Token;

pub struct Parser<'a> {
    lexer: Peekable<&'a mut Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        Parser {
            lexer: lexer.peekable(),
        }
    }

    fn peek(&mut self) -> Option<Token> {
        self.lexer.peek().cloned()
    }

    fn eat(&mut self) -> Option<Token> {
        self.lexer.next()
    }

    pub fn parse(&mut self) -> Option<Config> {
        let mut sections = HashMap::new();
        loop {
            let name = match self.peek() {
                Some(Token::Ident(s)) => s,
                _ => break,
            };
            self.eat();
            sections.insert(name, self.parse_section()?);
        }
        Some(Config::new(sections))
    }

    fn parse_section(&mut self) -> Option<Section> {
        if self.eat() != Some(Token::LBrace) {
            return None;
        }
        let mut values = HashMap::new();
        while self.peek() != Some(Token::RBrace) {
            let name = match self.eat()? {
                Token::Ident(s) => s,
                _ => return None,
            };
            let value = self.parse_value()?;
            values.insert(name, value);
        }
        self.eat();
        Some(Section::new(values))
    }

    fn parse_value(&mut self) -> Option<Value> {
        match self.peek()? {
            Token::LBrace => Some(Value::Section(self.parse_section()?)),
            Token::Ident(s) => {
                self.eat()?;
                match s.as_str() {
                    "rgb" => {
                        let params = self.parse_params()?;
                        if params.len() != 3 {
                            return None;
                        }
                        let params: Option<Vec<u8>> = params
                            .into_iter()
                            .map(|v| match v {
                                Value::Hex(v) => Some((v & 255) as u8),
                                Value::Number(v) => Some(v),
                                _ => None,
                            }).collect();
                        let params = params?;
                        Some(Value::RGB(params[0], params[1], params[2]))
                    }
                    _ => Some(Value::Str(s)),
                }
            }
            Token::Str(s) => {
                self.eat()?;
                Some(Value::Str(s))
            }
            Token::Hex(s) => {
                self.eat()?;
                Some(Value::Hex(u32::from_str_radix(&s, 16).ok()?))
            }
            Token::Path(p) => {
                self.eat()?;
                Some(Value::Path(p))
            }
            t => unimplemented!("unimplemented parsing expression for token {:?}", t),
        }
    }

    fn parse_params(&mut self) -> Option<Vec<Value>> {
        let mut params = Vec::new();
        if self.eat() != Some(Token::LParen) {
            return None;
        }
        while let Some(t) = self.eat() {
            match t {
                Token::Ident(s) | Token::Str(s) => params.push(Value::Str(s)),
                Token::Hex(s) => params.push(Value::Hex(u32::from_str_radix(&s, 16).ok()?)),
                Token::Number(s) => params.push(Value::Number(s.parse().ok()?)),
                Token::Path(s) => params.push(Value::Path(s)),
                Token::RParen => break,
                _ => return None,
            }
            match self.eat()? {
                Token::Comma => continue,
                Token::RParen => break,
                _ => return None,
            }
        }
        Some(params)
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use lexer::Lexer;

    fn expect_value(buf: &str, expected: Value) {
        let mut lexer = Lexer::new(buf);
        let mut parser = Parser::new(&mut lexer);
        assert_eq!(parser.parse_value(), Some(expected));
    }

    fn expect_section(buf: &str, expected: Section) {
        let mut lexer = Lexer::new(buf);
        let mut parser = Parser::new(&mut lexer);
        assert_eq!(parser.parse_section(), Some(expected));
    }

    fn expect_config(buf: &str, expected: Config) {
        let mut lexer = Lexer::new(buf);
        let mut parser = Parser::new(&mut lexer);
        assert_eq!(parser.parse(), Some(expected));
    }

    // TODO:
    #[test]
    fn rgb() {
        expect_value("rgb(10, 200, 230)", Value::RGB(10, 200, 230));
    }

    #[test]
    fn section() {
        expect_section(
            "{
            method feh
            file /path/to/background
        }",
            Section::new(
                vec![
                    ("method".to_owned(), Value::Str("feh".into())),
                    ("file".to_owned(), Value::Path("/path/to/background".into())),
                ].into_iter()
                .collect(),
            ),
        );
    }

    #[test]
    fn config() {
        expect_config(
            "desktop {
                method feh
                file /path/to/background
            }",
            Config::new(
                vec![(
                    "desktop".into(),
                    Section::new(
                        vec![
                            ("method".to_owned(), Value::Str("feh".into())),
                            ("file".to_owned(), Value::Path("/path/to/background".into())),
                        ].into_iter()
                        .collect(),
                    ),
                )].into_iter()
                .collect(),
            ),
        );
    }

    #[test]
    fn two_sections() {
        expect_config(
            "desktop {
                method feh
                file /path/to/background
            }
            empty {}
            ",
            Config::new(
                vec![
                    (
                        "desktop".into(),
                        Section::new(
                            vec![
                                ("method".to_owned(), Value::Str("feh".into())),
                                ("file".to_owned(), Value::Path("/path/to/background".into())),
                            ].into_iter()
                            .collect(),
                        ),
                    ),
                    ("empty".into(), Section::new(HashMap::new())),
                ].into_iter()
                .collect(),
            ),
        );
    }
}
