use std::collections::HashMap;
use std::io::stdin;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use num::integer::lcm;
use aoc23::run;
use crate::Direction::{Left, Right};

struct Node {
    left: usize,
    right: usize,
}

enum Direction {
    Left,
    Right,
}

fn main() {
    run!({
        let mut lines = stdin().lines();
        let instructions = lines
            .next()
            .expect("Not enough input")
            .expect("Error reading")
            .chars()
            .map(|c| match c {
                'L' => Left,
                'R' => Right,
                _ => panic!("Bad input"),
            })
            .collect::<Vec<_>>();
        let nodes = lines
            .skip(1)
            .enumerate()
            .map(|(n, line)|
                match line.unwrap().split_once('=') {
                    Some((name, directions)) => {
                        match directions.trim().split_once(',') {
                            Some((left, right)) => {
                                (
                                    name.trim().to_string(),
                                    (
                                        n,
                                        left.trim().strip_prefix('(').unwrap().trim().to_string(),
                                        right.trim().strip_suffix(')').unwrap().trim().to_string(),
                                    )
                                )
                            }
                            None => panic!()
                        }
                    }
                    None => panic!()
                }
            )
            .collect::<HashMap<_, _>>();
        let one_step_graph = nodes
            .values()
            .map(|(n, left, right)| {
                (n, Node {
                    left: nodes.get(left).unwrap().0,
                    right: nodes.get(right).unwrap().0,
                })
            })
            .sorted_by_key(|(&n, _)| n)
            .map(|(_, node)| node)
            .collect::<Vec<_>>();
        let graph = (0..one_step_graph.len())
            .into_iter()
            .map(|n| {
                instructions
                    .iter()
                    .fold(n, |n, direction| {
                        match direction {
                            Left => one_step_graph[n].left,
                            Right => one_step_graph[n].right,
                        }
                    })
            })
            .collect::<Vec<_>>();
        let starting_index = nodes.get("AAA").unwrap().0;
        let ending_index = nodes.get("ZZZ").unwrap().0;
        let len_1 = (0usize..).into_iter()
            .fold_while(starting_index, |n, len| {
                if n == ending_index {
                    Done(len)
                } else {
                    Continue(graph[n])
                }
            }).into_inner() * instructions.len();
        let starting_indexes = nodes.iter()
            .filter_map(|(name, (n, _, _))| {
                if name.ends_with('A') {
                    Some(*n)
                } else {
                    None
                }
            }).collect::<Vec<_>>();
        let ending_indexes = nodes.iter()
            .filter_map(|(name, (n, _, _))| {
                if name.ends_with('Z') {
                    Some(*n)
                } else {
                    None
                }
            }).collect::<Vec<_>>();
        let len_2 = starting_indexes.into_iter().map(|n| {
            (0usize..).into_iter()
                .fold_while(n, |n, len| {
                    if ending_indexes.contains(&n) {
                        Done(len)
                    } else {
                        Continue(graph[n])
                    }
                }).into_inner()
        }).reduce(|n, m| {
            lcm(n, m)
        }).unwrap() * instructions.len();
        (len_1, len_2)
    });
}