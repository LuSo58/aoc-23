use std::str::FromStr;
use itertools::Itertools;
use aoc23::{Direction, run, stdin_lines};
use aoc23::Direction::*;

#[derive(Copy, Clone)]
struct Command {
    direction: Direction,
    distance: usize,
}

#[derive(Copy, Clone)]
struct Line {
    normal_command: Command,
    color_command: Command,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ');
        let direction = iter.next()
            .ok_or(())?
            .parse()
            .map_err(|_| ())?;
        let distance = iter.next()
            .ok_or(())?
            .parse()
            .map_err(|_| ())?;
        let color_string = iter.next()
            .ok_or(())?
            .strip_prefix("(#")
            .ok_or(())?
            .strip_suffix(")")
            .ok_or(())?;
        let color_distance = usize::from_str_radix(&color_string[0..5], 16).map_err(|_| ())?;
        let color_direction = match &color_string[5..6] {
            "0" => Ok(East),
            "1" => Ok(South),
            "2" => Ok(West),
            "3" => Ok(North),
            _ => Err(())
        }?;
        Ok(Line {
            normal_command: Command {
                direction,
                distance,
            },
            color_command: Command {
                direction: color_direction,
                distance: color_distance,
            },
        })
    }
}

fn calculate_area(commands: Vec<Command>) -> usize {
    commands.into_iter()
        .fold((0, 0), |(area, current_y), Command{ direction, distance }| {
            let distance = distance as isize;
            match direction {
                East => (area - distance * current_y, current_y),
                West => (area + distance * (current_y + 1), current_y),
                North => (area, current_y - distance),
                South => (area + distance, current_y + distance),
            }
        }).0 as usize + 1
}

fn main() {
    run!({
        let commands = stdin_lines()
            .map(|line| {
                line.parse::<Line>()
            })
            .collect::<Result<Vec<_>, _>>()
            .expect("Bad input");
        let normal_commands = commands.iter().map(|Line { normal_command, color_command: _ }| *normal_command).collect_vec();
        let color_commands = commands.iter().map(|Line { normal_command: _, color_command }| *color_command).collect_vec();
        (calculate_area(normal_commands), calculate_area(color_commands))
    })
}