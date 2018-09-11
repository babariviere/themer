pub mod lexer;
pub mod parser;
pub mod token;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    Hex(u32),
    Number(u8),
    RGB(u8, u8, u8),
    Str(String),
    Path(String),
    Section(Section),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Section(HashMap<String, Value>);

impl Section {
    pub fn new(values: HashMap<String, Value>) -> Self {
        Section(values)
    }

    pub fn values(&self) -> &HashMap<String, Value> {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
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
