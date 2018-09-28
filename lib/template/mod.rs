mod parser;

pub use self::parser::*;
pub use super::Color;
use config::map::Map;

pub fn process_parts(parts: Vec<Part>, map: &Map<Color>) -> Option<String> {
    let mut result_vec = Vec::new();
    'main: for part in parts {
        match part {
            Part::Str(s) => result_vec.push(s),
            Part::Input(inputs) => {
                for input in &inputs {
                    if let Some(val) = map.get(&input) {
                        result_vec.push(format!("#{:02x}{:02x}{:02x}", val.0, val.1, val.2));
                        continue 'main;
                    }
                }
                println!("missing color, list of inputs : {:?}", inputs);
                println!("{:#?}", map);
                return None; // nothing found
            }
        }
    }
    Some(result_vec.join(""))
}

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
