use itertools::Itertools;

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.trim())
        .map(|line| i32::from_str_radix(line, 10).unwrap_or(0))
        .collect()
}

#[aoc(day1, part1)]
pub fn solver_part1(input: &Vec<i32>) -> u32 {
    input.iter().tuple_windows().fold(
        0,
        |acc, (elem, next)| if elem < next { acc + 1 } else { acc },
    )
}

#[aoc(day1, part2)]
pub fn solver_part2(input: &Vec<i32>) -> u32 {
    input
        .iter()
        .tuple_windows()
        .fold(0, |acc, (a, _b, _c, d)| if a < d { acc + 1 } else { acc })
}

// :: ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "199
                     200
                     208
                     210
                     200
                     207
                     240
                     269
                     260
                     263";

        assert_eq!(solver_part1(&generator(input)), 7);
    }

    #[test]
    fn example_2() {
        let input = "199
                     200
                     208
                     210
                     200
                     207
                     240
                     269
                     260
                     263";

        assert_eq!(solver_part2(&generator(input)), 5);
    }
}
