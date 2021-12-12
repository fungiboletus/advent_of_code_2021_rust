// Totally unecessary but fun
use rayon::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Forward,
}

pub struct Instruction {
    direction: Direction,
    units: i64,
}

pub struct Position {
    horizontal: i64,
    depth: i64,
}

pub fn parse_submarine_instructions(input: &str) -> Vec<Instruction> {
    return input
        .par_split(|c| c == '\n')
        .map(|s| {
            let instruction = s.split_whitespace().collect::<Vec<&str>>();
            let direction = match instruction[0] {
                "up" => Direction::Up,
                "down" => Direction::Down,
                "forward" => Direction::Forward,
                _ => panic!("Unknown direction"),
            };
            let units = instruction[1]
                .parse::<i64>()
                .expect("Could not parse units number");
            return Instruction { direction, units };
        })
        .collect();
}

pub fn day_2_part_1(data: &str) -> i64 {
    let instructions = parse_submarine_instructions(data);
    let final_position = instructions
        .par_iter()
        .fold(
            || Position {
                horizontal: 0,
                depth: 0,
            },
            |pos, instruction| match instruction.direction {
                Direction::Up => Position {
                    horizontal: pos.horizontal,
                    depth: pos.depth - instruction.units,
                },
                Direction::Down => Position {
                    horizontal: pos.horizontal,
                    depth: pos.depth + instruction.units,
                },
                Direction::Forward => Position {
                    horizontal: pos.horizontal + instruction.units,
                    depth: pos.depth,
                },
            },
        )
        .reduce(
            || Position {
                horizontal: 0,
                depth: 0,
            },
            |a, b| Position {
                horizontal: a.horizontal + b.horizontal,
                depth: a.depth + b.depth,
            },
        );

    return final_position.horizontal * final_position.depth;
}

pub fn day_2_part_2(data: &str) -> i64 {
    let instructions = parse_submarine_instructions(data);
    let mut horizontal: i64 = 0;
    let mut depth: i64 = 0;
    let mut aim: i64 = 0;

    for instruction in instructions {
        match instruction.direction {
            Direction::Up => aim -= instruction.units,
            Direction::Down => aim += instruction.units,
            Direction::Forward => {
                horizontal += instruction.units;
                depth += instruction.units * aim;
            }
        }
    }
    return horizontal * depth;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2";

    #[test]
    fn test_day_2_part_1() {
        assert_eq!(day_2_part_1(EXAMPLE), 150);
    }

    #[test]
    fn test_day_2_part_2() {
        assert_eq!(day_2_part_2(EXAMPLE), 900);
    }
}
