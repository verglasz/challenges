use std::fs::File;
use std::io::{BufReader, Write};
use std::process::Command;
use std::{convert::Infallible, io::Read};

use clap::{Args, Parser};

#[derive(Parser, Debug)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum CargoAoc {
    #[command(name = "aoc")]
    Aoc(AocArgs),
}

#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
struct AocArgs {
    /// aoc day number or utils crate name
    #[arg(value_parser=AocName::parse)]
    name: AocName,
    #[arg(long = "template", short = 't')]
    template: Option<String>,
}

#[derive(Debug, Clone)]
enum AocName {
    Day(u8),
    Aux(String),
}

impl AocName {
    fn parse(s: &str) -> Result<Self, Infallible> {
        use AocName::*;
        s.parse::<u8>().map(Day).or_else(|_| Ok(Aux(s.to_string())))
    }
}

fn main() {
    let CargoAoc::Aoc(args) = CargoAoc::parse();
    let mut res = vec![];
    let template = if let Some(tfile) = args.template {
        let f = File::open(tfile).expect("template file should exist");
        BufReader::new(f)
            .read_to_end(&mut res)
            .expect("failed to read template file");
        &res
    } else {
        DEFAULT_MAIN
    };

    create_new_crate(args.name, template);
}

fn create_new_crate(name: AocName, template: &[u8]) -> () {
    use AocName::*;
    match name {
        Day(day) => {
            let name = create_day_crate(day, template);
            init_day_crate(&name);
        }
        Aux(aux) => create_aux_crate(aux),
    }
}

fn create_day_crate(day: u8, template: &[u8]) -> String {
    println!("Creating day crate day{:02}", day);
    let crate_name = format!("day{:02}", day);
    Command::new("cargo")
        .arg("ws")
        .arg("create")
        .arg(&crate_name)
        .arg("--bin")
        .arg("--edition")
        .arg("2024")
        .arg("--name")
        .arg(&crate_name)
        .spawn()
        .expect("Failed to spawn cargo ws")
        .wait()
        .expect("Failed to create day crate");
    crate_name
}

fn init_day_crate(crate_name: &str) {
    println!("Initializing day crate {}", crate_name);
    Command::new("cargo")
        .arg("add")
        .arg("utils")
        .current_dir(crate_name)
        .spawn()
        .expect("Failed to spawn cargo add")
        .wait()
        .expect("Failed to add `utils` to deps");
    let mut main =
        File::create(format!("{}/src/main.rs", crate_name)).expect("Failed to create/open main.rs");
    let main_text = DEFAULT_MAIN;
    main.write_all(main_text)
        .expect("Failed to write to main.rs");
}

fn create_aux_crate(aux: String) -> () {
    todo!()
}

const DEFAULT_MAIN: &[u8] = r###"
use utils::get_stdinput;

fn main() {
    let input: Vec<_> = get_stdinput().collect();
    let parsed = parse(input.iter().map(|x| x.as_str()));
    let p1 = solve1(&parsed);
    println!("sol1: {p1}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2}");
}
type Input = ();

fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Input {
    lines;
}

fn solve1(input: &Input) -> usize {0}

fn solve2(input: &Input) -> usize {0}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 0);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 0);
    }
}


    "###
.as_bytes();
