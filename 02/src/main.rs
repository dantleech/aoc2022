use std::{collections::HashMap, fs};

fn main() {
    let input: String = fs::read_to_string("data/input").expect("nope");
    let pairs: Vec<(&str, &str)> = input.trim().split("\n").into_iter().map(|line| {
        let pair: Vec<&str> = line.split(" ").collect();

        (pair[0], pair[1])
    }).collect();
    println!("{}", calculate(pairs));
}

#[derive(Clone)]
enum Weapon {
    Paper,
    Scissors,
    Rock
}

impl Weapon {
    fn value(&self) -> i32 {
        match self {
            Weapon::Paper => 2,
            Weapon::Scissors => 3,
            Weapon::Rock => 1,
        }
    }
}
impl Weapon {
    fn fights(&self, s2: &Weapon) -> Result {
        match self {
            Weapon::Paper => match s2 {
                Weapon::Paper => Result::Draw,
                Weapon::Scissors => Result::Lost,
                Weapon::Rock => Result::Won
            },
            Weapon::Scissors => match s2 {
                Weapon::Paper => Result::Won,
                Weapon::Scissors => Result::Draw,
                Weapon::Rock => Result::Lost,
            },
            Weapon::Rock => match s2 {
                Weapon::Paper => Result::Lost,
                Weapon::Scissors => Result::Won,
                Weapon::Rock => Result::Draw,
            },
        }
    }
}

fn weapons_map() -> HashMap<&'static str, Weapon> {
    vec![
        ("A", Weapon::Rock),
        ("B", Weapon::Paper),
        ("C", Weapon::Scissors),

        ("X", Weapon::Rock),
        ("Y", Weapon::Paper),
        ("Z", Weapon::Scissors),
    ].iter().cloned().collect()
}

enum Result {
    Won,
    Draw,
    Lost
}

impl Result {
    fn score(&self) -> i32 {
        match self {
            Result::Won => 6,
            Result::Draw => 3,
            Result::Lost => 0,
        }
    }
}

pub fn calculate(pairs: Vec<(&str, &str)>) -> i32 {
    let weapons = weapons_map();

    let rounds = pairs.iter().map(|(p1, p2)| -> i32 {
        let w1 = weapons.get(p1).unwrap();
        let w2 = weapons.get(p2).unwrap();
        let round = w2.fights(w1);

        round.score() + w2.value()
    });

    return rounds.sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculates_score_based_on_guide() {
        assert_eq!(15, calculate(vec![
                ("A", "Y"),
                ("B", "X"),
                ("C", "Z"),
        ]))
    }
}
