extern crate structopt;
extern crate themer;
extern crate themer_config as config;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Command {
    /// Use specified theme
    #[structopt(name = "use")]
    Use,
    /// Generate a new theme
    #[structopt(name = "generate")]
    Generate,
    /// Apply theme (needed for Xresources or else
    #[structopt(name = "apply")]
    Apply,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "themer")]
struct Themer {
    #[structopt(subcommand)]
    command: Command,
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    config: Option<PathBuf>,
}

fn main() {
    let themer = Themer::from_args();
    println!("{:#?}", themer);
    let home = ::std::env::var("HOME").unwrap();
    let config = themer
        .config
        .unwrap_or(PathBuf::from(format!("{}/.config/themer/default.th", home)));
    let mut config = config::read_config(config).unwrap().unwrap();
    let mut themes = match themer::process_config(&mut config) {
        Ok(themes) => themes,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    for theme in themes {
        println!("{}", theme.generated().unwrap());
    }
}
