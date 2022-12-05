use std::collections::{HashMap, HashSet};

fn priorities_map() -> HashMap<char, u32> {
    let letters = ('a'..='z').chain('A'..='Z').collect::<Vec<char>>();
    let letters_with_priorities = letters
        .into_iter()
        .zip(1..=52 as u32)
        .collect::<Vec<(char, u32)>>();
    let tuples_arr: [(char, u32); 52] = letters_with_priorities.try_into().unwrap();
    HashMap::from(tuples_arr)
}

pub fn process_part1(input: &str) -> String {
    let map = priorities_map();
    input
        .lines()
        .map(|line| {
            let length = line.len();
            let mut chars: Vec<char> = line.chars().collect();
            let first_set: HashSet<char> = HashSet::from_iter(chars.split_off(length / 2));
            let second_set: HashSet<char> = HashSet::from_iter(chars);
            let matching = first_set
                .intersection(&second_set)
                .into_iter()
                .next()
                .unwrap();
            map.get(matching).unwrap()
        })
        .sum::<u32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    "testing".to_string()
}
