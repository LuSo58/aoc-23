use std::io::stdin;
use std::iter::zip;
use itertools::Itertools;
use aoc23::run;

fn main() {
    run!({
        let grids = stdin().lines()
            .map(|line| line.expect("Bad input"))
            .group_by(String::is_empty)
            .into_iter()
            .filter_map(|(empty, group)| {
                if empty {
                    None
                } else {
                    let grid = group.map(|line| line.chars().collect_vec()).collect_vec();
                    assert!(!grid.is_empty());
                    assert!(grid.iter().map(Vec::len).all_equal());
                    Some(grid)
                }
            })
            .collect_vec();
        let sum_perfect = grids.iter()
            .map(|grid| {
                let width = grid[0].len();
                if let Some(row_axis) = (1..grid.len()).find(|&y| {
                    zip((0..y).rev(), y..grid.len())
                        .all(|(y1, y2)| {
                            grid[y1] == grid[y2]
                        })
                }) {
                    row_axis * 100
                } else if let Some(col_axis) = (1..width).find(|&x| {
                    zip((0..x).rev(), x..width)
                        .all(|(x1, x2)| {
                            grid.iter()
                                .all(|line| {
                                    line[x1] == line[x2]
                                })
                        })
                }) {
                    col_axis
                } else {
                    panic!("Not mirrored at all")
                }
            })
            .sum::<usize>();
        let sum_smudge = grids.into_iter()
            .map(|grid| {
                let width = grid[0].len();
                if let Some(row_axis) = (1..grid.len()).find(|&y| {
                    zip((0..y).rev(), y..grid.len())
                        .map(|(y1, y2)| {
                            zip(&grid[y1], &grid[y2]).filter(|(&lhs, &rhs)| lhs != rhs).count()
                        })
                        .sum::<usize>() == 1
                }) {
                    row_axis * 100
                } else if let Some(col_axis) = (1..width).find(|&x| {
                    zip((0..x).rev(), x..width)
                        .map(|(x1, x2)| {
                            grid.iter()
                                .map(|line| {
                                    if line[x1] == line[x2] { 0 } else { 1 }
                                })
                                .sum::<usize>()
                        })
                        .sum::<usize>() == 1
                }) {
                    col_axis
                } else {
                    panic!("Not mirrored at all")
                }
            })
            .sum::<usize>();
        (sum_perfect, sum_smudge)
    });
}