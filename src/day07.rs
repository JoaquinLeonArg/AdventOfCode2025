use std::collections::{HashMap, HashSet};

use advent_of_code2025::Challenge;

pub struct Laboratories;

impl Laboratories {
    pub fn new() -> Laboratories {
        Laboratories
    }
}

impl Challenge for Laboratories {
    fn solve(&self, input: &str) -> Result<String, String> {
        let lines = input
            .lines()
            .into_iter()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();
        let mut current_rays = HashSet::from([lines[0]
            .iter()
            .position(|c| *c == 'S')
            .expect("Cannot find starting point")]);

        let mut result = 0;

        for row in 1..lines.len() {
            let mut new_rays = HashSet::new();
            for ray in current_rays.iter() {
                if lines[row][*ray] == '^' {
                    result += 1;
                    new_rays.insert(*ray - 1);
                    new_rays.insert(*ray + 1);
                } else {
                    new_rays.insert(*ray);
                }
            }
            current_rays = new_rays;
        }

        Ok(result.to_string())
    }

    fn solve_b(&self, input: &str) -> Result<String, String> {
        let lines = input
            .lines()
            .into_iter()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();
        let mut current_rays = HashMap::from([(
            lines[0]
                .iter()
                .position(|c| *c == 'S')
                .expect("Cannot find starting point"),
            1,
        )]);

        for row in 1..lines.len() {
            let mut new_rays = HashMap::new();
            for (ray, count) in current_rays.iter() {
                if lines[row][*ray] == '^' {
                    if let Some(existing_count) = new_rays.get(&(ray - 1)) {
                        new_rays.insert(*ray - 1, *existing_count + count);
                    } else {
                        new_rays.insert(*ray - 1, *count);
                    }
                    if let Some(existing_count) = new_rays.get(&(ray + 1)) {
                        new_rays.insert(*ray + 1, *existing_count + count);
                    } else {
                        new_rays.insert(*ray + 1, *count);
                    }
                } else {
                    if let Some(existing_count) = new_rays.get(ray) {
                        new_rays.insert(*ray, *existing_count + count);
                    } else {
                        new_rays.insert(*ray, *count);
                    }
                }
            }
            current_rays = new_rays;
        }

        Ok(current_rays.values().sum::<u64>().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("../input/example7.txt");
        let result = Laboratories::new().solve(input);
        assert_eq!(result, Ok("21".to_string()));
    }

    #[test]
    fn example_b() {
        let input = include_str!("../input/example7.txt");
        let result = Laboratories::new().solve_b(input);
        assert_eq!(result, Ok("40".to_string()));
    }
}
