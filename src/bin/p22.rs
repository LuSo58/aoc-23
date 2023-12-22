use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap};
use std::str::FromStr;
use itertools::Itertools;
use aoc23::{Coord, Grid, stdin_lines};

#[derive(Copy, Clone, Default, Hash, Debug, Ord, Eq, PartialEq)]
struct Coord3D {
    x: usize,
    y: usize,
    z: usize,
}

impl Coord3D {
    fn minimum(&self, other: &Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }
    fn maximum(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }
}

impl FromStr for Coord3D {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s.split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| Self::Err::default())?;
        if coords.len() != 3 {
            Err(Self::Err::default())
        } else {
            Ok(Coord3D {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            })
        }
    }
}

impl PartialOrd for Coord3D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.z.cmp(&other.z)
            .then(self.y.cmp(&other.y))
            .then(self.x.cmp(&other.x)))
    }
}

#[derive(Copy, Clone, Default, Hash, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Brick {
    min_corner: Coord3D,
    max_corner: Coord3D,
}

impl Brick {
    fn new(lhs: Coord3D, rhs: Coord3D) -> Self {
        Self {
            min_corner: lhs.minimum(&rhs),
            max_corner: lhs.maximum(&rhs),
        }
    }
    fn width(&self) -> usize {
        self.max_corner.x - self.min_corner.x + 1
    }
    fn depth(&self) -> usize {
        self.max_corner.y - self.min_corner.y + 1
    }
    fn height(&self) -> usize {
        self.max_corner.z - self.min_corner.z + 1
    }
    fn size(&self) -> Coord3D {
        Coord3D {
            x: self.width(),
            y: self.depth(),
            z: self.height(),
        }
    }
    fn descend(&mut self, distance: usize) {
        self.min_corner.z -= distance;
        self.max_corner.z -= distance;
    }
}

impl FromStr for Brick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lhs, rhs) = s.split_once('~')
            .ok_or(Self::Err::default())?;
        let lhs = lhs.parse()?;
        let rhs = rhs.parse()?;
        Ok(Brick {
            min_corner: lhs,
            max_corner: rhs,
        })
    }
}

fn get_tops_and_bottoms(bricks: &Vec<Brick>) -> (BTreeMap<Coord3D, usize>, BTreeMap<Coord3D, usize>) {
    bricks.iter()
        .enumerate()
        .map(|(i, brick)| {
            ((brick.max_corner, i), (brick.min_corner, i))
        })
        .unzip()
}

fn main() {
    let mut bricks = stdin_lines()
        .map(|line| line.parse::<Brick>().expect("Bad input"))
        .sorted()
        .collect::<Vec<_>>();
    bricks.iter()
        .for_each(|brick| {
            assert!((brick.min_corner.x != brick.max_corner.x) as i32 +
                (brick.min_corner.y != brick.max_corner.y) as i32 +
                (brick.min_corner.z != brick.max_corner.z) as i32 <= 1);
        });
    // let (tops, bottoms) = get_tops_and_bottoms(&bricks);
    let mut grid_size = bricks.iter()
        .fold(Coord3D::default(), |max_coord, brick| {
            brick.max_corner.maximum(&max_coord)
        });
    // let bottom_bricks = bricks.iter()
    //     .enumerate()
    //     .map(|(i, brick)| {
    //         (brick.min_corner, i)
    //     })
    //     .collect::<BinaryHeap<_>>();
    grid_size.x += 1;
    grid_size.y += 1;
    grid_size.z += 1;

    let mut occupancy_grid3d = vec![Grid::<Option<usize>>::new(grid_size.x, grid_size.y); grid_size.z];
    for (idx, brick) in bricks.iter_mut().enumerate() {
        let level_coords = (brick.min_corner.x..=brick.max_corner.x)
            .cartesian_product(brick.min_corner.y..=brick.max_corner.y)
            .map(|(x, y)| Coord::new(x, y))
            .collect_vec();
        let bottom_free_z = (0..=brick.min_corner.z).rev()
            .find(|&z| {
                level_coords.iter()
                    .any(|coord| {
                        occupancy_grid3d[z][*coord].is_some()
                    })
            })
            .map(|z| z + 1)
            .unwrap_or(0);
        (bottom_free_z..bottom_free_z + brick.height())
            .for_each(|z| {
                level_coords.iter()
                    .for_each(|coord| {
                        occupancy_grid3d[z][*coord] = Some(idx)
                    })
            });
        brick.descend(brick.min_corner.z - bottom_free_z);
    }
}