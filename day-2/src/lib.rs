use std::collections::HashMap;
use aoc_util::string_to_lines;

#[derive(Debug)]
struct Game {
    id: i32,
    dice: HashMap<String, i32>
}

impl Game {
    fn power(&self) -> i32 {
        let r = *self.dice.get("red").unwrap_or(&0);
        let g = *self.dice.get("green").unwrap_or(&0);
        let b = *self.dice.get("blue").unwrap_or(&0);
        r*g*b
    }
}

impl Game {
    fn new(id: i32) -> Self {
        Self {
            id,
            dice: HashMap::new(),
        }
    }
}
pub fn solve_part1(input: &str) -> i32 {
    let lines = string_to_lines(input);
    let games: Vec<_> = lines.iter()
        .map(|l| parse_line(l))
        .collect();
    
    let valid_games: Vec<_> = games.iter()
        .filter(|g| is_valid(g))
        .collect();

    valid_games
        .iter()
        .map(|g| g.id)
        .sum()
}

fn is_valid(game: &Game) -> bool {
    // it is valid if there is only draws of red, green and blue and a max of 12, 13 and 14 respectively
    let r = *game.dice.get("red").unwrap_or(&0);
    let g = *game.dice.get("green").unwrap_or(&0);
    let b = *game.dice.get("blue").unwrap_or(&0);

    r <= 12 && g <= 13 && b <= 14
}

pub fn solve_part2(input: &str) -> i32 {
    let lines = string_to_lines(input);
    let games: Vec<_> = lines.iter()
        .map(|l| parse_line(l))
        .collect();

    let power = games.iter()
        .map(|g| g.power())
        .sum();
    power
}

fn parse_line(draw: &str) -> Game {
    let (game_string, draws_string) = draw.split_once(':').expect("there must be a colon");
    let (_, number_string) = game_string.split_once(' ').expect("game_string should have a space");
    let game_no = number_string.parse::<i32>().expect("second part of number_string should be a number");

    let d = parse_draws(draws_string);

    let mut g = Game::new(game_no);
    g.dice = d;

    g
}

fn parse_draws(draws_string: &str) -> HashMap<String, i32> {
    let mut hm = HashMap::new();

    draws_string
        .split(';')
        .map(|s| s.trim())
        .flat_map(|s| s.split(',').collect::<Vec<_>>())
        .map(|s| s.trim())
        .filter_map(|s| s.split_once(' '))
        .map(|(n, c)| (n.parse::<i32>().expect("must be a number"), c))
        .for_each(|(n, c)| {

            let old_value = hm.get(c);
            match old_value {
                None => _ = hm.insert(c.into(), n),
                Some(o) => {
                    if n > *o {
                        hm.insert(c.into(), n);
                    }
                }
            };
        });

    hm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_real_data() {
        let data = include_str!("../input.txt").trim();
        assert_eq!(2348, solve_part1(data));
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn part1_test_data() {
        let test_data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        assert_eq!(8, solve_part1(test_data));
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn part2_test_data() {
        let test_data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(2286, solve_part2(test_data));
    }

    #[test]
    fn part2_real_data() {
        let data = include_str!("../input.txt").trim();
        assert_eq!(76008, solve_part2(data));
    }
}
