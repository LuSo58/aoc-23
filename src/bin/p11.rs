use std::collections::BTreeSet;
use std::convert::identity;
use itertools::Itertools;
use aoc23::{Grid, run, some, stdin_lines};

fn main() {
    run!({
        let grid = Grid::from_input(stdin_lines(), some).expect("Bad input");
        let row_indices = grid.iter()
            .enumerate()
            .filter_map(|(y, line)| {
                if line.iter().all(|&c| c == '.') {
                    Some(y)
                } else {
                    None
                }
            })
            .collect::<BTreeSet<_>>();
        let col_indices = (0..grid.width())
            .filter_map(|x| {
                if grid.iter()
                    .map(|line| line[x])
                    .all(|c| c == '.') {
                    Some(x)
                } else {
                    None
                }
            })
            .collect::<BTreeSet<_>>();
        let galaxies = grid.iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(move |(x, &c)| {
                        if c == '#' {
                            Some((x, y))
                        } else {
                            None
                        }
                    })
            })
            .flatten()
            .filter_map(identity)
            .collect_vec();
        let distances = galaxies.iter()
            .enumerate()
            .map(|(i, &(x1, y1))| {
                galaxies[i + 1..galaxies.len()].iter()
                    .map(|&(x2, y2)| {
                        let distance = x1.abs_diff(x2) + y1.abs_diff(y2);
                        let col_expansion = if x1.abs_diff(x2) > 1 {
                            col_indices.range(x1.min(x2) + 1..x1.max(x2))
                                .count()
                        } else { 0 };
                        let row_expansion = if y1.abs_diff(y2) > 1 {
                            row_indices.range(y1.min(y2) + 1..y1.max(y2))
                                .count()
                        } else { 0 };
                        distance + col_expansion + row_expansion
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();
        let expansion_ratio = 1000000 - 1;
        let big_distances = galaxies.iter()
            .enumerate()
            .map(|(i, &(x1, y1))| {
                galaxies[i + 1..galaxies.len()].iter()
                    .map(|&(x2, y2)| {
                        let distance = x1.abs_diff(x2) + y1.abs_diff(y2);
                        let col_expansion = if x1.abs_diff(x2) > 1 {
                            col_indices.range(x1.min(x2) + 1..x1.max(x2))
                                .count() * expansion_ratio
                        } else { 0 };
                        let row_expansion = if y1.abs_diff(y2) > 1 {
                            row_indices.range(y1.min(y2) + 1..y1.max(y2))
                                .count() * expansion_ratio
                        } else { 0 };
                        distance + col_expansion + row_expansion
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();
        (distances, big_distances)
    })
}