use clap::{ Command, Arg, Subcommand};

#[derive(Debug)]
pub struct Options {
    pub version: String,
    pub file: String
}

pub fn parse_args() -> Options {
    let matches = Command::new("coco")
        .version("0.1.0")
        .author("seaung")
        .about("Coco application command-line utility")
        .get_matches();

    Options{
        version: matches.get_one::<String>("version").expect("version is null").clone(),
        file: matches.get_one::<String>("file").expect("file is null").clone(),
    }
}