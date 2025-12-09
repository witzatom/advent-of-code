advent_of_code::solution!(7);

struct Manifold {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl From<&str> for Manifold {
    fn from(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = grid.len();
        let width = grid[0].len();
        Manifold {
            grid,
            width,
            height,
        }
    }
}

impl Manifold {
    fn count_splits(&self) -> usize {
        // find the starting beams
        let mut beams = vec![false; self.width];
        // we assume the start is on the first line
        self.grid
            .first()
            .unwrap()
            .iter()
            .enumerate()
            .for_each(|(x, &c)| {
                if c == 'S' {
                    beams[x] = true;
                }
            });
        let mut split_count = 0;

        for y in 0..self.height {
            let mut new_beams = vec![false; self.width];
            for x in 0..self.width {
                if beams[x] {
                    match self.grid[y][x] {
                        '^' => {
                            // split the beam
                            split_count += 1;
                            if x > 0 {
                                new_beams[x - 1] = true;
                            }
                            if x + 1 < self.width {
                                new_beams[x + 1] = true;
                            }
                        }
                        '.' | 'S' => {
                            // continue the beam downward
                            new_beams[x] = true;
                        }
                        _ => {}
                    }
                }
            }
            beams = new_beams;
        }
        split_count
    }

    fn count_timelines(&self) -> u64 {
        let mut beam_timeline_counts = vec![0; self.width];
        self.grid
            .first()
            .unwrap()
            .iter()
            .enumerate()
            .for_each(|(x, &c)| {
                if c == 'S' {
                    beam_timeline_counts[x] = 1;
                }
            });

        for y in 0..self.height {
            let mut new_timeline_counts = vec![0; self.width];
            for x in 0..self.width {
                let count = beam_timeline_counts[x];
                if count > 0 {
                    match self.grid[y][x] {
                        '^' => {
                            // split the timelines
                            if x > 0 {
                                new_timeline_counts[x - 1] += count;
                            }
                            if x + 1 < self.width {
                                new_timeline_counts[x + 1] += count;
                            }
                        }
                        '.' | 'S' => {
                            // continue the timelines downward
                            new_timeline_counts[x] += count;
                        }
                        _ => {}
                    }
                }
            }
            beam_timeline_counts = new_timeline_counts;
        }
        beam_timeline_counts.iter().sum()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let manifold = Manifold::from(input);
    Some(manifold.count_splits() as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let manifold = Manifold::from(input);
    Some(manifold.count_timelines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
