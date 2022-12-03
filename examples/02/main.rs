use itertools::Itertools;
use std::fs::read_to_string;

fn part_one(contents: &str) -> u32 {
    contents
        .lines()
        .flat_map(|s| s.split_whitespace().tuples::<(&str, &str)>())
        .map(|c| {
            /*
             * A for Rock, B for Paper, and C for Scissors
             * X for Rock, Y for Paper, and Z for Scissors
             * 1 for Rock, 2 for Paper, and 3 for Scissors
             * 0 if you lost, 3 if the round was a draw, and 6 if you won
             */
            match c {
                ("A", "X") => 1 + 3,
                ("A", "Y") => 2 + 6,
                ("A", "Z") => 3, // + 0

                ("B", "X") => 1, // + 0
                ("B", "Y") => 2 + 3,
                ("B", "Z") => 3 + 6,

                ("C", "X") => 1 + 6,
                ("C", "Y") => 2, // + 0
                ("C", "Z") => 3 + 3,
                _ => 0,
            }
        })
        .sum()
}

fn part_two(contents: &str) -> u32 {
    contents
        .lines()
        .flat_map(|s| s.split_whitespace().tuples::<(&str, &str)>())
        .map(|c| {
            // X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win
            match c {
                ("A", "X") => 3, // + 0
                ("A", "Y") => 1 + 3,
                ("A", "Z") => 2 + 6,

                ("B", "X") => 1, // + 0
                ("B", "Y") => 2 + 3,
                ("B", "Z") => 3 + 6,

                ("C", "X") => 2, // + 0
                ("C", "Y") => 3 + 3,
                ("C", "Z") => 1 + 6,
                _ => 0,
            }
        })
        .sum()
}

fn main() {
    let contents =
        read_to_string("examples/02/main.in").expect("Something went wrong reading the file");

    println!("What would your total score be if everything goes exactly according to your strategy guide??\n- {}", part_one(&contents));

    println!("What would your total score be if everything goes exactly according to your strategy guide??\n- {}", part_two(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_contents() -> String {
        read_to_string("examples/02/sample.in").expect("Something went wrong reading the file")
    }

    #[test]
    fn test_part_one() {
        assert_eq!(15, part_one(&get_contents()));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(12, part_two(&get_contents()));
    }
}
