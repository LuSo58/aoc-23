use std::collections::VecDeque;
use std::convert::identity;
use std::io::stdin;
use std::mem::swap;
use itertools::Itertools;
use rayon::prelude::*;
use aoc23::run;

enum Cell {
    Empty,
    RightMirror,
    LeftMirror,
    HorizontalSplitter,
    VerticalSplitter,
}

impl TryFrom<char> for Cell {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Cell::Empty),
            '/' => Ok(Cell::RightMirror),
            '\\' => Ok(Cell::LeftMirror),
            '-' => Ok(Cell::HorizontalSplitter),
            '|' => Ok(Cell::VerticalSplitter),
            _ => Err(value),
        }
    }
}

#[derive(Default, Copy, Clone)]
struct CellBeams {
    top_down: bool,
    bottom_up: bool,
    left_to_right: bool,
    right_to_left: bool,
}

impl CellBeams {
    fn append(&mut self, direction: Direction) -> bool {
        let position = match direction {
            Direction::North => &mut self.bottom_up,
            Direction::East => &mut self.left_to_right,
            Direction::South => &mut self.top_down,
            Direction::West => &mut self.right_to_left,
        };
        let prev = *position;
        *position = true;
        !prev
    }

    fn is_energised(&self) -> bool {
        self.right_to_left || self.left_to_right || self.top_down || self.bottom_up
    }
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn apply_cell(&self, cell: &Cell) -> (Self, Option<Self>) {
        match cell {
            Cell::Empty => (*self, None),
            Cell::RightMirror => (match self {
                Direction::North => Direction::East,
                Direction::East => Direction::North,
                Direction::South => Direction::West,
                Direction::West => Direction::South,
            }, None),
            Cell::LeftMirror => (match self {
                Direction::North => Direction::West,
                Direction::East => Direction::South,
                Direction::South => Direction::East,
                Direction::West => Direction::North,
            }, None),
            Cell::HorizontalSplitter => match self {
                Direction::North | Direction::South => (Direction::West, Some(Direction::East)),
                _ => (*self, None),
            }
            Cell::VerticalSplitter => match self {
                Direction::West | Direction::East => (Direction::North, Some(Direction::South)),
                _ => (*self, None),
            }
        }
    }
}

struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn next(&self, direction: Direction) -> Option<Self> {
        match direction {
            Direction::North => if self.y > 0 { Some(Self::new(self.x, self.y - 1)) } else { None }
            Direction::East => Some(Self::new(self.x + 1, self.y)),
            Direction::South => Some(Self::new(self.x, self.y + 1)),
            Direction::West => if self.x > 0 { Some(Self::new(self.x - 1, self.y)) } else { None }
        }
    }

    fn next_xy(x: usize, y: usize, direction: Direction) -> Option<Self> {
        Coord::new(x, y).next(direction)
    }
}

#[allow(unused)]
fn print_cell_beams(beams: &Vec<Vec<CellBeams>>) {
    beams.iter()
        .for_each(|row| {
            row.iter()
                .for_each(|cell_beams| {
                    if cell_beams.is_energised() {
                        print!("#");
                    } else {
                        print!(".");
                    }
                });
            println!();
        });
    println!();
}

fn energize_grid(grid: &Vec<Vec<Cell>>, start_coord: Coord, start_direction: Direction) -> Vec<Vec<CellBeams>> {
    let width = grid[0].len();
    let height = grid.len();
    let mut beams = vec![vec![CellBeams::default(); width]; height];
    let mut prev_directions = VecDeque::new();
    let mut next_directions = VecDeque::new();
    next_directions.push_back((start_coord, start_direction));
    while !next_directions.is_empty() {
        swap(&mut prev_directions, &mut next_directions);
        while let Some((Coord { x, y }, direction)) = prev_directions.pop_back() {
            if beams[y][x].append(direction) {
                let (first, second) = direction.apply_cell(&grid[y][x]);
                [Some(first), second].into_iter()
                    .filter_map(identity)
                    .for_each(|direction| {
                        Coord::next_xy(x, y, direction).map(|next| {
                            if next.x < width && next.y < height {
                                next_directions.push_back((next, direction));
                            }
                        });
                    });
            }
        }
    }
    beams
}

fn count_energized_cells(beams: &Vec<Vec<CellBeams>>) -> usize {
    beams.iter()
        .map(|row| {
            row.iter()
                .map(CellBeams::is_energised)
                .map(<bool as Into<usize>>::into)
        })
        .flatten()
        .sum::<usize>()
}

fn main() {
    run!({
        let grid = stdin().lines()
            .map(|line| {
                let line = line.expect("Bad input");
                line.chars()
                    .map(<char as TryInto<Cell>>::try_into)
                    .collect::<Result<Vec<_>, _>>()
                    .expect("Bad input")
            })
            .collect_vec();
        assert!(!grid.is_empty());
        assert!(!grid[0].is_empty());
        assert!(grid.iter().map(Vec::len).all_equal());
        let width = grid[0].len();
        let height = grid.len();
        let first = count_energized_cells(&energize_grid(&grid, Coord::new(0, 0), Direction::East));
        let entrypoints = (0..width)
            .map(|x| {
                [(0, Direction::South), (height - 1, Direction::North)].into_iter()
                    .map(move |(y, direction)| (Coord::new(x, y), direction))
            }).flatten()
            .chain((0..height).map(|y| {
                [(0, Direction::East), (width - 1, Direction::West)].into_iter()
                    .map(move |(x, direction)| (Coord::new(x, y), direction))
            })
            .flatten())
            .collect_vec();
        let second = entrypoints.into_par_iter()
            .map(|(coord, direction)| energize_grid(&grid, coord, direction))
            .map(|beams| count_energized_cells(&beams))
            .max()
            .expect("Infallible");
        (first, second)
    })
}
