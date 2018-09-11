extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate themer_config as config;

mod x11;

use config::{Config, Section, Value};
use std::collections::HashMap;
use std::path::PathBuf;
use x11::X11;

#[derive(Clone, Debug)]
pub struct Color(u8, u8, u8);

pub struct Colors {
    pub cursor: Color,
    pub cursor_foreground: Option<Color>,
    pub cursor_background: Option<Color>,

    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub highlight: Option<Color>,

    pub black: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub magenta: Color,
    pub cyan: Color,
    pub white: Color,

    pub bright_black: Option<Color>,
    pub bright_red: Option<Color>,
    pub bright_green: Option<Color>,
    pub bright_yellow: Option<Color>,
    pub bright_blue: Option<Color>,
    pub bright_magenta: Option<Color>,
    pub bright_cyan: Option<Color>,
    pub bright_white: Option<Color>,
}

impl Colors {
    pub fn get(&self, name: &str) -> Option<&Color> {
        match name {
            "cursor" => Some(&self.cursor),
            "cursor_foreground" => self.cursor_foreground.as_ref(),
            "cursor_background" => self.cursor_background.as_ref(),
            "foreground" => self.foreground.as_ref(),
            "background" => self.background.as_ref(),
            "highlight" => self.highlight.as_ref(),
            "black" => Some(&self.black),
            "red" => Some(&self.red),
            "green" => Some(&self.green),
            "yellow" => Some(&self.yellow),
            "blue" => Some(&self.blue),
            "magenta" => Some(&self.magenta),
            "cyan" => Some(&self.cyan),
            "white" => Some(&self.white),
            "bright_black" => self.bright_black.as_ref(),
            "bright_red" => self.bright_red.as_ref(),
            "bright_green" => self.bright_green.as_ref(),
            "bright_yellow" => self.bright_yellow.as_ref(),
            "bright_blue" => self.bright_blue.as_ref(),
            "bright_magenta" => self.bright_magenta.as_ref(),
            "bright_cyan" => self.bright_cyan.as_ref(),
            "bright_white" => self.bright_white.as_ref(),
        }
    }
}

static mut DEFINED: Option<HashMap<String, Value>> = None;
static mut COLORS: Option<Colors> = None;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "expected color found {:?}", value)]
    ExpectedColor { value: Value },
    #[fail(display = "unknown section `{}`", section)]
    UnknownSection { section: String },
}

// TODO: better name
pub trait Theme {
    fn available_fields(&self) -> &[&str];
    fn create(&mut self, &Section) -> Result<(), Error>;
    fn generated(&self) -> Result<String, Error>;
    fn apply(&self) -> Result<(), Error>;
    fn output(&self) -> Option<&PathBuf>;
}

pub(crate) fn expect_color(value: &Value) -> Result<Color, Error> {
    match value {
        Value::Hex(h) => Ok(Color(
            ((h & 0xff0000) >> 16) as u8,
            ((h & 0x00ff00) >> 8) as u8,
            (h & 0x0000ff) as u8,
        )),
        Value::RGB(r, g, b) => Ok(Color(*r, *g, *b)),
        v => Err(Error::ExpectedColor { value: v.clone() }),
    }
}

// Template?

pub(crate) fn resolve(name: &str) -> Option<Value> {
    unsafe {
        if let Some(ref def) = DEFINED {
            return def.get(name).cloned();
        }
    }
    None
}

pub(crate) fn get_color(name: &str) -> Option<Color> {
    unsafe {
        if let Some(color) = COLORS {
            return color.get(name).cloned();
        }
    }
    None
}

pub fn process_section(name: &str, section: &Section) -> Result<Option<Box<Theme>>, Error> {
    match name.to_lowercase().as_str() {
        "x11" | "xresources" => {
            let mut x11 = X11::new();
            x11.create(section)?;
            Ok(Some(Box::new(x11)))
        }
        "define" => {
            unsafe {
                DEFINED.get_or_insert_with(|| section.values().to_owned());
            }
            Ok(None)
        }
        _ => Err(Error::UnknownSection {
            section: name.to_owned(),
        }),
    }
}

pub fn process_config(config: &Config) -> Result<Vec<Box<Theme>>, Error> {
    let mut result = Vec::new();
    for (name, section) in config.sections() {
        if let Some(gen) = process_section(name, section)? {
            result.push(gen);
        }
    }
    Ok(result)
}
