use std::{collections::HashMap, ops::RangeInclusive};

fn main(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut ranges = line.split(',').map(|range| {
                let mut ranges_anchors = range
                    .split('-')
                    .map(|number| number.parse::<usize>().unwrap());
                (
                    ranges_anchors.next().unwrap(),
                    ranges_anchors.next().unwrap(),
                )
            });

            (ranges.next().unwrap(), ranges.next().unwrap())
        })
        .map(|ranges| {
            (
                RangeInclusive::<usize>::new(ranges.0 .0, ranges.0 .1),
                RangeInclusive::<usize>::new(ranges.1 .0, ranges.1 .1),
            )
        })
        .map(|ranges| -> usize {
            let mut num_map = HashMap::<usize, bool>::new();

            for num in ranges.0 {
                num_map.insert(num, false);
            }

            for num in ranges.1 {
                if num_map.get(&num).is_some() {
                    num_map.insert(num, true);
                }
            }

            if num_map.into_values().any(|value| value == true) {
                1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::main;

    #[test]
    pub fn four() {
        assert_eq!(872, main(std::include_str!("./four.txt")));
    }
}
