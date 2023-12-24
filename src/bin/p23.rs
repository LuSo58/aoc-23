use std::collections::{HashMap, HashSet, VecDeque};
use std::collections::hash_map::Entry;
use std::convert::identity;
use itertools::Itertools;
use tinyvec::ArrayVec;
use aoc23::{Coord, Direction, Grid, run, some, stdin_lines, xy};
use aoc23::Direction::{East, North, South, West};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
struct DAGNode {
    coord: Coord,
    in_neighbours: ArrayVec<[(Coord, usize); 4]>,
    out_neighbours: ArrayVec<[(Coord, usize); 4]>,
}

impl DAGNode {
    fn new(coord: Coord) -> Self {
        DAGNode {
            coord,
            in_neighbours: ArrayVec::new(),
            out_neighbours: ArrayVec::new(),
        }
    }
}

fn cell_is_node(grid: &Grid<char>, cell: &Coord) -> bool {
    !cell.orthogonal_neighbours()
        .into_iter()
        .any(|coord| {
            grid.get_coord(&coord)
                .map(|&c| c == '.')
                .expect("Infallible")
        })
}

fn arrow_into_direction(c: char) -> Option<Direction> {
    match c {
        '<' => Some(West),
        '>' => Some(East),
        '^' => Some(North),
        'v' => Some(South),
        _ => None,
    }
}

fn build_dag(grid: &Grid<char>, start: Coord, end: Coord) -> HashMap<Coord, DAGNode> {
    let mut next_cells = VecDeque::new();
    let mut dag_nodes = HashMap::new();
    let mut visited = HashSet::new();
    next_cells.push_back((start.next(South).expect("Infallible"), start, 1));
    dag_nodes.insert(start, DAGNode::new(start));
    visited.insert(start);
    while let Some((cell, from, distance)) = next_cells.pop_front() {
        if !visited.contains(&cell) {
            visited.insert(cell);
            if cell == end {
                let mut node = DAGNode::new(cell);
                node.in_neighbours.push((from, distance));
                dag_nodes.get_mut(&from)
                    .expect("Infallible")
                    .out_neighbours.push((cell, distance));
                dag_nodes.insert(cell, node);
            } else if cell_is_node(grid, &cell) {
                let mut node = DAGNode::new(cell);
                node.in_neighbours.push((from, distance));
                dag_nodes.get_mut(&from)
                    .expect("Infallible")
                    .out_neighbours.push((cell, distance));
                dag_nodes.insert(cell, node);
                cell.orthogonal_neighbours().into_iter()
                    .filter(|&next_cell| arrow_into_direction(grid[next_cell]).map(|direction| {
                        next_cell.next(direction)
                            .map(|next_next_cell| next_next_cell != cell)
                    })
                        .flatten()
                        .unwrap_or(false))
                    .for_each(|next_cell| {
                        next_cells.push_back((next_cell, cell, 1));
                    });
            } else {
                cell.orthogonal_neighbours().into_iter()
                    .for_each(|next_cell| {
                        if grid[next_cell] == '.' || arrow_into_direction(grid[next_cell]).map(|direction| {
                            next_cell.next(direction)
                                .map(|next_next_cell| next_next_cell != cell)
                        }).flatten()
                            .unwrap_or(false) {
                            if next_cell != from {
                                next_cells.push_back((next_cell, from, distance + 1));
                            }
                        }
                    });
            }
        } else {
            if let Some(dag_node) = dag_nodes.get_mut(&cell) {
                dag_node.in_neighbours.push((from, distance));
                dag_nodes.get_mut(&from)
                    .expect("Infallible")
                    .out_neighbours
                    .push((cell, distance));
            }
        }
    }
    dag_nodes
}

