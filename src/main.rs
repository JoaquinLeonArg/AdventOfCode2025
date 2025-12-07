pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

use advent_of_code2025::Challenge;

use crate::day01::SecretEntrance;
use crate::day02::GiftShop;
use crate::day03::Lobby;
use crate::day04::PrintingDepartment;
use crate::day05::Cafeteria;
use crate::day06::TrashCompactor;
use std::fs;
use std::time::SystemTime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for day in 1..=6 {
        let handler: Box<dyn Challenge> = match day {
            1 => Box::new(SecretEntrance::new()),
            2 => Box::new(GiftShop::new()),
            3 => Box::new(Lobby::new()),
            4 => Box::new(PrintingDepartment::new()),
            5 => Box::new(Cafeteria::new()),
            6 => Box::new(TrashCompactor::new()),
            _ => return Err(format!("Invalid day: {}", day).into()),
        };
        let input = fs::read_to_string(format!("./input/day{}.txt", day))?;

        let start_a = SystemTime::now();
        let result_a = handler.solve(&input)?;
        let time_a = SystemTime::now().duration_since(start_a).unwrap();

        let start_b = SystemTime::now();
        let result_b = handler.solve_b(&input)?;
        let time_b = SystemTime::now().duration_since(start_b).unwrap();

        println!("--- Day {} ---", day);
        println!("Solution (A): {} ({}ms)", result_a, time_a.as_millis());
        println!("Solution (B): {} ({}ms)", result_b, time_b.as_millis());
        println!("");
    }

    Ok(())
}
