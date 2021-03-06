use std::{
    fs::File,
    io::{Read, Result},
};

use clap::{load_yaml, App};

fn main() -> Result<()> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let day = i32::from_str_radix(matches.value_of("day").unwrap(), 10).unwrap();

    let mut file = File::open(matches.value_of("INPUT").unwrap())?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    match day {
        1 => {
            let input = buf
                .split("\n")
                .filter_map(|s| match s.trim() {
                    s if s.len() > 0 => Some(s),
                    _ => None,
                })
                .map(|s| i32::from_str_radix(s, 10).unwrap())
                .collect::<Vec<_>>();

            let result = if matches.is_present("second") {
                day1::execute_second(&input)
            } else {
                day1::execute_first(&input)
            };

            println!("Result: {}", result);
        }
        2 => {
            let input = buf
                .split("\n")
                .filter_map(|s| match s.trim() {
                    s if s.len() > 0 => Some(s),
                    _ => None,
                })
                .collect::<Vec<_>>();
            let moves = day2::parse(&input);

            let result = if matches.is_present("second") {
                day2::execute_second(moves)
            } else {
                day2::execute_first(moves)
            };

            println!("Result: {}", result);
        }
        3 => {
            let input = buf
                .split("\n")
                .filter_map(|s| match s.trim() {
                    s if s.len() > 0 => Some(s),
                    _ => None,
                })
                .collect::<Vec<_>>();

            let result = if matches.is_present("second") {
                day3::execute_second::<12>(&input)
            } else {
                day3::execute_first::<12>(&input)
            };

            println!("Result: {}", result);
        }
        _ => panic!("Unsupported day {}", day),
    };

    Ok(())
}

pub mod day1;
pub mod day2;
pub mod day3;
