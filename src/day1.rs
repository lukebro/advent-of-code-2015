#[aoc(day1, part1)]
pub fn solve(input: &str) -> i32 {

    let mut floor = 0;

    for c in input.chars() {
        floor += match c {
            ')' => -1,
            '(' => 1,
            _ => 0
        }
    }

    floor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        assert_eq!(solve(&"()()"), 0);
    }

    #[test]
    fn second_example() {
        assert_eq!(solve(&")())())"), -3);
    }
}