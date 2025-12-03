use advent_of_code2025::Challenge;

pub struct Lobby;

impl Lobby {
    pub fn new() -> Lobby {
        Lobby
    }
}

impl Challenge for Lobby {
    fn solve(&self, input: &str) -> Result<String, String> {
        let banks = input
            .lines()
            .map(|bank| {
                bank.chars()
                    .map(|battery| {
                        str::parse::<i64>(&battery.to_string()).expect("Non integer battery found")
                    })
                    .collect()
            })
            .collect::<Vec<Vec<i64>>>();

        let mut sum: i64 = 0;
        for bank in banks {
            let (_, without_last) = bank.split_last().expect("Empty battery found");
            let first = without_last
                .iter()
                .max()
                .expect("Couldn't find first battery");
            let first_index = without_last.iter().position(|x| x == first).unwrap();
            let second = bank[(first_index + 1)..]
                .iter()
                .max()
                .expect("Couldn't find second battery");

            sum += first * 10 + second;
        }

        Ok(sum.to_string())
    }

    fn solve_b(&self, input: &str) -> Result<String, String> {
        let banks = input
            .lines()
            .map(|bank| {
                bank.chars()
                    .map(|battery| {
                        str::parse::<i64>(&battery.to_string()).expect("Non integer battery found")
                    })
                    .collect()
            })
            .collect::<Vec<Vec<i64>>>();

        let mut sum: i64 = 0;
        for mut bank in banks {
            for battery_number in 0..12 {
                let (first, _) = bank
                    .split_at_checked(bank.len() - (12 - battery_number - 1))
                    .expect("Not enough batteries");
                let current = first.iter().max().expect("Couldn't find current battery");
                let current_index = first.iter().position(|x| x == current).unwrap();

                sum += current
                    * 10i64.pow(
                        (12 - battery_number - 1)
                            .try_into()
                            .expect("Invalid battery index"),
                    );

                bank = bank[(current_index + 1)..].to_vec();
            }
        }

        Ok(sum.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("../input/example3.txt");
        let result = Lobby::new().solve(input);
        assert_eq!(result, Ok("357".to_string()));
    }

    #[test]
    fn example_b() {
        let input = include_str!("../input/example3.txt");
        let result = Lobby::new().solve_b(input);
        assert_eq!(result, Ok("3121910778619".to_string()));
    }
}
