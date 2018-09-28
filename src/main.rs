extern crate structopt;
extern crate themer;
extern crate themer_config as config;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Generate {
    #[structopt(name = "file", parse(from_os_str))]
    file: PathBuf,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Use specified theme
    #[structopt(name = "use")]
    Use,
    /// Generate a new theme
    #[structopt(name = "generate")]
    Generate(Generate),
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
    let home = ::std::env::var("HOME").unwrap();
    let config = themer
        .config
        .unwrap_or(PathBuf::from(format!("{}/.config/themer/default.th", home)));
    let config = config::read_config(config).unwrap().unwrap();
    //let themes = match themer::process_config(&mut config) {
    //    Ok(themes) => themes,
    //    Err(e) => {
    //        println!("{}", e);
    //        return;
    //    }
    //};
    match themer.command {
        //Command::Apply => {
        //    for theme in themes {
        //        theme.apply().unwrap();
        //    }
        //}
        Command::Generate(gen) => {
            use std::io::Read;
            let mut file = std::fs::File::open(gen.file).unwrap();
            let mut buf = String::new();
            file.read_to_string(&mut buf).unwrap();
            let template = themer::template::Parser::new(&buf).parse().unwrap();
            let state = themer::process_state(&config);
            let result = themer::template::process_parts(template.parts, &state.colors);
            println!("{}", result.unwrap());
        }
        _c => unimplemented!(),
    }
}
