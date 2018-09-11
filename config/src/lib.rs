pub mod lexer;
pub mod parser;
pub mod token;

use lexer::Lexer;
use parser::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Hex(u32),
    Number(u8),
    RGB(u8, u8, u8),
    Str(String),
    Path(String),
    Section(Section),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Section(HashMap<String, Value>);

impl Section {
    pub fn new(values: HashMap<String, Value>) -> Self {
        Section(values)
    }

    pub fn values(&self) -> &HashMap<String, Value> {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    sections: HashMap<String, Section>,
}

impl Config {
    pub fn new(sections: HashMap<String, Section>) -> Self {
        Config { sections }
    }

    pub fn sections(&self) -> &HashMap<String, Section> {
        &self.sections
    }
}

pub fn read_config<P: AsRef<Path>>(path: P) -> Result<Option<Config>> {
    let mut file = File::open(path.as_ref())?;
    read_stream(&mut file)
}

pub fn read_stream<R: Read>(reader: &mut R) -> Result<Option<Config>> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let mut lexer = Lexer::new(&buf);
    let mut parser = Parser::new(&mut lexer);
    Ok(parser.parse())
}
