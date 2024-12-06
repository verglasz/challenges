use std::convert::Infallible;
use std::fs::File;
use std::io::Write;
use std::process::Command;

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

    create_new_crate(args.name);
}

fn create_new_crate(name: AocName) -> () {
    use AocName::*;
    match name {
        Day(day) => {
            let name = create_day_crate(day);
            init_day_crate(&name);
        }
        Aux(aux) => create_aux_crate(aux),
    }
}

fn create_day_crate(day: u8) -> String {
    println!("Creating day crate day{:02}", day);
    let crate_name = format!("day{:02}", day);
    Command::new("cargo")
        .arg("ws")
        .arg("create")
        .arg(&crate_name)
        .arg("--bin")
        .arg("--edition")
        .arg("2021")
        .arg("--name")
        .arg(&crate_name)
        .spawn()
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
        .expect("Failed to add `utils` to deps");
    let mut main =
        File::create(format!("{}/src/main.rs", crate_name)).expect("Failed to create/open main.rs");
    main.write_all(
        r###"
use utils::get_stdinput;

fn main() {
    let input = get_stdinput();
    let parsed = parse(input);
    let p1 = solve1(&parsed);
    println!("sol1: {p1:?}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2:?}");
}
type Input = ();

fn parse(lines: impl Iterator<Item = impl AsRef<str>>) -> Input {
    lines;
}

fn solve1(input: &Input) -> () {}

fn solve2(input: &Input) -> () {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), ());
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), ());
    }
}


    "###
        .as_bytes(),
    )
    .expect("Failed to write to main.rs");
}

fn create_aux_crate(aux: String) -> () {
    todo!()
}
