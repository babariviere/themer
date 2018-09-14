use super::{Color, Error, GetResult, Getter, State, Theme};
use config::map::Map;
use config::Section;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

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

const COLOR_MAP: &[(&str, &str)] = &[
    ("black", "color0"),
    ("red", "color1"),
    ("green", "color2"),
    ("yellow", "color3"),
    ("blue", "color4"),
    ("magenta", "color5"),
    ("cyan", "color6"),
    ("white", "color7"),
    ("bright_black", "color8"),
    ("bright_red", "color9"),
    ("bright_green", "color10"),
    ("bright_yellow", "color11"),
    ("bright_blue", "color12"),
    ("bright_magenta", "color13"),
    ("bright_cyan", "color14"),
    ("bright_white", "color15"),
    ("foreground", "foreground"),
    ("background", "background"),
    ("cursor", "cursorColor"),
];

#[derive(Default, Debug)]
pub struct X11 {
    program: Option<String>,
    output: Option<PathBuf>,
    colors: Map<Color>,
}

impl X11 {
    pub fn new() -> Self {
        X11::default()
    }

    pub fn program(program: String) -> Self {
        let mut x11 = X11::new();
        x11.program = Some(program);
        x11
    }
}

impl Theme for X11 {
    fn available_fields(&self) -> &[&str] {
        AVAILABLE_FIELDS
    }

    fn create(&mut self, state: &State, section: &Section) -> Result<(), Error> {
        if let GetResult::Ok(program) = section.get_str(state, "program") {
            self.program = Some(program);
        }
        self.output = section.get_path(state, "output").to_option();
        for (color, newname) in COLOR_MAP {
            if let Some(c) = section
                .get_color(state, color)
                .to_option()
                .or(section.get_color(state, newname).to_option())
            {
                self.colors.insert(newname.to_string(), c);
            }
        }
        Ok(())
    }

    fn generated(&self) -> Result<String, Error> {
        let program = self.program.as_ref().map(|s| &**s).unwrap_or("*");
        let mut buf = Vec::new();
        for entry in &self.colors {
            let name = &entry.name;
            let value = &entry.value;
            buf.push(format!(
                "{}.{}: #{:02x}{:02x}{:02x}",
                program, name, value.0, value.1, value.2
            ));
        }
        Ok(buf.join("\n"))
    }

    fn apply(&self) -> Result<(), Error> {
        let generated = self.generated()?;
        let path;
        let mut f = match self.output {
            Some(ref p) => {
                path = p.display().to_string();
                File::create(p)?
            }
            None => {
                let program = self.program.as_ref().map(|s| &**s).unwrap_or("default");
                let p = ::std::env::temp_dir().join(format!("themer-x11_{}", program));
                path = p.display().to_string();
                File::create(p)?
            }
        };
        f.write_all(generated.as_bytes())?;
        let command = Command::new("xrdb").arg("-merge").arg(&path).output()?;
        println!("{:?}", command);
        Ok(())
    }

    fn output(&mut self) -> Option<&PathBuf> {
        self.output.as_ref()
    }
}