fn get_topological_order(dag: &HashMap<Coord, DAGNode>) -> Vec<Coord> {
    let mut order = Vec::with_capacity(dag.len());
    let mut visited = HashSet::new();
    let mut processing_queue = VecDeque::new();
    dag.iter()
        .filter_map(|(coord, node)| {
            if node.in_neighbours.is_empty() {
                Some(coord)
            } else {
                None
            }
        })
        .for_each(|&start| {
            processing_queue.push_back(start);
        });
    while let Some(coord) = processing_queue.pop_back() {
        if !visited.contains(&coord) {
            visited.insert(coord);
            order.push(coord);
            dag.get(&coord)
                .expect("Infallible")
                .out_neighbours
                .iter()
                .for_each(|(out_coord, _)| {
                    if dag.get(out_coord)
                        .expect("Infallible")
                        .in_neighbours
                        .iter()
                        .all(|(in_coord, _)| {
                            visited.contains(in_coord)
                        }) {
                        processing_queue.push_back(*out_coord);
                    }
                });
        }
    }
    order
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
struct Vertex {
    neighbours: ArrayVec<[(Coord, usize); 4]>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
struct SimpleVertex {
    neighbours: ArrayVec<[(u8, usize); 4]>,
}

impl From<DAGNode> for Vertex {
    fn from(value: DAGNode) -> Self {
        let mut neighbours = value.in_neighbours;
        value.out_neighbours
            .into_iter()
            .for_each(|(coord, distance)| {
                if let Some((idx, _)) = neighbours.iter().find_position(|&other| *other == (coord, distance)) {
                    let old_distance = &mut neighbours[idx].1;
                    *old_distance = (*old_distance).max(distance);
                } else {
                    neighbours.push((coord, distance));
                }
            });
        Vertex {
            neighbours,
        }
    }
}

fn recursive_dfs_memoize_adaptor(graph: &HashMap<u8, SimpleVertex>, start: u8, end: u8, remaining_vertices: u64, memory: &mut HashMap<(u64, u8), Option<usize>>) -> Option<usize> {
    if let Some(distance) = memory.get(&(remaining_vertices.clone(), start)) {
        *distance
    } else {
        let tmp = recursive_dfs_memoize(graph, start, end, remaining_vertices.clone(), memory);
        memory.insert((remaining_vertices, start), tmp);
        tmp
    }
}

fn recursive_dfs_memoize(graph: &HashMap<u8, SimpleVertex>, start: u8, end: u8, mut remaining_vertices: u64, memory: &mut HashMap<(u64, u8), Option<usize>>) -> Option<usize> {
    if start == end {
        Some(0)
    } else {
        remaining_vertices &= !(1 << start);
        graph[&start]
            .neighbours
            .iter()
            .filter_map(|(next, distance)| {
                if remaining_vertices & (1 << next) != 0 {
                    Some(recursive_dfs_memoize_adaptor(graph, *next, end, remaining_vertices.clone(), memory).map(|forward_distance| forward_distance + *distance))
                } else {
                    None
                }
            })
            .filter_map(identity)
            .max()
    }
}

fn main() {
    run!({
        let grid = Grid::from_input(stdin_lines(), some)
            .expect("Bad input");
        let start_coord = xy!(1, 0);
        let end_coord = xy!(grid.width() - 2, grid.height() - 1);
        let dag = build_dag(&grid, start_coord, end_coord);
        let order = get_topological_order(&dag);
        let mut distances = HashMap::new();
        distances.insert(start_coord, 0usize);
        for from_coord in &order {
            for (out_neighbour, distance) in &dag[from_coord].out_neighbours {
                let new_distance = distances[from_coord] + *distance;
                match distances.entry(*out_neighbour) {
                    Entry::Occupied(mut entry) => {
                        let distance = entry.get_mut();
                        *distance = (*distance).max(new_distance);
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(new_distance);
                    }
                }
            }
        }
        let graph = dag.into_iter()
            .map(|(coord, node)| {
                (coord, node.into())
            })
            .collect::<HashMap<Coord, Vertex>>();
        let graph_keys = graph.keys()
            .cloned()
            .enumerate()
            .map(|(idx, coord)| (coord, idx as u8))
            .collect::<HashMap<_, _>>();
        let graph = graph.into_iter()
            .map(|(coord, vertex)| {
                (
                    graph_keys[&coord],
                    SimpleVertex {
                        neighbours: vertex.neighbours
                            .into_iter()
                            .map(|(neighbour, distance)| (graph_keys[&neighbour], distance))
                            .collect()
                    }
                )
            })
            .collect::<HashMap<_, _>>();
        let undirected_distance = recursive_dfs_memoize_adaptor(&graph,
                                                                graph_keys[&start_coord],
                                                                graph_keys[&end_coord],
                                                                !0,
                                                                &mut HashMap::new())
            .expect("Infallible");
        (distances[&end_coord], undirected_distance)
    })
}