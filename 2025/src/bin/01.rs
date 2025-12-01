advent_of_code::solution!(1);

fn load_data(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            let direction: char = line.chars().next().unwrap();
            let distance: u32 = line[1..].parse().unwrap();
            if direction == 'L' {
                -(distance as i32)
            } else {
                distance as i32
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let hits = load_data(input)
        .iter()
        .fold((50i32, 0u32), |(mut position, mut zero_count), rotation| {
            position = (position + rotation).rem_euclid(100);
            if position == 0 {
                zero_count += 1;
            }
            (position, zero_count)
        })
        .1;
    Some(hits)
}

pub fn part_two(input: &str) -> Option<u32> {
    let hits = load_data(input)
        .iter()
        .fold((50i32, 0u32), |(mut position, mut zero_count), rotation| {
            let new_position = position + rotation;
            let zero_passes = (new_position / 100).abs()
                + (if new_position <= 0 && position != 0 {
                    1
                } else {
                    0
                });
            position = new_position.rem_euclid(100);
            zero_count += zero_passes as u32;
            (position, zero_count)
        })
        .1;
    Some(hits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
