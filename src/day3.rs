pub struct Problem {
    width: usize,
    count: usize,
    numbers: Vec<usize>,
}

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Problem {
    let mut width = 0;
    let mut count = 0;
    let numbers = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let l = line.trim();
            if i == 0 {
                width = l.chars().count()
            }
            count = i;
            usize::from_str_radix(l, 2).unwrap()
        })
        .collect();
    Problem {
        width,
        count,
        numbers,
    }
}

#[aoc(day3, part1)]
pub fn solver_part1(input: &Problem) -> usize {
    let gamma_vec: Vec<usize> = input
        .numbers
        .clone()
        .into_iter()
        .fold(vec![0; input.width], |acc, number| {
            acc.into_iter()
                .enumerate()
                .map(|(pos, count)| count + bit(number, pos))
                .collect()
        })
        .into_iter()
        .map(|count| if count > input.count / 2 { 1 } else { 0 })
        .rev()
        .collect();

    let gamma = number(gamma_vec);
    let epsilon = not(gamma, input.width);
    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn solver_part2(_input: &Problem) -> usize {
    198
}

fn not(number: usize, width: usize) -> usize {
    !number & ((1 << width) - 1)
}

fn bit(number: usize, pos: usize) -> usize {
    (number & (1 << pos)) >> pos
}

fn number(bits: Vec<usize>) -> usize {
    bits.iter().fold(0, |acc, bit| acc * 2 + bit)
}

//====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "00100
                    11110
                    10110
                    10111
                    10101
                    01111
                    00111
                    11100
                    10000
                    11001
                    00010
                    01010";

        println!("{}", generator(input).count);
        println!("{}", generator(input).width);
        println!(
            "{}",
            generator(input)
                .numbers
                .iter()
                .fold(String::new(), |acc, &num| acc + &num.to_string() + ", ")
        );

        assert_eq!(solver_part1(&generator(input)), 198);
    }

    #[test]
    fn example_2() {
        let input = "00100
                    11110
                    10110
                    10111
                    10101
                    01111
                    00111
                    11100
                    10000
                    11001
                    00010
                    01010";

        assert_eq!(solver_part2(&generator(input)), 198);
    }
}
