use super::{Color, Error, Theme};
use config::Section;
use std::path::PathBuf;

const AVAILABLE_FIELDS: &[&str] = &[
    "program",
    "output",
    "black",
    "red",
    "green",
    "yellow",
    "blue",
    "magenta",
    "cyan",
    "white",
    "bright_black",
    "bright_red",
    "bright_green",
    "bright_yellow",
    "bright_blue",
    "bright_magenta",
    "bright_cyan",
    "bright_white",
    "foreground",
    "background",
    "cursor",
];

#[derive(Default, Debug)]
pub struct X11 {
    program: Option<String>,
    output: Option<PathBuf>,
    black: Option<Color>,
    red: Option<Color>,
    green: Option<Color>,
    yellow: Option<Color>,
    blue: Option<Color>,
    magenta: Option<Color>,
    cyan: Option<Color>,
    white: Option<Color>,
    bright_black: Option<Color>,
    bright_red: Option<Color>,
    bright_green: Option<Color>,
    bright_yellow: Option<Color>,
    bright_blue: Option<Color>,
    bright_magenta: Option<Color>,
    bright_cyan: Option<Color>,
    bright_white: Option<Color>,
    foreground: Option<Color>,
    background: Option<Color>,
    cursor: Option<Color>,
}

impl X11 {
    pub fn new() -> Self {
        X11::default()
    }

    pub fn program(name: String) -> Self {
        let mut x11 = X11::default();
        x11.program = Some(name);
        x11
    }
}

impl Theme for X11 {
    fn available_fields(&self) -> &[&str] {
        AVAILABLE_FIELDS
    }

    fn create(&mut self, _section: &Section) -> Result<(), Error> {
        Ok(())
    }

    fn generated(&self) -> Result<String, Error> {
        let _program = self.program.as_ref().map(|s| &**s).unwrap_or("*");
        Ok(String::new())
    }

    fn apply(&self) -> Result<(), Error> {
        Ok(())
    }

    fn output(&self) -> Option<&PathBuf> {
        self.output.as_ref()
    }
}
