use std::collections::{HashMap, HashSet};
use std::convert::identity;
use std::iter::zip;
use std::ops::BitXor;
use itertools::Itertools;
use aoc23::{Grid, xy, run, some, stdin_lines};
use aoc23::Coord;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Node {
    coord: Coord,
    neighbours: [Coord; 2],
}

impl Node {
    fn new(coord: Coord, neighbours: (Coord, Coord)) -> Self {
        Self {
            coord,
            neighbours: [neighbours.0, neighbours.1],
        }
    }
}

#[allow(unused)]
fn print_path(grid: &Grid<char>, path: &Vec<((Coord, Coord), (Coord, Coord))>) {
    let path = path.iter().cloned().map(|((a, b), (c, d))| [a, b, c, d].into_iter()).flatten().collect::<HashSet<_>>();
    grid.iter()
        .enumerate()
        .map(|(y, line)| {
            line.into_iter()
                .enumerate()
                .map(|(x, &c)| {
                    if c == 'S' {
                        'S'
                    } else if path.contains(&(x, y).into()) {
                        'X'
                    } else {
                        ' '
                    }
                })
                .for_each(|c| print!("{c}"));
        }).for_each(|_| println!());
}

fn main() {
    run!({
        let grid = Grid::from_input(stdin_lines(), some).expect("Bad input");
        let mut start = None;
        let graph = grid.iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map({
                        let grid = &grid;
                        move |(x, c)| {
                            let mut start = None;
                            let neighbours = match c {
                                '|' => if y > 0 { Some(((x, y - 1), (x, y + 1))) } else { None },
                                '-' => if x > 0 { Some(((x - 1, y), (x + 1, y))) } else { None },
                                'F' => Some(((x + 1, y), (x, y + 1))),
                                '7' => if x > 0 { Some(((x - 1, y), (x, y + 1))) } else { None },
                                'L' => if y > 0 { Some(((x + 1, y), (x, y - 1))) } else { None },
                                'J' => if x > 0 && y > 0 { Some(((x - 1, y), (x, y - 1))) } else { None },
                                'S' => Some({
                                    let mut neightbours = Vec::new();
                                    let mut up = false;
                                    let mut down = false;
                                    let mut left = false;
                                    let mut right = false;
                                    if x > 0 && ['F', 'L', '-'].contains(&grid[xy!(x - 1, y)]) {
                                        neightbours.push((x - 1, y));
                                        left = true;
                                    }
                                    if ['7', 'J', '-'].contains(&grid[xy!(x + 1, y)]) {
                                        neightbours.push((x + 1, y));
                                        right = true;
                                    }
                                    if y > 0 && ['F', '7', '|'].contains(&grid[xy!(x, y - 1)]) {
                                        neightbours.push((x, y - 1));
                                        up = true
                                    }
                                    if ['L', 'J', '|'].contains(&grid[xy!(x, y + 1)]) {
                                        neightbours.push((x, y + 1));
                                        down = true;
                                    }
                                    assert_eq!(neightbours.len(), 2);
                                    assert!(start.is_none());
                                    let start_symbol = match (up, down, left, right) {
                                        (true, true, false, false) => '|',
                                        (false, false, true, true) => '-',
                                        (true, false, true, false) => 'J',
                                        (true, false, false, true) => 'L',
                                        (false, true, true, false) => '7',
                                        (false, true, false, true) => 'F',
                                        _ => panic!("Infallible")
                                    };
                                    start = Some((Coord::new(x, y), start_symbol));
                                    (neightbours[0], neightbours[1])
                                }),
                                _ => None
                            };
                            neighbours.map(|(n1, n2)| (Coord::new(x, y), Node::new((x, y).into(), (n1.into(), n2.into())), start))
                        }
                    })
            })
            .flatten()
            .filter_map(identity)
            .map(|(coord, neighbours, s)| {
                if s.is_some() {
                    start = s;
                }
                (coord, neighbours)
            })
            .collect::<HashMap<_, _>>();
        assert!(start.is_some());
        let (start_coord, start_symbol) = start.unwrap();
        let start_neighbours = graph[&start_coord].neighbours;
        let create_iter = |coord: Coord, start: Coord| {
            let graph = &graph;
            (0..).scan((start, coord), move |(prev, curr), _| {
                if *curr == start {
                    None
                } else {
                    let next: [Coord; 1] = graph[&curr].neighbours
                        .iter()
                        .cloned()
                        .filter(|next| *next != *prev).collect::<Vec<_>>().try_into().expect("Bad number of neighbours");
                    let next = next[0];
                    *prev = *curr;
                    *curr = next;
                    Some((*prev, *curr))
                }
            })
        };
        let path = zip(create_iter(start_neighbours[0], start_coord), create_iter(start_neighbours[1], start_coord))
            .take_while(|((lhs, _), (rhs, _))| {
                *lhs != *rhs
            })
            .collect_vec();
        let distance = path.len() + 1;
        let path_nodes = path.iter().cloned().map(|((a, b), (c, d))| [a, b, c, d].into_iter()).flatten().collect::<HashSet<_>>();
        let cell_count = grid.into_iter()
            .enumerate()
            .map(|(y, line)| {
                line.into_iter()
                    .enumerate()
                    .map({
                        let path_nodes = &path_nodes;
                        move |(x, &c)| {
                            if c == 'S' {
                                start_symbol
                            } else if path_nodes.contains(&(x, y).into()) {
                                c
                            } else {
                                '.'
                            }
                        }
                    })
                    .fold((false, Option::<bool>::None, 0usize), |(inside, horizontal_top_down, count), c| {
                        if let Some(top_down) = horizontal_top_down {
                            match c {
                                '-' => (inside, Some(top_down), count),
                                'J' => (inside.bitxor(!top_down), None, count),
                                '7' => (inside.bitxor(top_down), None, count),
                                _ => panic!("Infallible"),
                            }
                        } else {
                            match c {
                                '|' => (!inside, None, count),
                                'F' => (inside, Some(false), count),
                                'L' => (inside, Some(true), count),
                                '.' => (inside, None, count + inside as usize),
                                ' ' => (inside, None, count),
                                _ => panic!("Infallible"),
                            }
                        }
                    }).2
            })
            .sum::<usize>();
        (distance, cell_count)
    });
}