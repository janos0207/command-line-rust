use clap::{arg, Command};
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_flag: bool,
    number_nonblank_flag: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Janos <janos0207terminal@gmail.com>")
        .about("Rust cat")
        .args(&[
            arg!([FILE]),
            arg!(-n --number "number all output lines"),
            arg!(-b --"number-nonblank" "number nonempty output lines, overrides -n"),
        ])
        .get_matches();

    let files = match matches.get_many::<String>("FILE") {
        Some(value) => value.map(|v| v.to_string()).collect::<Vec<_>>(),
        None => vec![],
    };

    let number_flag = matches.get_flag("number");
    let number_nonblank_flag = matches.get_flag("number-nonblank");

    Ok(Config {
        files: files,
        number_flag: number_flag,
        number_nonblank_flag: number_nonblank_flag,
    })
}
