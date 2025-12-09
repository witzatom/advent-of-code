use advent_of_code::helpers::Point2;
use std::collections::HashSet;
use std::ops::Add;

advent_of_code::solution!(4);

#[derive(Debug)]
struct Grid {
    rolls: HashSet<Point2<i32>>,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let mut rolls: HashSet<Point2<i32>> = HashSet::new();
        let lines: Vec<&str> = value.lines().collect();
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '@' {
                    rolls.insert(Point2 {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }
        Self { rolls }
    }
}

impl Grid {
    fn accessible_rolls(&self) -> HashSet<Point2<i32>> {
        let mut accessible: HashSet<Point2<i32>> = HashSet::new();
        let directions: Vec<Point2<i32>> = vec![
            Point2 { x: -1, y: -1 },
            Point2 { x: 0, y: -1 },
            Point2 { x: 1, y: -1 },
            Point2 { x: -1, y: 0 },
            Point2 { x: 1, y: 0 },
            Point2 { x: -1, y: 1 },
            Point2 { x: 0, y: 1 },
            Point2 { x: 1, y: 1 },
        ];
        for roll in &self.rolls {
            let adjacent_count = directions
                .iter()
                .filter_map(|dir| {
                    let position = roll.add(*dir);
                    if self.rolls.contains(&position) {
                        Some(position)
                    } else {
                        None
                    }
                })
                .count();
            if adjacent_count < 4 {
                accessible.insert(*roll);
            }
        }
        accessible
    }

    fn remove_rolls(&mut self, rolls_to_remove: &HashSet<Point2<i32>>) {
        for roll in rolls_to_remove {
            self.rolls.remove(roll);
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid::from(input);
    Some(grid.accessible_rolls().len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = Grid::from(input);
    let mut total_removed = 0;
    loop {
        let accessible = grid.accessible_rolls();
        if accessible.is_empty() {
            break;
        }
        total_removed += accessible.len();
        grid.remove_rolls(&accessible);
    }
    Some(total_removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
