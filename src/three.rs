use std::collections::HashMap;

use itertools::Itertools;

pub fn main(input: &str) -> Option<usize> {
    let mut total = 0;
    let chars_index_map = {
        let mut chars_map = HashMap::<char, usize>::new();
        vec![
            ('a'..='z').collect::<Vec<char>>(),
            ('A'..='Z').collect::<Vec<char>>(),
        ]
        .concat()
        .iter()
        .enumerate()
        .for_each(|(index, char)| {
            chars_map.insert(char.clone(), index + 1);
        });

        chars_map
    };
    let mut chars_map = HashMap::<char, usize>::new();

    for (one, two, three) in input
        .lines()
        .map(|line| String::from(line))
        .collect::<Vec<String>>()
        .chunks(3)
        .map(|chunk| (chunk[0].clone(), chunk[1].clone(), chunk[2].clone()))
    {
        chars_map.clear();

        for char in one.chars().unique() {
            chars_map.insert(char, 1);
        }

        for char in two.chars().unique() {
            chars_map.insert(char, chars_map.get(&char).unwrap_or(&0) + 1);
        }

        for char in three.chars().unique() {
            if chars_map
                .get(&char)
                .map(|count| count == &2)
                .unwrap_or_default()
            {
                println!("Found duplicated char {char}!");
                total += chars_index_map.get(&char).unwrap();

                break;
            }
        }
    }

    Some(total)
}

#[cfg(test)]
mod test {
    use super::main;

    #[test]
    pub fn three() {
        assert_eq!(Some(70), main(std::include_str!("./three.txt")));
    }
}
