use super::{Part, Template, TemplateHeader};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "invalid header")]
    InvalidHeader,
    #[fail(display = "invalid input field")]
    InvalidInput,
}

pub struct Parser<'a> {
    buf: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(buf: &'a str) -> Parser<'a> {
        Parser {
            buf: buf.chars().peekable(),
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.buf.peek().cloned()
    }

    fn next(&mut self) -> Option<char> {
        self.buf.next()
    }

    fn take(&mut self, count: usize) -> String {
        let mut buf = String::new();
        let mut i = 0;
        while let Some(c) = self.next() {
            if i >= count {
                break;
            }
            i += 1;
            buf.push(c);
        }
        buf
    }

    fn skip_whitespace(&mut self) {
        while self.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
            self.next();
        }
    }

    pub fn parse(mut self) -> Result<Template, Error> {
        let header = self.parse_header()?;
        let parts = self.parse_parts()?;
        Ok(Template { header, parts })
    }

    fn parse_ident(&mut self) -> String {
        let mut buf = String::new();
        while self.peek().map(|c| c.is_alphanumeric()).unwrap_or(false) {
            buf.push(self.next().unwrap());
        }
        buf
    }

    fn parse_ident_list(&mut self, sep: char) -> Vec<String> {
        let mut vec = Vec::new();
        vec.push(self.parse_ident());
        self.skip_whitespace();
        while self.peek() == Some(sep) {
            self.next();
            vec.push(self.parse_ident());
            self.skip_whitespace();
        }
        vec
    }

    fn parse_input(&mut self) -> Result<Part, Error> {
        self.next();
        let mut inputs = Vec::new();
        loop {
            self.skip_whitespace();
            let mut buf = String::new();
            while self
                .peek()
                .map(|c| c != '}' && c != '|' && !c.is_whitespace())
                .unwrap_or(false)
            {
                buf.push(self.next().unwrap());
            }
            inputs.push(buf);
            self.skip_whitespace();
            match self.peek() {
                Some('}') => {
                    self.next();
                    break;
                }
                Some('|') => {
                    self.next();
                }
                _ => return Err(Error::InvalidInput),
            }
        }
        Ok(Part::Input(inputs))
    }

    fn parse_line(&mut self) -> Result<Vec<Part>, Error> {
        let mut parts = Vec::new();
        loop {
            match self.peek() {
                Some('{') => {
                    parts.push(self.parse_input()?);
                }
                Some('\n') => {
                    self.next();
                    break;
                }
                Some(_) => {
                    let mut buf = String::new();
                    while self.peek().map(|c| c != '\n' && c != '{').unwrap_or(false) {
                        buf.push(self.next().unwrap());
                    }
                    parts.push(Part::Str(buf));
                }
                None => break,
            }
        }
        Ok(parts)
    }

    pub fn parse_header(&mut self) -> Result<TemplateHeader, Error> {
        if self.take(3) != "---" {
            return Err(Error::InvalidHeader);
        }
        let mut header = TemplateHeader {
            name: Vec::new(),
            output: None,
            apply: None,
        };
        self.skip_whitespace();
        while self.peek() != Some('-') {
            let ident = self.parse_ident();
            self.skip_whitespace();
            if self.peek() != Some(':') {
                return Err(Error::InvalidHeader);
            }
            self.next();
            self.skip_whitespace();
            match ident.as_str() {
                "name" => header.name = self.parse_ident_list(','),
                "output" => header.output = Some(self.parse_line()?),
                "apply" => header.apply = Some(self.parse_line()?),
                i if i.is_empty() => return Err(Error::InvalidHeader),
                i => {
                    println!("unknown header field {}", i);
                }
            }
            self.skip_whitespace();
        }

        if self.take(3) != "---" || header.name.is_empty() {
            return Err(Error::InvalidHeader);
        }
        Ok(header)
    }

    pub fn parse_parts(&mut self) -> Result<Vec<Part>, Error> {
        let mut parts = Vec::new();
        loop {
            match self.peek() {
                Some('{') => {
                    parts.push(self.parse_input()?);
                }
                Some(_) => {
                    let mut buf = String::new();
                    while self.peek().map(|c| c != '{').unwrap_or(false) {
                        buf.push(self.next().unwrap());
                    }
                    parts.push(Part::Str(buf));
                }
                None => break,
            }
        }
        Ok(parts)
    }
}
