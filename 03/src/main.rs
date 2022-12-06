use std::{collections::HashMap, fs, slice::ChunksExact};

fn main() {
    let input: String = fs::read_to_string("data/input").expect("nope");
    println!("{}", calculate_part_1(input));
    let input: String = fs::read_to_string("data/input").expect("nope");
    println!("{}", calculate_part_2(input));
}

type Backpack = (Vec<char>, Vec<char>);
type Backpacks = Vec<Backpack>;

pub fn calculate_part_1(input: String) -> i32 {
    let backpacks = create_backpacks(input);
    let common_items = common_items(backpacks);
    let priority_map = priority_map();

    let cips: Vec<i32> = common_items
        .iter()
        .map(|i: &char| -> i32 { *priority_map.get(&i).unwrap() })
        .collect();
    cips.iter().sum()
}

pub fn calculate_part_2(input: String) -> i32 {
    let backpacks = create_backpacks(input);
    let priority_map = priority_map();
    let groups = backpacks.chunks_exact(3);
    let common_items = common_items_in_groups(groups);
    println!("{:?}", common_items);

    let cips: Vec<i32> = common_items
        .iter()
        .map(|i: &char| -> i32 { *priority_map.get(&i).unwrap() })
        .collect();
    cips.iter().sum()
}

pub fn create_backpacks(input: String) -> Backpacks {
    input
        .trim()
        .split("\n")
        .into_iter()
        .map(|line| -> Backpack {
            let line = line.trim();
            let c1 = &line[0..line.len() / 2];
            let c2 = &line[line.len() / 2..line.len()];
            (c1.chars().collect(), c2.chars().collect())
        })
        .collect()
}

fn priority_map() -> HashMap<char, i32> {
    let letters = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let mut map: HashMap<char, i32> = HashMap::new();
    let mut i = 1;
    for char in letters {
        map.insert(char, i);
        i += 1
    }
    map
}

fn common_items(backpacks: Backpacks) -> Vec<char> {
    backpacks
        .into_iter()
        .map(|b| -> char {
            let c1 = &b.0;
            let c2 = &b.1;

            for i1 in c1 {
                for i2 in c2 {
                    if i1 == i2 {
                        return *i1;
                    }
                }
            }
            panic!("Could not determine common item");
        })
        .collect()
}

fn common_items_in_groups(groups: ChunksExact<(Vec<char>, Vec<char>)>) -> Vec<char> {
    groups
        .map(|group| -> char {
            let mut map: HashMap<char, i32> = HashMap::new();
            for mut backpack in group.to_vec() {
                // merge compartments
                backpack.0.append(&mut backpack.1);
                backpack.0.sort();
                backpack.0.dedup();

                for i in backpack.0 {
                    let entry = map.entry(i).or_insert(0);
                    *entry += 1;
                    if *entry == 3 {
                        return i;
                    }
                }
            }
            panic!("Could not find common item in group");
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(
            157,
            calculate_part_1(
                "
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw"
                    .to_string()
            )
        )
    }

    #[test]
    fn part_2() {
        assert_eq!(
            70,
            calculate_part_2(
                "
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw"
                    .to_string()
            )
        )
    }

    #[test]
    fn generate_priority_map() {
        assert_eq!(1, *priority_map().get(&'a').unwrap());
        assert_eq!(26, *priority_map().get(&'z').unwrap());
        assert_eq!(27, *priority_map().get(&'A').unwrap());
    }
}
