use itertools::Itertools;
use std::{collections::HashSet, fs::read_to_string};

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 96, //('a' as u32 - 1)
        'A'..='Z' => c as u32 - 38,
        _ => 0,
    }
}

fn part_one(contents: &str) -> u32 {
    contents
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .flat_map(|(l, r)| {
            l.chars()
                .collect::<HashSet<char>>()
                .intersection(&r.chars().collect())
                .cloned()
                .collect::<Vec<char>>()
        })
        .map(priority)
        .sum()
}

fn part_two(contents: &str) -> u32 {
    contents
        .lines()
        .map(|l| l.chars().collect::<HashSet<char>>())
        .chunks(3)
        .into_iter()
        .flat_map(|chunks| {
            chunks
                .reduce(|a, b| a.intersection(&b).cloned().collect())
                .expect("There will always be a common char")
        })
        .map(priority)
        .sum()
}

fn main() {
    let contents =
        read_to_string("examples/03/main.in").expect("Something went wrong reading the file");

    println!(
        "What is the sum of the priorities of those item types?\n- {}",
        part_one(&contents)
    );

    println!("?\n- {}", part_two(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_contents() -> String {
        read_to_string("examples/03/sample.in").expect("Something went wrong reading the file")
    }

    #[test]
    fn test_part_one() {
        assert_eq!(157, part_one(&get_contents()));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(70, part_two(&get_contents()));
    }
}
