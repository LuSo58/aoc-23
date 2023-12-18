use std::collections::VecDeque;
use std::convert::identity;
use std::mem::swap;
use itertools::Itertools;
use rayon::prelude::*;
use aoc23::{Coord, Direction, Grid, run, stdin_lines, xy};

#[derive(Copy, Clone)]
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

impl Cell {
    fn translate_direction(&self, direction: &Direction) -> (Direction, Option<Direction>) {
        match self {
            Cell::Empty => (*direction, None),
            Cell::RightMirror => (match direction {
                Direction::North => Direction::East,
                Direction::East => Direction::North,
                Direction::South => Direction::West,
                Direction::West => Direction::South,
            }, None),
            Cell::LeftMirror => (match direction {
                Direction::North => Direction::West,
                Direction::East => Direction::South,
                Direction::South => Direction::East,
                Direction::West => Direction::North,
            }, None),
            Cell::HorizontalSplitter => match direction {
                Direction::North | Direction::South => (Direction::West, Some(Direction::East)),
                _ => (*direction, None),
            }
            Cell::VerticalSplitter => match direction {
                Direction::West | Direction::East => (Direction::North, Some(Direction::South)),
                _ => (*direction, None),
            }
        }
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

fn energize_grid(grid: &Grid<Cell>, start_coord: Coord, start_direction: Direction) -> Grid<CellBeams> {
    let mut beams = Grid::<CellBeams>::new(grid.width(), grid.height());
    let mut prev_directions = VecDeque::new();
    let mut next_directions = VecDeque::new();
    next_directions.push_back((start_coord, start_direction));
    while !next_directions.is_empty() {
        swap(&mut prev_directions, &mut next_directions);
        while let Some((Coord { x, y }, direction)) = prev_directions.pop_back() {
            if beams[xy!(x, y)].append(direction) {
                let (first, second) = grid[xy!(x, y)].translate_direction(&direction);
                [Some(first), second].into_iter()
                    .filter_map(identity)
                    .for_each(|direction| {
                        Coord::next_xy(x, y, direction).map(|next| {
                            if next.x < grid.width() && next.y < grid.height() {
                                next_directions.push_back((next, direction));
                            }
                        });
                    });
            }
        }
    }
    beams
}

fn count_energized_cells(beams: &Grid<CellBeams>) -> usize {
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
        let grid = Grid::from_input(stdin_lines(), |c| c.try_into().ok()).expect("Bad input");
        let first = count_energized_cells(&energize_grid(&grid, Coord::new(0, 0), Direction::East));
        let entrypoints = (0..grid.width())
            .map(|x| {
                [(0, Direction::South), (grid.height() - 1, Direction::North)].into_iter()
                    .map(move |(y, direction)| (Coord::new(x, y), direction))
            }).flatten()
            .chain((0..grid.height()).map(|y| {
                [(0, Direction::East), (grid.width() - 1, Direction::West)].into_iter()
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
