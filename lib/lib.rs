extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate themer_config as config;

mod x11;

use config::{map::Map, Config, Section, Value};
use std::collections::VecDeque;
use std::path::PathBuf;
use x11::X11;

#[derive(Clone, Debug)]
pub struct Color(u8, u8, u8);

//const COLOR_NAMES: &[&str] = &[
//    "cursor",
//    "cursor_foreground",
//    "cursor_background",
//    "foreground",
//    "background",
//    "highlight",
//    "black",
//    "red",
//    "green",
//    "yellow",
//    "blue",
//    "magenta",
//    "cyan",
//    "white",
//    "bright_black",
//    "bright_red",
//    "bright_green",
//    "bright_yellow",
//    "bright_blue",
//    "bright_magenta",
//    "bright_cyan",
//    "bright_white",
//];
//
//fn is_color_name(name: &str) -> bool {
//    for color in COLOR_NAMES {
//        if &name == color {
//            return true;
//        }
//    }
//    false
//}

#[derive(Debug)]
pub struct State {
    pub colors: Map<Color>,
    pub defined: Map<Value>,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "expected color found {:?}", _0)]
    ExpectedColor(Value),
    #[fail(display = "unknown section `{}`", _0)]
    UnknownSection(String),
    #[fail(display = "io error: {}", _0)]
    Io(#[cause] ::std::io::Error),
}

impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Self {
        Error::Io(err)
    }
}

// TODO: better name
pub trait Theme: ::std::fmt::Debug {
    fn available_fields(&self) -> &[&str];
    fn create(&mut self, &State, &Section) -> Result<(), Error>;
    fn generated(&self) -> Result<String, Error>;
    fn apply(&self) -> Result<(), Error>;
    fn output(&mut self) -> Option<&PathBuf>;
}

pub enum GetResult<T> {
    Ok(T),
    Expected(String),
    NotFound,
}

impl<T> GetResult<T> {
    pub fn to_result(self, name: &str) -> Result<T, ::failure::Error> {
        match self {
            GetResult::Ok(s) => Ok(s),
            GetResult::Expected(s) => {
                Err(::failure::err_msg(format!("expected {} for {}", s, name)))
            }
            GetResult::NotFound => Err(::failure::err_msg(format!("{} needs a value", name))),
        }
    }

    pub fn to_option(self) -> Option<T> {
        match self {
            GetResult::Ok(t) => Some(t),
            _ => None,
        }
    }
}

pub trait Getter {
    fn get(&self, name: &str) -> Option<&Value>;

    fn get_str(&self, state: &State, name: &str) -> GetResult<String> {
        match self.get(name).or(state.defined.get(name)) {
            Some(&Value::Str(ref s)) => GetResult::Ok(s.to_owned()),
            Some(_) => GetResult::Expected("string".to_string()),
            None => GetResult::NotFound,
        }
    }

    fn get_color(&self, state: &State, name: &str) -> GetResult<Color> {
        let mut is_some = false;
        if let Some(s) = self.get(name) {
            is_some = true;
            if let Ok(c) = expect_color(s) {
                return GetResult::Ok(c);
            }
        }
        if let Some(c) = state.colors.get(name) {
            return GetResult::Ok(c.to_owned());
        }
        if is_some {
            GetResult::Expected("color".to_string())
        } else {
            GetResult::NotFound
        }
    }

    fn get_path(&self, state: &State, name: &str) -> GetResult<PathBuf> {
        match self.get(name).or(state.defined.get(name)) {
            Some(&Value::Path(ref s)) => GetResult::Ok(PathBuf::from(s)),
            Some(_) => GetResult::Expected("path".to_string()),
            None => GetResult::NotFound,
        }
    }
}

impl Getter for Section {
    fn get(&self, name: &str) -> Option<&Value> {
        self.values().get(name)
    }
}

fn expect_color(value: &Value) -> Result<Color, Error> {
    match value {
        Value::Hex(h) => Ok(Color(
            ((h & 0xff0000) >> 16) as u8,
            ((h & 0x00ff00) >> 8) as u8,
            (h & 0x0000ff) as u8,
        )),
        Value::RGB(r, g, b) => Ok(Color(*r, *g, *b)),
        v => Err(Error::ExpectedColor(v.clone())),
    }
}

// Template?

pub fn process_section(
    state: &mut State,
    name: &str,
    section: &Section,
) -> Result<Option<Box<Theme>>, Error> {
    match name.to_lowercase().as_str() {
        "x11" | "xresources" => {
            let mut x11 = X11::new();
            x11.create(state, section)?;
            Ok(Some(Box::new(x11)))
        }
        "urxvt" => {
            let mut x11 = X11::program(name.to_owned());
            x11.create(state, section)?;
            Ok(Some(Box::new(x11)))
        }
        "define" => {
            state.defined = section.values().to_owned();
            Ok(None)
        }
        "colors" => {
            let mut to_resolve = VecDeque::new();
            for entry in section.values() {
                if let Value::Section(ref s) = entry.value {
                    for sentry in s.values() {
                        let sname = format!("{}_{}", entry.name, sentry.name);
                        if let Ok(color) = expect_color(&sentry.value) {
                            state.colors.insert(sname, color);
                        } else if let Value::Str(ref s) = sentry.value {
                            to_resolve.push_back((sname.to_string(), s));
                        }
                        // TODO: else for error
                    }
                } else if let Ok(color) = expect_color(&entry.value) {
                    state.colors.insert(entry.name.to_string(), color);
                } else if let Value::Str(ref s) = entry.value {
                    to_resolve.push_back((entry.name.to_string(), s));
                }
            }
            let mut has_resolved = true;
            while has_resolved && !to_resolve.is_empty() {
                has_resolved = false;
                for _ in 0..to_resolve.len() {
                    if let Some((name, value)) = to_resolve.pop_front() {
                        if let GetResult::Ok(color) = section.get_color(state, &value) {
                            state.colors.insert(name.to_string(), color);
                            has_resolved = true;
                        } else {
                            to_resolve.push_back((name, value));
                        }
                    }
                }
            }
            Ok(None)
        }
        _ => Err(Error::UnknownSection(name.to_owned())),
    }
}

pub fn process_config(config: &mut Config) -> Result<Vec<Box<Theme>>, Error> {
    let mut result = Vec::new();
    let mut state = State {
        colors: Map::new(),
        defined: Map::new(),
    };
    if let Some(defined) = config.sections().get("defined") {
        let _ = process_section(&mut state, "defined", &defined);
    }
    if let Some(colors) = config.sections().get("colors") {
        let _ = process_section(&mut state, "colors", &colors);
    }
    for entry in config.sections() {
        if entry.name == "colors" || entry.name == "defined" {
            continue;
        }
        match process_section(&mut state, &entry.name, &entry.value) {
            Ok(Some(gen)) => {
                result.push(gen);
            }
            Ok(None) | Err(Error::UnknownSection { .. }) => continue,
            Err(e) => return Err(e),
        }
    }
    Ok(result)
}
