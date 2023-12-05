#[doc = " The current day."]
const DAY:advent_of_code::Day = advent_of_code::day!(1);
fn main(){
  use advent_of_code::template::runner::*;
  let input = advent_of_code::template::read_file("inputs",DAY);
  run_part(part_one, &input,DAY,1);
  run_part(part_two, &input,DAY,2);
}

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

// pub fn part_two(input: &str) -> Option<u32> {
//     None
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        let result = part_one(input);
        assert_eq!(result, Some(142));
    }

//     #[test]
//     fn test_part_two() {
//         let result = part_two(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, None);
//     }
 }
