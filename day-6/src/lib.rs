use std::collections::HashMap;
use aoc_util::string_to_lines;
use itertools::Itertools;

struct Race {
    time: i64,
    distance: i64
}

impl Race {
    fn calc_one(&self, n: i64) -> i64 {
        n * (self.time-n)
    }

    fn calc_all(&self) -> HashMap<i64, i64> {
        // For n 0 to time, calculate distance. Return vec of distances indexed by time
        (0..self.time)
            .map(|i| (i, self.calc_one(i)))
            .collect::<HashMap<_, _>>()
    }

    fn beats_distance(&self) -> HashMap<i64, i64> {
        self.calc_all().iter()
            .filter(|&(&k, &v)| v > self.distance)
            .map(|(k,v )| (*k, *v))
            .collect()
    }
}

pub fn solve_part1(input: &str) -> i64 {
    let lines = string_to_lines(input);

    let (_, times_string) = lines[0]
        .split_once(":")
        .expect("needs to have a :");

    let times: Vec<_> = dbg!(times_string)
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i64>().expect("needs to be a number"))
        .collect();

    println!("times: {:?}", times);

    let (_, distances_string) = lines[1]
        .split_once(":")
        .expect("needs to have a :");

    let distances: Vec<_> = dbg!(distances_string)
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i64>().expect("needs to be a number"))
        .collect();

    println!("distances: {:?}", distances);

    let mut races = vec![];

    for i in 0..times.len() {
        races.push(Race {
            time: times[i],
            distance: distances[i]
        })
    }

    for r in &races {
        println!("all: {:?}", r.calc_all());
        println!("winning: {:?}", r.beats_distance())
    }

    let margin_of_error = &races.iter()
        .map(|r| r.beats_distance())
        .map(|d| d.len())
        .reduce(|acc, e| acc*e)
        .expect("can calculate");

    println!("Margin of error: {}", margin_of_error);

    *margin_of_error as i64
}


pub fn solve_part2(input: &str) -> i64 {
    let lines = string_to_lines(input);

    let time = lines[0].strip_prefix("Time:").expect("needs prefix")
        .trim()
        .split_whitespace()
        .fold(String::new(), |acc, e| acc+e)
        .parse::<i64>()
        .expect("parsable");

    let distance = lines[1].strip_prefix("Distance:").expect("needs prefix")
        .trim()
        .split_whitespace()
        .fold(String::new(), |acc, e| acc+e)
        .parse::<i64>()
        .expect("parsable");

    let r = Race {
        time,
        distance
    };

    let ways_to_beat = r.beats_distance().len();
    println!("{:?}", ways_to_beat);

    ways_to_beat as i64
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_real_data() {
        let data = include_str!("../input.txt").trim();
        assert_eq!(500346, solve_part1(data));
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn part1_test_data() {
        let test_data = "Time:      7  15   30
Distance:  9  40  200
";
        assert_eq!(288, solve_part1(test_data));
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn part2_test_data() {
        let test_data = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(71503, solve_part2(test_data));
    }

    #[test]
    fn part2_real_data() {
        let data = include_str!("../input.txt").trim();
        assert_eq!(42515755, solve_part2(data));
    }
}
