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
        while let Some(section) = self.parse_section() {
            let name = match self.peek()? {
                Token::Ident(s) => s,
                _ => break,
            };
            self.eat();
            sections.insert(name, section);
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
            Token::Ident(s) | Token::Str(s) => {
                // TODO: check for rgb ident
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
}

#[cfg(test)]
mod unit_tests {
    // TODO:
}
