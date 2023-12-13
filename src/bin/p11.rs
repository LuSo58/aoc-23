use std::collections::BTreeSet;
use std::convert::identity;
use std::io::stdin;
use itertools::Itertools;

fn main() {
    let grid = stdin().lines()
        .map(|line| {
            let line = line.unwrap();
            line.chars()
                .collect_vec()
        })
        .collect_vec();
    assert!(!grid.is_empty());
    assert!(grid.iter().map(Vec::len).all_equal());
    let width = grid[0].len();
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
    let col_indices = (0..width)
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
    let expansion_ratio = 1000000 - 1;
    let distances = galaxies.iter()
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
    println!("{distances}");
}