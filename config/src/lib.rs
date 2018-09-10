pub mod lexer;
pub mod token;

use std::collections::HashMap;

#[derive(Debug)]
pub enum Value {
    Hex(u32),
    RGB(u8, u8, u8),
    Str(String),
    Section(Section),
}

#[derive(Debug)]
pub struct Section(HashMap<String, Value>);

#[derive(Debug)]
pub struct Config {
    sections: Vec<Section>,
}
