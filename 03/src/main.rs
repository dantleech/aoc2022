use std::{collections::HashMap, fs};

fn main() {
    let input: String = fs::read_to_string("data/input").expect("nope");
    calculate(input);
}

pub fn calculate(input: String) -> i32 {

    let backpacks: Vec<(Vec<char>,Vec<char>)> = input.trim().split("\n").into_iter().map(|line| -> (Vec<char>,Vec<char>) {
        let compartment1 = &line[0..line.len() / 2];
        let compartment2 = &line[line.len() / 2..line.len()];
        (compartment1.trim().chars().collect(),compartment2.trim().chars().collect())

    }).collect();


    12
}

fn priority_map() -> HashMap<char, i32> {
    let letters = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 
        'o', 'p', 'q', 'r', 's', 't', 'u', 'z', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 
        'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'Z', 'W', 'X', 'Y', 'Z',
    ];
    let mut map: HashMap<char,i32> = HashMap::new();
    let mut i = 1;
    for char in letters {
        map.insert(char, i);
        i+=1
    }
    map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculates_score_based_on_guide() {
        assert_eq!(15, calculate("
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw".to_string()
        ))
    }

    #[test]
    fn generate_priority_map() {
        assert_eq!(1, *priority_map().get(&'a').unwrap());
        assert_eq!(26, *priority_map().get(&'z').unwrap());
        assert_eq!(27, *priority_map().get(&'A').unwrap());
    }
}
