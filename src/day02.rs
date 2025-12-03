use advent_of_code2025::Challenge;

pub struct GiftShop;

impl GiftShop {
    pub fn new() -> GiftShop {
        GiftShop
    }
}

impl Challenge for GiftShop {
    fn solve(&self, input: &str) -> Result<String, String> {
        let ranges: Vec<(u64, u64)> = input
            .split(",")
            .into_iter()
            .map(|s| s.split_once("-").expect("Invalid range"))
            .map(|(start, end)| {
                (
                    start.parse::<u64>().expect("Invalid integer"),
                    end.parse::<u64>().expect("Invalid integer"),
                )
            })
            .collect();

        let mut invalid_ids_sum: u64 = 0;
        for (start, end) in ranges {
            for id in start..=end {
                let as_str = id.to_string();
                let length = as_str.chars().count();
                if length % 2 != 0 {
                    continue;
                }
                if as_str[..(length / 2)] == as_str[(length / 2)..] {
                    invalid_ids_sum += id
                }
            }
        }

        Ok(invalid_ids_sum.to_string())
    }

    fn solve_b(&self, input: &str) -> Result<String, String> {
        let ranges: Vec<(u64, u64)> = input
            .split(",")
            .into_iter()
            .map(|s| s.split_once("-").expect("Invalid range"))
            .map(|(start, end)| {
                (
                    start.parse::<u64>().expect("Invalid integer"),
                    end.parse::<u64>().expect("Invalid integer"),
                )
            })
            .collect();

        let mut invalid_ids_sum: u64 = 0;
        for (start, end) in ranges {
            for id in start..=end {
                let as_str = id.to_string();
                let length = as_str.chars().count();
                for part_size in 1..=(length / 2) {
                    let sections = GiftShop::generate_sections_for_id(&as_str, part_size);
                    if !sections.is_empty()
                        && sections.iter().all(|section| section.eq(&sections[0]))
                    {
                        invalid_ids_sum += id;
                        break;
                    }
                }
            }
        }

        Ok(invalid_ids_sum.to_string())
    }
}

impl GiftShop {
    pub(crate) fn generate_sections_for_id(id: &String, size: usize) -> Vec<String> {
        let length = id.chars().count();
        if length % size != 0 {
            return vec![];
        }
        let mut sections: Vec<String> = vec![];
        for section_index in 0..(length / size) {
            sections.push(
                id.chars().collect::<Vec<char>>()[section_index * size..(section_index + 1) * size]
                    .into_iter()
                    .collect(),
            )
        }
        sections
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("../input/example2.txt");
        let result = GiftShop::new().solve(input);
        assert_eq!(result, Ok("1227775554".to_string()));
    }

    #[test]
    fn generate_sections_for_id_even() {
        let input = "123456789".to_string();
        let size = 3;
        let result = GiftShop::generate_sections_for_id(&input, size);
        assert_eq!(result, vec!["123", "456", "789"])
    }

    #[test]
    fn example_b() {
        let input = include_str!("../input/example2.txt");
        let result = GiftShop::new().solve_b(input);
        assert_eq!(result, Ok("4174379265".to_string()));
    }
}
