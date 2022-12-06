use std::collections::BinaryHeap;
use std::fs::read_to_string;
use std::str::FromStr;

fn part_one(contents: &str) -> u32 {
    contents
        .lines()
        .map(u32::from_str)
        .fold((0u32, 0u32), |(max, acc), x| {
            if let Ok(y) = x {
                (max, acc + y)
            }
            // NOTE: Added an extra new line at the end of input to simply breaking condition
            else {
                (max.max(acc), 0)
            }
        })
        .0
}

fn part_two(contents: &str) -> u32 {
    let mut numbers = BinaryHeap::new();

    contents.lines().map(u32::from_str).fold(0u32, |acc, x| {
        if let Ok(y) = x {
            acc + y
        }
        // NOTE: Added an extra new line at the end of input to simply breaking condition
        else {
            numbers.push(acc);
            0
        }
    });

    (0..3).fold(0, |acc, _| acc + numbers.pop().unwrap())
}

fn main() {
    let contents =
        read_to_string("examples/01/main.in").expect("Something went wrong reading the file");

    println!(
        "How many total Calories is that Elf carrying?\n{}",
        part_one(&contents)
    );

    println!(
        "How many Calories are those Elves carrying in total?\n{}",
        part_two(&contents)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_contents() -> String {
        read_to_string("examples/01/sample.in").expect("Something went wrong reading the file")
    }

    #[test]
    fn test_part_one() {
        assert_eq!(24000, part_one(&get_contents()));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(45000, part_two(&get_contents()));
    }
}
