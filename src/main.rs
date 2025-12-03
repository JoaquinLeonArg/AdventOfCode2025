pub mod day01;
pub mod day02;
pub mod day03;

use advent_of_code2025::Challenge;

use crate::day01::SecretEntrance;
use crate::day02::GiftShop;
use crate::day03::Lobby;
use std::env::args;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = args().collect();
    let process_name = args.get(0).unwrap();
    let day = args.get(1).expect(&usage_string(process_name));
    let filename = args.get(2).expect(&usage_string(process_name));

    let handler: Box<dyn Challenge> = match day.as_str() {
        "1" => Box::new(SecretEntrance::new()),
        "2" => Box::new(GiftShop::new()),
        "3" => Box::new(Lobby::new()),
        _ => return Err(format!("Invalid day: {}", day).into()),
    };

    let input = fs::read_to_string(format!("./input/{}", filename))?;

    let result_a = handler.solve(&input)?;
    let result_b = handler.solve_b(&input)?;

    println!("Solution (A): {}", result_a);
    println!("Solution (B): {}", result_b);

    Ok(())
}

fn usage_string(process_name: &str) -> String {
    format!("Usage: {} <day> <filename>", process_name)
}
