advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input.lines().filter_map(|f|{
            let f_num = f.chars().find(char::is_ascii_digit);
            let last_num = f.chars().rfind(char::is_ascii_digit);
            Some(f_num?.to_digit(10)? *10 + last_num?.to_digit(10)?)
        })
        .sum(),

    )
}

const NUMBER_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_first_number(s: &str) -> Option<u32> {
    s.chars()
        .enumerate()
        .find_map(|(idx, _)| find_number_at_position(s, idx, false))
}

fn find_last_number(s: &str) -> Option<u32> {
    s.char_indices()
        .rev()
        .find_map(|(idx, _)| find_number_at_position(s, idx, true))
}

fn find_number_at_position(s: &str, idx: usize, reverse: bool) -> Option<u32> {
    if let Some(c) = s.chars().nth(idx) {
        if c.is_ascii_digit() {
            return c.to_digit(10);
        }
    }

    for (i, word) in NUMBER_WORDS.iter().enumerate() {
        let pattern = if reverse {
            if idx + 1 < word.len() { continue; }
            &s[idx + 1 - word.len()..=idx]
        } else {
            if idx + word.len() > s.len() { continue; }
            &s[idx..idx + word.len()]
        };

        if pattern == *word {
            return Some(i as u32 + 1);
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|line| {
                let first = find_first_number(line)?;
                let last = find_last_number(line)?;
                Some(first * 10 + last)
            })
            .sum()
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        let result = part_one(input);
        assert_eq!(result, Some(142));
    }

     #[test]
     fn test_part_two() {
        let input = &advent_of_code::template::read_file_part("inputs", DAY, 2);
         let result = part_two(input);
         println!("{}", result.unwrap());
         assert_eq!(result, Some(53515));
     }
 }
