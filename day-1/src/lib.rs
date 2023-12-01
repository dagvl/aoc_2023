use aoc_util::string_to_lines;

pub fn solve_part1(input: &str) -> i64 {
    let lines = string_to_lines(input);
    let digits: Vec<_> = lines
        .iter()
        .map(|l| {
            l.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>()
        }).collect();

    let numbers: Vec<_> = digits
        .iter()
        .map(|ds| format!("{}{}", ds.first().unwrap(), ds.last().unwrap()))
        .map(|num| num.parse::<i64>().unwrap())
        .collect();


    numbers.iter().sum()
}

pub fn solve_part2(input: &str) -> i64 {
    let lines = string_to_lines(input);


    let digits: Vec<_> = lines
         .iter()
         .map(|s| find_all_digits(s))
        .collect();

    digits
        .iter()
        .map(|dgs| 10*dgs.first().unwrap() + dgs.last().unwrap())
        .sum()


}

fn find_all_digits(input: &str) -> Vec<i64> {
    let n = input.chars().count();

    let mut digits = vec![];
    for i in 0..n {
        match find_digit(&input[i..]) {
            Some(d) => {digits.push(d)}
            None => {}
        }
    }

    digits
}

pub fn find_digit(input: &str) -> Option<i64> {
    let first_char = input.chars().nth(0).unwrap();

    // if it is a number it's easy
    if first_char.is_numeric() {
        let d = first_char.to_digit(10).unwrap();
        return Some(d as i64);
    }

    // check for all written "digits"
    match input {
        _ if input.starts_with("zero") => Some(0),
        _ if input.starts_with("one") => Some(1),
        _ if input.starts_with("two") => Some(2),
        _ if input.starts_with("three") => Some(3),
        _ if input.starts_with("four") => Some(4),
        _ if input.starts_with("five") => Some(5),
        _ if input.starts_with("six") => Some(6),
        _ if input.starts_with("seven") => Some(7),
        _ if input.starts_with("eight") => Some(8),
        _ if input.starts_with("nine") => Some(9),

        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_real_data() {
        let data = include_str!("input.txt");
        assert_eq!(54561, solve_part1(data));
    }

    #[test]
    fn part1_test_data() {
        let test_data = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        assert_eq!(142, solve_part1(test_data));
    }

    #[test]
    fn part2_test_data() {
        let test_data = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(281, solve_part2(test_data));
    }

    #[test]
    fn part2_real_data() {
        let data = include_str!("input.txt");
        assert_eq!(54076, solve_part2(data));
    }
}
