/*
After helping the Elves in the kitchen, you were taking a break and helping them re-enact a movie scene when you over-enthusiastically jumped into the garbage chute!

A brief fall later, you find yourself in a garbage smasher. Unfortunately, the door's been magnetically sealed.

As you try to find a way out, you are approached by a family of cephalopods! They're pretty sure they can get the door open, but it will take some time. While you wait, they're curious if you can help the youngest cephalopod with her [math homework](/2021/day/18).

Cephalopod math doesn't look that different from normal math. The math worksheet (your puzzle input) consists of a list of *problems*; each problem has a group of numbers that need to be either *added* (`+`) or *multiplied* (`*`) together.

However, the problems are arranged a little strangely; they seem to be presented next to each other in a very long horizontal list. For example:

```
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +

```

Each problem's numbers are arranged vertically; at the bottom of the problem is the symbol for the operation that needs to be performed. Problems are separated by a full column of only spaces. The left/right alignment of numbers within each problem can be ignored.

So, this worksheet contains four problems:

* `123` \* `45` \* `6` = `*33210*`
* `328` + `64` + `98` = `*490*`
* `51` \* `387` \* `215` = `*4243455*`
* `64` + `23` + `314` = `*401*`

To check their work, cephalopod students are given the *grand total* of adding together all of the answers to the individual problems. In this worksheet, the grand total is `33210` + `490` + `4243455` + `401` = `*4277556*`.

Of course, the actual worksheet is *much* wider. You'll need to make sure to unroll it completely so that you can read the problems clearly.

Solve the problems on the math worksheet. *What is the grand total found by adding together all of the answers to the individual problems?*

--- Part Two ---
The big cephalopods come back to check on how things are going. When they see that your grand total doesn't match the one expected by the worksheet, they realize they forgot to explain how to read cephalopod math.

Cephalopod math is written right-to-left in columns. Each number is given in its own column, with the most significant digit at the top and the least significant digit at the bottom. (Problems are still separated with a column consisting only of spaces, and the symbol at the bottom of the problem is still the operator to use.)

Here's the example worksheet again:

123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
Reading the problems right-to-left one column at a time, the problems are now quite different:

The rightmost problem is 4 + 431 + 623 = 1058
The second problem from the right is 175 * 581 * 32 = 3253600
The third problem from the right is 8 + 248 + 369 = 625
Finally, the leftmost problem is 356 * 24 * 1 = 8544
Now, the grand total is 1058 + 3253600 + 625 + 8544 = 3263827.

Solve the problems on the math worksheet again. What is the grand total found by adding together all of the answers to the individual problems?



 */
advent_of_code::solution!(6);

#[derive(Debug, Clone)]
struct Problem {
    numbers: Vec<u128>,
    operation: char,
}

#[derive(Debug)]
struct Worksheet {
    problems: Vec<Problem>,
}

#[derive(Debug)]
struct WorksheetPart2 {
    problems: Vec<Problem>,
}

impl From<&str> for Worksheet {
    fn from(value: &str) -> Self {
        let lines: Vec<&str> = value.lines().collect();
        let number_lines: Vec<Vec<u128>> = lines
            .iter()
            .take(lines.len() - 1)
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse::<u128>().unwrap())
                    .collect::<Vec<u128>>()
            })
            .collect();
        let problems = lines
            .last()
            .unwrap()
            .split_whitespace()
            .enumerate()
            .map(|(idx, op)| {
                let numbers = number_lines
                    .iter()
                    .map(|line| line[idx])
                    .collect::<Vec<u128>>();
                Problem {
                    numbers,
                    operation: op.chars().next().unwrap(),
                }
            })
            .collect();
        Worksheet { problems }
    }
}

impl Problem {
    fn solve(&self) -> u128 {
        match self.operation {
            '+' => self.numbers.iter().sum(),
            '*' => self.numbers.iter().product(),
            _ => 0,
        }
    }
}

impl From<&str> for WorksheetPart2 {
    fn from(value: &str) -> Self {
        let lines: Vec<&str> = value.lines().collect();
        let mut problems = vec![];
        let mut problem = Problem {
            numbers: vec![],
            operation: ' ',
        };
        let width = lines.iter().map(|l| l.len()).max().unwrap();
        for x in 0..width {
            let mut number = 0;
            for line in &lines {
                let maybe_c = line.chars().nth(x);
                if maybe_c.is_none() {
                    continue;
                }
                let c = maybe_c.unwrap();
                if c.is_ascii_digit() {
                    number = number * 10 + c.to_digit(10).unwrap() as u128;
                } else if c == '+' || c == '*' {
                    problem.operation = c;
                }
            }
            if number != 0 {
                problem.numbers.push(number);
            } else {
                problems.push(problem.clone());
                problem.numbers.clear();
            }
        }
        problems.push(problem);
        WorksheetPart2 { problems }
    }
}

pub fn part_one(input: &str) -> Option<u128> {
    let worksheet = Worksheet::from(input);
    worksheet
        .problems
        .iter()
        .map(|p| p.solve())
        .sum::<u128>()
        .into()
}

pub fn part_two(input: &str) -> Option<u128> {
    let worksheet = WorksheetPart2::from(input);
    worksheet
        .problems
        .iter()
        .map(|p| p.solve())
        .sum::<u128>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
