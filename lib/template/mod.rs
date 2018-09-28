mod parser;

pub use self::parser::*;

#[derive(Debug)]
pub enum Part {
    Str(String),
    Input(Vec<String>), // can have multiple fields
}

#[derive(Debug)]
pub struct TemplateHeader {
    pub name: Vec<String>,
    pub output: Option<Vec<Part>>,
    pub apply: Option<Vec<Part>>,
}

#[derive(Debug)]
pub struct Template {
    pub header: TemplateHeader,
    pub parts: Vec<Part>,
}
