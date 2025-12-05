use advent_of_code2025::Challenge;

pub struct Cafeteria;

impl Cafeteria {
    pub fn new() -> Cafeteria {
        Cafeteria
    }
}

impl Challenge for Cafeteria {
    fn solve(&self, input: &str) -> Result<String, String> {
        let breakpoint = input
            .lines()
            .position(|line| line == "")
            .expect("Can't find separator between ranges and available products");
        let lines: Vec<&str> = input.lines().collect();
        let fresh_ranges: Vec<(u64, u64)> = lines[0..breakpoint]
            .iter()
            .map(|range| {
                let (start, end) = range.split_once("-").expect("Range cannot be split");
                (
                    str::parse::<u64>(start).expect("Invalid range start"),
                    str::parse::<u64>(end).expect("Invalid range end"),
                )
            })
            .collect();
        let available_products = lines[(breakpoint + 1)..]
            .iter()
            .map(|product| str::parse::<u64>(product).expect("Invalid product id"));

        let mut fresh_ingredients = 0;
        for product in available_products {
            for (start, end) in &fresh_ranges {
                if product >= *start && product <= *end {
                    fresh_ingredients += 1;
                    break;
                }
            }
        }

        Ok(fresh_ingredients.to_string())
    }

    fn solve_b(&self, input: &str) -> Result<String, String> {
        let breakpoint = input
            .lines()
            .position(|line| line == "")
            .expect("Can't find separator between ranges and available products");
        let lines: Vec<&str> = input.lines().collect();
        let mut fresh_ranges: Vec<(i64, i64)> = lines[0..breakpoint]
            .iter()
            .map(|range| {
                let (start, end) = range.split_once("-").expect("Range cannot be split");
                (
                    str::parse::<i64>(start).expect("Invalid range start"),
                    str::parse::<i64>(end).expect("Invalid range end"),
                )
            })
            .collect();
        fresh_ranges.sort_by(|a, b| a.0.cmp(&b.0));

        let mut fresh_ids: i64 = 0;
        let mut max = 0;
        for (start, end) in fresh_ranges {
            if end <= max {
                continue;
            }
            fresh_ids += end - start.max(max + 1) + 1;
            if end > max {
                max = end;
            }
        }

        Ok(fresh_ids.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("../input/example5.txt");
        let result = Cafeteria::new().solve(input);
        assert_eq!(result, Ok("3".to_string()));
    }

    #[test]
    fn example_b() {
        let input = include_str!("../input/example5.txt");
        let result = Cafeteria::new().solve_b(input);
        assert_eq!(result, Ok("14".to_string()));
    }
}
