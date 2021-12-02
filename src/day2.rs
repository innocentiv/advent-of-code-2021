pub enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| line.trim().split(' '))
        .map(|mut pair| {
            (
                pair.next().unwrap(),
                pair.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .map(|tuple| match tuple {
            ("forward", quantity) => Command::Forward(quantity),
            ("up", quantity) => Command::Up(quantity),
            ("down", quantity) => Command::Down(quantity),
            _ => panic!(),
        })
        .collect()
}

struct Coordinates {
    depth: i32,
    horizontal: i32,
}

#[aoc(day2, part1, original)]
pub fn solver_part1(input: &Vec<Command>) -> i32 {
    let result = input.iter().fold(
        Coordinates {
            depth: 0,
            horizontal: 0,
        },
        |acc, command| match command {
            Command::Forward(quantity) => Coordinates {
                depth: acc.depth,
                horizontal: acc.horizontal + quantity,
            },
            Command::Up(quantity) => Coordinates {
                depth: acc.depth - quantity,
                horizontal: acc.horizontal,
            },
            Command::Down(quantity) => Coordinates {
                depth: acc.depth + quantity,
                horizontal: acc.horizontal,
            },
        },
    );
    result.horizontal * result.depth
}

struct CoordinatesWithAim {
    aim: i32,
    depth: i32,
    horizontal: i32,
}

#[aoc(day2, part2, original)]
pub fn solver_part2(input: &Vec<Command>) -> i32 {
    let result = input.iter().fold(
        CoordinatesWithAim {
            aim: 0,
            depth: 0,
            horizontal: 0,
        },
        |acc, command| match command {
            Command::Forward(quantity) => CoordinatesWithAim {
                aim: acc.aim,
                depth: acc.depth + quantity * acc.aim,
                horizontal: acc.horizontal + quantity,
            },
            Command::Up(quantity) => CoordinatesWithAim {
                aim: acc.aim - quantity,
                ..acc
            },
            Command::Down(quantity) => CoordinatesWithAim {
                aim: acc.aim + quantity,
                ..acc
            },
        },
    );
    result.horizontal * result.depth
}

// :: ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "forward 5
                    down 5
                    forward 8
                    up 3
                    down 8
                    forward 2";

        assert_eq!(solver_part1(&generator(input)), 150);
    }

    #[test]
    fn example_2() {
        let input = "forward 5
                    down 5
                    forward 8
                    up 3
                    down 8
                    forward 2";

        assert_eq!(solver_part2(&generator(input)), 900);
    }
}
