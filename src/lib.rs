use std::fmt::Debug;
use std::io::stdin;
use std::ops::{Index, IndexMut, Sub};
use std::str::FromStr;
use itertools::Itertools;
use crate::Direction::{East, North, South, West};
#[macro_export]
macro_rules! run {
    ($x:tt) => {
        {
            let start = std::time::Instant::now();
            let (r1, r2) = $x;
            let time = start.elapsed().as_secs_f64() * 1000.0;
            println!("Part 1: {r1}\nPart 2: {r2}\nTime: {time}ms");
        }
    };
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn orthogonal(&self) -> (Self, Self) {
        match self {
            North | South => (East, West),
            East | West => (North, South),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = char;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' | 'W' => Ok(West),
            'R' | 'E' => Ok(East),
            'U' | 'N' => Ok(North),
            'D' | 'S' => Ok(South),
            _ => Err(value),
        }
    }
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            Err(())
        } else {
            s.chars()
                .next()
                .expect("Infallible")
                .try_into()
                .map_err(|_| ())
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn next(&self, direction: Direction) -> Option<Self> {
        match direction {
            North => if self.y > 0 { Some(Self::new(self.x, self.y - 1)) } else { None }
            East => Some(Self::new(self.x + 1, self.y)),
            South => Some(Self::new(self.x, self.y + 1)),
            West => if self.x > 0 { Some(Self::new(self.x - 1, self.y)) } else { None }
        }
    }

    pub fn next_xy(x: usize, y: usize, direction: Direction) -> Option<Self> {
        Coord::new(x, y).next(direction)
    }
}

#[macro_export]
macro_rules! xy {
    ($x:expr, $y:expr) => { Coord::new($x, $y) };
}

impl Sub for Coord {
    type Output = usize;

    fn sub(self, rhs: Self) -> Self::Output {
        self.x.abs_diff(rhs.x) + self.y.abs_diff(rhs.y)
    }
}

impl From<(usize, usize)> for Coord {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0, value.1)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Grid<T> where T: Copy {
    grid: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> where T: Copy {
    pub fn new(width: usize, height: usize) -> Grid<T> where T: Default {
        Grid{
            grid: vec![T::default(); width * height],
            width,
            height,
        }
    }
    pub fn from_input<I, F>(lines: I, conv_fn: F) -> Option<Self>
        where
            I: Iterator<Item=String>,
            F: Fn(char) -> Option<T> + Copy,
    {
        let rows = lines.map(|line| {
            let row = line.chars()
                .map(conv_fn)
                .collect::<Option<Vec<T>>>()?;
            let width = row.len();
            Some((row, width))
        })
            .collect::<Option<Vec<(Vec<T>, usize)>>>()?;
        let height = rows.len();
        if height == 0 {
            None
        } else {
            let width = rows.iter().map(|(_, width)| *width).all_equal_value().ok()?;
            if width == 0 {
                None
            } else {
                let grid = rows.into_iter()
                    .map(|(row, _)| row.into_iter())
                    .flatten()
                    .collect_vec();
                Some(Grid { grid, width, height })
            }
        }
    }
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            Some(&self.grid[x + y * self.width])
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.width && y < self.height {
            Some(&mut self.grid[x + y * self.width])
        } else {
            None
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn iter(&self) -> BorrowedGridRowIterator<'_, T> {
        BorrowedGridRowIterator {
            grid: &self.grid,
            width: self.width,
            height: self.height,
            position: 0,
        }
    }
    pub fn row(&self, y: usize) -> &'_ [T] {
        &self.grid[y * self.width..(y + 1) * self.width]
    }
}

pub struct BorrowedGridRowIterator<'a, T> {
    grid: &'a Vec<T>,
    width: usize,
    height: usize,
    position: usize,
}

impl<'a, T> Iterator for BorrowedGridRowIterator<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.height {
            None
        } else {
            let prev_position = self.position;
            self.position += 1;
            Some(&self.grid[prev_position * self.width..self.position * self.width])
        }
    }
}

pub struct GridRowIterator<T> {
    grid: Vec<T>,
    width: usize,
    height: usize,
    position: usize,
}

impl<T> Iterator for GridRowIterator<T> where T: Copy {
    type Item = Box<[T]>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.height {
            None
        } else {
            let prev_position = self.position;
            self.position += 1;
            Some(self.grid[prev_position * self.width..self.position * self.width].into())
        }
    }
}

impl<T> IntoIterator for Grid<T> where T: Copy {
    type Item = Box<[T]>;
    type IntoIter = GridRowIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        GridRowIterator {
            grid: self.grid,
            width: self.width,
            height: self.height,
            position: 0,
        }
    }
}

impl<'a, T> Index<Coord> for Grid<T> where T: Copy {
    type Output = T;
    fn index(&self, index: Coord) -> &Self::Output {
        self.get(index.x, index.y).expect("Grid index out of bounds")
    }
}

impl<'a, T> IndexMut<Coord> for Grid<T> where T: Copy {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        self.get_mut(index.x, index.y).expect("Grid index out of bounds")
    }
}

pub fn some<T>(value: T) -> Option<T> {
    Some(value)
}

pub fn stdin_lines() -> impl Iterator<Item=String> {
    stdin().lines().map(|x| x.expect("Bad input"))
}