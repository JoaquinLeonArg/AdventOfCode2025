use advent_of_code2025::Challenge;

pub struct TrashCompactor;

impl TrashCompactor {
    pub fn new() -> TrashCompactor {
        TrashCompactor
    }
}

impl Challenge for TrashCompactor {
    fn solve(&self, input: &str) -> Result<String, String> {
        let mut lines = input.lines();
        let operations: Vec<&str> = lines
            .next_back()
            .expect("Operations not found")
            .split_whitespace()
            .collect();
        let data = lines.map(|line| {
            line.split_whitespace()
                .map(|value| str::parse::<u64>(value).expect("Invalid integer found"))
                .collect::<Vec<u64>>()
        });

        let mut results = vec![0; operations.len()];

        for row in data {
            for (index, value) in row.iter().enumerate() {
                match operations[index] {
                    "+" => results[index] += value,
                    "*" => results[index] = results[index].max(1) * value,
                    _ => return Err("Invalid operation found".to_string()),
                }
            }
        }

        Ok(results.iter().sum::<u64>().to_string())
    }

    fn solve_b(&self, input: &str) -> Result<String, String> {
        let lines: Vec<&str> = input.lines().collect();
        let (raw_operations, raw_data) = lines.split_last().expect("Operations not found");
        let operations = raw_operations.split_whitespace().collect::<Vec<&str>>();

        let mut transposed_data: Vec<String> = vec![String::new(); lines[0].len()];
        for (_, line) in raw_data.iter().enumerate() {
            for (c_index, c) in line.chars().enumerate() {
                transposed_data[c_index] += &c.to_string();
            }
        }

        let mut groups: Vec<Vec<u64>> = vec![];
        let mut current_group: Vec<u64> = vec![];
        for line in transposed_data {
            let number = str::parse::<u64>(line.trim());
            match number {
                Ok(x) => {
                    current_group.push(x);
                }
                Err(_) => {
                    groups.push(current_group);
                    current_group = vec![];
                }
            }
        }
        groups.push(current_group);

        let mut result: u64 = 0;

        for (index, group) in groups.into_iter().enumerate() {
            match operations[index] {
                "+" => result += group.into_iter().sum::<u64>(),
                "*" => result += group.into_iter().product::<u64>(),

                _ => return Err("Invalid operation found".to_string()),
            }
        }

        Ok(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("../input/example6.txt");
        let result = TrashCompactor::new().solve(input);
        assert_eq!(result, Ok("4277556".to_string()));
    }

    #[test]
    fn example_b() {
        let input = include_str!("../input/example6.txt");
        let result = TrashCompactor::new().solve_b(input);
        assert_eq!(result, Ok("3263827".to_string()));
    }
}
