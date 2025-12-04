use std::vec;

use advent_of_code2025::Challenge;

pub struct PrintingDepartment;

impl PrintingDepartment {
    pub fn new() -> PrintingDepartment {
        PrintingDepartment
    }
}

impl Challenge for PrintingDepartment {
    fn solve(&self, input: &str) -> Result<String, String> {
        let data: Vec<Vec<char>> = input
            .lines()
            .into_iter()
            .map(|row| row.chars().into_iter().collect())
            .collect();

        let offsets: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut result = 0;
        for (y, row) in data.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let neighbours = offsets.iter().fold(0, |acc, offset| {
                    let checked_y = y as i32 + offset.1;
                    let checked_x = x as i32 + offset.0;
                    if checked_x < 0 || checked_y < 0 {
                        return acc;
                    }
                    acc + match data
                        .get(checked_y as usize)
                        .unwrap_or(&vec![])
                        .get(checked_x as usize)
                        .unwrap_or(&'.')
                    {
                        '@' => 1,
                        _ => 0,
                    }
                });
                if neighbours < 4 && cell == &'@' {
                    result += 1;
                }
            }
        }

        Ok(result.to_string())
    }

    fn solve_b(&self, input: &str) -> Result<String, String> {
        let mut data: Vec<Vec<char>> = input
            .lines()
            .into_iter()
            .map(|row| row.chars().into_iter().collect())
            .collect();

        let offsets: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut result = 0;
        let mut to_remove = vec![];
        while result == 0 || !to_remove.is_empty() {
            to_remove = vec![];
            for (y, row) in data.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    let neighbours = offsets.iter().fold(0, |acc, offset| {
                        if let Some(checked_y) = y.checked_add_signed(offset.1)
                            && let Some(checked_x) = x.checked_add_signed(offset.0)
                        {
                            return acc
                                + match data
                                    .get(checked_y)
                                    .unwrap_or(&vec![])
                                    .get(checked_x)
                                    .unwrap_or(&'.')
                                {
                                    '@' => 1,
                                    _ => 0,
                                };
                        }
                        return acc;
                    });
                    if neighbours < 4 && cell == &'@' {
                        result += 1;
                        to_remove.push((x, y));
                    }
                }
            }
            for (x, y) in to_remove.clone() {
                data[y][x] = '.';
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
        let input = include_str!("../input/example4.txt");
        let result = PrintingDepartment::new().solve(input);
        assert_eq!(result, Ok("13".to_string()));
    }

    #[test]
    fn example_b() {
        let input = include_str!("../input/example4.txt");
        let result = PrintingDepartment::new().solve_b(input);
        assert_eq!(result, Ok("43".to_string()));
    }
}
