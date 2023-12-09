use day_4::*;

fn main() {
    let data = include_str!("../../input.txt").trim();
    let result1 = solve_part1(data);
    println!("Solution part 1: {}", result1);

    let result2 = solve_part2(data);
    println!("Solution part 2: {}", result2);
}
