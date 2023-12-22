use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;
use itertools::Itertools;
use aoc23::{Coord, Grid, run, stdin_lines};

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
    #[allow(unused)]
    fn width(&self) -> usize {
        self.max_corner.x - self.min_corner.x + 1
    }
    #[allow(unused)]
    fn depth(&self) -> usize {
        self.max_corner.y - self.min_corner.y + 1
    }
    fn height(&self) -> usize {
        self.max_corner.z - self.min_corner.z + 1
    }
    #[allow(unused)]
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
        Ok(Brick::new(lhs, rhs))
    }
}

#[allow(unused)]
fn print_occupancy_grid(grid3d: &Vec<Grid<Option<usize>>>) {
    grid3d.into_iter()
        .for_each(|level| {
            level.iter()
                .for_each(|row| {
                    row.iter()
                        .for_each(|cell| {
                            match cell {
                                None => { print!("  .  ") }
                                Some(idx) => { print!("{idx:^5}") }
                            }
                        });
                    println!()
                });
            println!("{:-<1$}", "", level.width() * 5)
        });
}

fn push_supported_bricks_to_queue(queue: &mut BinaryHeap<(usize, usize)>, bricks: &Vec<Brick>, deleted_brick_idx: usize, supported_bricks: &Vec<HashSet<usize>>) {
    supported_bricks[deleted_brick_idx].iter()
        .for_each(|&supported_brick_idx| {
            queue.push((bricks[supported_brick_idx].min_corner.z, supported_brick_idx));
        });
}

fn main() {
    run!({
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
        let mut grid_size = bricks.iter()
            .fold(Coord3D::default(), |max_coord, brick| {
                brick.max_corner.maximum(&max_coord)
            });
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
        bricks.sort();
        let mut supporting_bricks = vec![HashSet::new(); bricks.len()];
        occupancy_grid3d.windows(2)
            .into_iter()
            .for_each(|layers| {
                let bottom = &layers[0];
                let top = &layers[1];
                bottom.iter()
                    .zip(top.iter())
                    .for_each(|(bottom_row, top_row)| {
                        bottom_row.into_iter()
                            .zip(top_row.into_iter())
                            .for_each(|(bottom_cell, top_cell)| {
                                match (bottom_cell, top_cell) {
                                    (Some(bottom_idx), Some(top_idx)) => {
                                        if bottom_idx != top_idx {
                                            supporting_bricks[*top_idx].insert(*bottom_idx);
                                        }
                                    }
                                    _ => {}
                                }
                            });
                    });
            });
        let unmovable_bricks = supporting_bricks.iter()
            .filter_map(|supports| {
                if supports.len() == 1 {
                    Some(*supports.into_iter().next().expect("Infallible"))
                } else {
                    None
                }
            })
            .unique()
            .collect_vec();
        let count_removable_bricks = bricks.len() - unmovable_bricks.len();
        let mut supported_bricks = vec![HashSet::new(); bricks.len()];
        supporting_bricks.iter()
            .enumerate()
            .for_each(|(supported_idx, supports)| {
                supports.into_iter()
                    .for_each(|support_idx| {
                        supported_bricks[*support_idx].insert(supported_idx);
                    })
            });
        let mut chain_reaction_cache = HashMap::new();
        let chain_reaction_sum = unmovable_bricks.into_iter()
            .sorted()
            .rev()
            .map(|unmovable_brick_idx| {
                let mut deleted_bricks = HashSet::new();
                deleted_bricks.insert(unmovable_brick_idx);
                let mut chain_reaction_queue = BinaryHeap::new();
                push_supported_bricks_to_queue(&mut chain_reaction_queue, &bricks, unmovable_brick_idx, &supported_bricks);
                while let Some((_, supported_brick_idx)) = chain_reaction_queue.pop() {
                    if supporting_bricks[supported_brick_idx].iter()
                        .all(|supporting_brick_idx| deleted_bricks.contains(supporting_brick_idx)) {
                        match chain_reaction_cache.get(&supported_brick_idx) {
                            Some(already_deleted_bricks) => {
                                deleted_bricks.extend(already_deleted_bricks);
                            }
                            None => {
                                deleted_bricks.insert(supported_brick_idx);
                                push_supported_bricks_to_queue(&mut chain_reaction_queue, &bricks, supported_brick_idx, &supported_bricks);
                            }
                        }
                    }
                }
                let deleted_count = deleted_bricks.len() - 1;
                chain_reaction_cache.insert(unmovable_brick_idx, deleted_bricks);
                deleted_count
            })
            .sum::<usize>();
        (count_removable_bricks, chain_reaction_sum)
    });
}