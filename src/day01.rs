use advent_of_code2025::Challenge;

pub struct SecretEntrance;

impl SecretEntrance {
    pub fn new() -> SecretEntrance {
        SecretEntrance
    }
}

impl Challenge for SecretEntrance {
    fn solve(&self, input: &str) -> Result<String, String> {
        let mut zero_rotations: i64 = 0;
        let mut current: i64 = 50;

        for r in input.lines() {
            let (direction, raw_amount) = r.split_at(1);
            let amount = str::parse::<i64>(raw_amount).expect("Non integer rotation found");
            match direction {
                "L" => {
                    current = (current - amount).rem_euclid(100);
                    Ok(())
                }
                "R" => {
                    current = (current + amount).rem_euclid(100);
                    Ok(())
                }
                _ => Err("Invalid rotation type found"),
            }?;
            if current == 0 {
                zero_rotations += 1;
            }
        }

        Ok(zero_rotations.to_string())
    }

    fn solve_b(&self, input: &str) -> Result<String, String> {
        let mut zero_rotations: i64 = 0;
        let mut current: i64 = 50;

        for r in input.lines() {
            let (direction, raw_amount) = r.split_at(1);
            let amount = str::parse::<i64>(raw_amount).expect("Non integer rotation found");
            let mut new_current: i64 = 0;
            match direction {
                "L" => {
                    new_current = current - amount;
                    while new_current < 0 {
                        new_current += 100;
                        zero_rotations += 1;
                    }
                    Ok(())
                }
                "R" => {
                    new_current = current + amount;
                    while new_current > 99 {
                        new_current -= 100;
                        zero_rotations += 1;
                    }
                    Ok(())
                }
                _ => Err("Invalid rotation type found"),
            }?;

            current = new_current;
        }

        Ok(zero_rotations.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("../input/example1.txt");
        let result = SecretEntrance::new().solve(input);
        assert_eq!(result, Ok("3".to_string()));
    }

    #[test]
    fn example_b() {
        let input = include_str!("../input/example1.txt");
        let result = SecretEntrance::new().solve_b(input);
        assert_eq!(result, Ok("6".to_string()));
    }
}
