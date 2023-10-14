use core::fmt;
use std::{collections::BTreeMap, ops::Range};

#[derive(Debug)]
pub struct Container(char);

pub struct Containers(BTreeMap<usize, Vec<Container>>);

impl Containers {
    pub fn r#move(&mut self, count: usize, from: usize, to: usize) {
        let range = Range {
            start: 0,
            end: count,
        };

        let mut moved = Vec::<Container>::new();
        let from = self.0.get_mut(&from).unwrap();
        for _ in range {
            moved.push(from.pop().unwrap());
        }
        moved.reverse();

        let to = self.0.get_mut(&to).unwrap();
        to.append(&mut moved);
    }

    pub fn push(&mut self, to: usize, container: Container) {
        let mut vector = self.0.remove(&to).unwrap_or_default();
        vector.reverse();
        vector.push(container);
        vector.reverse();

        self.0.insert(to, vector);
    }
}

impl fmt::Debug for Containers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "")?;
        for stack in self.0.values() {
            writeln!(
                f,
                "{:?}",
                stack
                    .iter()
                    .map(|container| format!("[{}] ", container.0))
                    .collect::<String>()
            )?;
        }

        fmt::Result::Ok(())
    }
}

fn main(input: &str) -> String {
    let mut lines = input.lines().into_iter();
    let mut containers = Containers(BTreeMap::new());

    println!("parse crate configuration");
    for line in &mut lines {
        println!("Parsing line {line}");
        if line.is_empty() {
            break;
        }

        for (index, chunk) in String::from(line)
            .chars()
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|chunk| (chunk[0].clone() as char, chunk[1].clone() as char))
            .enumerate()
        {
            let index = index + 1;
            println!("Found chunk: {:?} at index {index}", chunk);
            if chunk.0 == '[' {
                containers.push(index, Container(chunk.1));
                println!("Inserting container {} at index {index}", chunk.1)
            }
        }
    }

    dbg!(&containers);

    println!("parse & execute instructions");
    for mut line in lines.map(|line| line.split_whitespace()) {
        println!("{}", line.clone().collect::<String>());
        line.next().unwrap();
        let move_count = line.next().unwrap().parse::<usize>().unwrap();
        line.next().unwrap();
        let from = line.next().unwrap().parse::<usize>().unwrap();
        line.next().unwrap();
        let to = line.next().unwrap().parse::<usize>().unwrap();

        containers.r#move(move_count, from, to);
        dbg!(&containers);
    }

    containers
        .0
        .into_iter()
        .map(|(_, containers)| {
            containers
                .last()
                .map(|container| container.0)
                .unwrap_or(' ')
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::main;

    #[test]
    pub fn five() {
        assert_eq!("RBTWJWMCF", main(std::include_str!("./five.txt")));
    }
}
