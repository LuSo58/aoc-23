use std::str::FromStr;
use aoc23::{Coord, Direction, Grid, stdin_lines};
use aoc23::Direction::*;

struct Command {
    direction: Direction,
    distance: usize,
    color: [u8; 3],
}

impl FromStr for Command {
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
        let color = u32::from_str_radix(color_string, 16).map_err(|_| ())?;
        Ok(Command {
            direction,
            distance,
            color: [(color >> 16 & 0xff) as u8, (color >> 8 & 0xff) as u8, (color & 0xff) as u8],
        })
    }
}

fn find_limits<'a, I>(commands: I) -> (i32, i32) where I: Iterator<Item=&'a Command> {
    let (low, high, _) = commands.fold((0, 0, 0), |(mut up, mut down, mut current), command| {
        match command.direction {
            North | West => {
                current -= command.distance as i32;
                if up > current {
                    up = current
                }
                (up, down, current)
            }
            South | East => {
                current += command.distance as i32;
                if down < current {
                    down = current
                }
                (up, down, current)
            }
        }
    });
    (low, high)
}

fn main() {
    let commands = stdin_lines()
        .map(|line| {
            line.parse::<Command>()
        })
        .collect::<Result<Vec<_>, _>>()
        .expect("Bad input");
    let (up, down) = find_limits(commands.iter().filter(|&command| { command.direction == North || command.direction == South }));
    let (left, right) = find_limits(commands.iter().filter(|&command| { command.direction == East || command.direction == West }));
    let mut grid = Grid::new((right - left) as usize, (down - up) as usize);
    let start = Coord::new(-left as usize, -right as usize);
    grid[start] = 'S';
}