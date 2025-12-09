advent_of_code::solution!(5);

#[derive(Debug)]
struct Database {
    fresh_ranges: Vec<(u64, u64)>,
    available_ids: Vec<u64>,
}

impl From<&str> for Database {
    fn from(value: &str) -> Self {
        let mut sections = value.split("\n\n");
        let fresh_ranges_section = sections.next().unwrap();
        let available_ids_section = sections.next().unwrap();

        let fresh_ranges = fresh_ranges_section
            .lines()
            .filter_map(|line| {
                let mut parts = line.split('-');
                let start = parts.next()?.parse::<u64>().ok()?;
                let end = parts.next()?.parse::<u64>().ok()?;
                Some((start, end))
            })
            .collect::<Vec<(u64, u64)>>();

        let available_ids = available_ids_section
            .lines()
            .filter_map(|line| line.parse::<u64>().ok())
            .collect::<Vec<u64>>();

        Database {
            fresh_ranges,
            available_ids,
        }
    }
}

impl Database {
    fn is_fresh(&self, id: u64) -> bool {
        for (start, end) in &self.fresh_ranges {
            if id >= *start && id <= *end {
                return true;
            }
        }
        false
    }

    fn fresh_ids_count(&self) -> usize {
        let mut fresh_ranges = self.fresh_ranges.clone();
        fresh_ranges.sort_by_key(|(start, _)| *start);
        let mut fresh_count = 0;
        let mut offset = None;
        for (start, end) in fresh_ranges {
            let start_with_offset = match offset {
                Some(o) if start <= o => o,
                _ => start,
            };
            if end >= start_with_offset {
                fresh_count += (end - start_with_offset + 1) as usize;
                offset = Some(end + 1);
            }
        }
        fresh_count
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let database = Database::from(input);
    let fresh_ids = database
        .available_ids
        .iter()
        .filter(|idx| database.is_fresh(**idx))
        .count();
    Some(fresh_ids)
}

pub fn part_two(input: &str) -> Option<usize> {
    let database = Database::from(input);
    Some(database.fresh_ids_count())
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
        assert_eq!(result, Some(14));
    }
}
