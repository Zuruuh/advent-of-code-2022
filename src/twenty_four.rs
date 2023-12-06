use std::{collections::BTreeMap, fmt};

use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => '^',
                Direction::Down => 'v',
                Direction::Left => '<',
                Direction::Right => '>',
            }
        )
    }
}

impl Direction {
    pub fn apply_to_position(&self, position: &Position) -> Position {
        match self {
            Direction::Up => Position {
                x: position.x - 1,
                y: position.y,
            },
            Direction::Down => Position {
                x: position.x + 1,
                y: position.y,
            },
            Direction::Left => Position {
                x: position.x,
                y: position.y - 1,
            },
            Direction::Right => Position {
                x: position.x,
                y: position.y + 1,
            },
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
enum Cell {
    Blizzards(Vec<Direction>),
    Wall,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Blizzards(directions) => directions
                    .len()
                    .gt(&1)
                    .then_some(directions.len() as u8 as char)
                    .unwrap_or(
                        directions
                            .first()
                            .unwrap()
                            .to_string()
                            .chars()
                            .next()
                            .unwrap()
                    ),
                Self::Wall => '#',
            }
        )
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Position {
    pub x: usize,
    pub y: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.x, self.y)
    }
}

pub fn twenty_four(input: &str) -> usize {
    let mut current_grid = BTreeMap::from_iter(
        input
            .lines()
            .enumerate()
            .flat_map(|(x, line)| {
                line.chars()
                    .map(|char| {
                        Some(match char {
                            '#' => Cell::Wall,
                            '>' => Cell::Blizzards(vec![Direction::Right]),
                            '<' => Cell::Blizzards(vec![Direction::Left]),
                            'v' => Cell::Blizzards(vec![Direction::Down]),
                            '^' => Cell::Blizzards(vec![Direction::Up]),
                            _ => return None,
                        })
                    })
                    .enumerate()
                    .map(|(y, char)| (Position { x, y }, char))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );

    let boundaries = current_grid.last_key_value().unwrap().0.clone();

    let mut next_grid = BTreeMap::<Position, Option<Cell>>::new();
    let mut player_position = Position { x: 0, y: 1 };
    let mut move_count = 0;

    loop {
        move_count += 1;

        current_grid.iter().for_each(|(position, cell)| {
            render(&next_grid, &boundaries);
            println!("========");
            match cell {
                Some(cell) => match cell {
                    Cell::Wall => {
                        next_grid.insert(*position, Some(Cell::Wall));
                    }
                    Cell::Blizzards(directions) => {
                        for direction in directions {
                            let mut new_position = direction.apply_to_position(position);
                            if new_position.x == 0 {
                                new_position.x = boundaries.x - 1;
                            }

                            if new_position.x == boundaries.x {
                                new_position.x = 1;
                            }

                            if new_position.y == 0 {
                                new_position.y = boundaries.y - 1;
                            }

                            if new_position.y == boundaries.y {
                                new_position.y = 1;
                            }

                            println!("Moving blizzard in {position} to {new_position}");
                            next_grid.insert(*position, None);

                            let existing = next_grid.get(&new_position).map(|cell| match cell {
                                Some(cell) => match cell {
                                    Cell::Blizzards(directions) => {
                                        let mut directions = directions.clone();
                                        directions.push(*direction);

                                        Cell::Blizzards(directions)
                                    }
                                    _ => panic!("wtf"),
                                },
                                None => Cell::Blizzards(vec![*direction]),
                            });

                            next_grid.insert(new_position, existing);
                        }
                    }
                },
                None => {
                    (!next_grid.contains_key(position)).then(|| next_grid.insert(*position, None));
                }
            };
        });

        break;
    }

    move_count
}

fn render(grid: &BTreeMap<Position, Option<Cell>>, boundary: &Position) {
    let mut grid_copy = BTreeMap::<Position, char>::new();

    for x in 0..=boundary.x {
        for y in 0..=boundary.y {
            grid_copy.insert(Position { x, y }, ' ');
        }
    }

    grid.iter().for_each(|(pos, cell)| {
        if let Some(cell) = cell {
            grid_copy.insert(*pos, cell.to_string().clone().chars().next().unwrap());
        } else {
            grid_copy.insert(*pos, '.');
        }
    });

    let mut buffer = String::default();
    let mut old_x = 0;

    for (pos, char) in grid_copy.into_iter().sorted() {
        if pos.x != old_x {
            println!("{buffer}");
            buffer.clear();
            old_x = pos.x;
        }

        buffer.push(char);
    }

    println!("{buffer}");
}

#[cfg(test)]
mod test {
    use super::twenty_four;

    #[test]
    pub fn test() {
        let input = include_str!("./twenty_four.txt");
        let result = twenty_four(input);
        assert_eq!(18, result);
    }
}
