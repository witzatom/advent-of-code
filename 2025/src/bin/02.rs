use std::collections::HashSet;

advent_of_code::solution!(2);

fn sum_double_ids_in_range(a: u128, b: u128) -> u128 {
    if a > b {
        return 0;
    }
    let mut total: u128 = 0;
    let digits_b = ((b as f64).log10().floor() as usize) + 1;
    let max_k = digits_b / 2;
    let mut pow10_k: u128 = 1;
    for _k in 1..=max_k {
        pow10_k *= 10; // 10^k
        let coeff = pow10_k + 1; // value = x * coeff
                                 // xmin = ceil(a / coeff)
        let x_min = a.div_ceil(coeff);
        let x_max = b / coeff;
        let lower_bound = pow10_k / 10; // 10^(k-1)
        let upper_bound = pow10_k - 1; // 10^k - 1
        let lo = x_min.max(lower_bound);
        let hi = x_max.min(upper_bound);
        if lo <= hi {
            let count = hi - lo + 1;
            // sum_x = (lo + hi) * count / 2
            let sum_x = (lo + hi) * count / 2;
            total += sum_x * coeff;
        }
    }
    total
}

fn sum_repeated_invalid_ids(a: u128, b: u128) -> u128 {
    let mut invalid_ids: HashSet<u128> = HashSet::new();
    let digits_b = ((b as f64).log10().floor() as usize) + 1;

    for pattern_digits in 1..digits_b {
        let lower_bound = 10u128.pow(pattern_digits as u32 - 1);
        let upper_bound = 10u128.pow(pattern_digits as u32) - 1;
        let max_repeats = digits_b / pattern_digits;
        for repeats in 2..=max_repeats {
            let power_10_kr = 10u128.pow((pattern_digits * repeats) as u32);
            let s = (power_10_kr - 1) / (upper_bound);
            let x_min = a.div_ceil(s);
            let x_max = b / s;
            let lo = x_min.max(lower_bound);
            let hi = x_max.min(upper_bound);
            if lo <= hi {
                for x in lo..=hi {
                    invalid_ids.insert(x * s);
                }
            }
        }
    }

    //...

    invalid_ids.iter().sum()
}

pub fn part_one(input: &str) -> Option<u128> {
    let mut total: u128 = 0;
    for token in input
        .trim()
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        if let Some(pos) = token.find('-') {
            let a = token[..pos].parse::<u128>().ok()?;
            let b = token[pos + 1..].parse::<u128>().ok()?;
            total += sum_double_ids_in_range(a, b);
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut total: u128 = 0;
    for token in input
        .trim()
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        if let Some(pos) = token.find('-') {
            let a = token[..pos].parse::<u128>().ok()?;
            let b = token[pos + 1..].parse::<u128>().ok()?;
            total += sum_repeated_invalid_ids(a, b);
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
