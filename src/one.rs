use std::collections::BTreeSet;

fn main(input: &str) -> Option<usize> {
    let mut current_buffer: usize = 0;
    let mut calories: BTreeSet<usize> = BTreeSet::new();

    for line in input.lines() {
        if line.is_empty() {
            calories.insert(current_buffer);
            current_buffer = 0;
        } else {
            current_buffer += line.parse::<usize>().unwrap();
        }
    }

    Some(calories.pop_last()? + calories.pop_last()? + calories.pop_last()?)
}

#[cfg(test)]
mod test {
    use super::main;

    #[test]
    pub fn one() {
        assert_eq!(Some(209603), main(std::include_str!("./one.txt")));
    }
}
