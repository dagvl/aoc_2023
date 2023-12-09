#![allow(unused_variables)]

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Card {
    id: i32,
    numbers: HashSet<i32>,
    winners: HashSet<i32>,
    line: String,
}

impl Card {
    pub fn parse(line: &str) -> Self {

        let (head, rest) = line.trim().split_once(":").expect("needs a colon");
        // dbg!(head);

        let n_string = head.strip_prefix("Card")
            .expect("needs a Game n header")
            .trim();
        //dbg!(n_string);

        let n = n_string
            .parse::<i32>()
            .expect("needs to be a number");

        let (winner_list, number_list) = rest
            .split_once("|")
            .expect("needs a |");

        let numbers: HashSet<_> = number_list
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("must be a number"))
            .collect();


        let winners: HashSet<_> = winner_list
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("must be a number"))
            .collect();

        let r = Card {
            id: n,
            numbers,
            winners,
            line: String::from(line)
        };
        // dbg!(&r);
        r
    }

    fn score(&self) -> i32 {
        let matches = self.winners.intersection(&self.numbers).count() as u32;
        if matches == 0 {
            0
        } else {
            2_i32.pow(matches - 1)
        }

    }

    fn matches(&self) -> i32 {
        self.winners.intersection(&self.numbers).count() as i32
    }

}

pub fn solve_part1(input: &str) -> i32 {
   let lines = aoc_util::string_to_lines(input);
    let games: Vec<_> = lines.iter()
        .map(|l| Card::parse(l))
        .collect();

    // Lines are "Game n: n n n n | n n n n" where N is a potentially multidigit number
    // games.iter()
    //     .for_each(|g| println!("{} -> {}", g.line, g.score()));


    let s = games.iter()
        .map(|g| g.score())
        .sum();

    s
}

pub fn solve_part2(input: &str) -> i32 {
    let lines = aoc_util::string_to_lines(input);
    let mut cards_vec: Vec<_> = lines.iter()
        .map(|l| Card::parse(l))
        .collect();

    let mut cards = HashMap::new();
    cards_vec
        .iter()
        .for_each(|c| _ = cards.insert(c.id, c.clone()));


    let mut i = 0;
    while i < cards_vec.len() {
        let c = &cards_vec[i];

        let matches = c.matches();
        // if we have matches, throw copies of the card on the vec
        // println!("Card {} has {} matches", c.id, matches);
        if matches > 0 {

            for i in c.id+1..c.id+1+matches {
                let extra_card = cards.get(&i).unwrap().clone();
                // println!("Adding card {}", i);
                cards_vec.push(extra_card);
            }

        }

        if i%1000==0 {
            println!("--- after processing card {} the list is in total {}. (to be processed: {})", i, cards_vec.len(), cards_vec.len()-i);
        }
        i += 1;
        // println!("len: {}", cards_vec.len());
    }

    cards_vec.len() as i32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_real_data() {
        let data = include_str!("../input.txt").trim();
        assert_eq!(22488, solve_part1(data));
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn part1_test_data() {
        let test_data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        assert_eq!(13, solve_part1(test_data));
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn part2_test_data() {
        let test_data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(30, solve_part2(test_data));
    }
//
//     #[test]
//     fn part2_real_data() {
//         let data = include_str!("../input.txt").trim();
//         assert_eq!(76008, solve_part2(data));
//     }
}
