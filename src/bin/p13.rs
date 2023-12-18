use std::io::stdin;
use std::iter::zip;
use itertools::Itertools;
use aoc23::{Grid, run, some};

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
                    Some(Grid::from_input(group, some))
                }
            })
            .collect::<Option<Vec<_>>>()
            .expect("Bad input");
        let sum_perfect = grids.iter()
            .map(|grid| {
                if let Some(row_axis) = (1..grid.height()).find(|&y| {
                    zip((0..y).rev(), y..grid.height())
                        .all(|(y1, y2)| {
                            grid.row(y1) == grid.row(y2)
                        })
                }) {
                    row_axis * 100
                } else if let Some(col_axis) = (1..grid.width()).find(|&x| {
                    zip((0..x).rev(), x..grid.width())
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
                if let Some(row_axis) = (1..grid.height()).find(|&y| {
                    zip((0..y).rev(), y..grid.height())
                        .map(|(y1, y2)| {
                            zip(grid.row(y1), grid.row(y2)).filter(|(&lhs, &rhs)| lhs != rhs).count()
                        })
                        .sum::<usize>() == 1
                }) {
                    row_axis * 100
                } else if let Some(col_axis) = (1..grid.width()).find(|&x| {
                    zip((0..x).rev(), x..grid.width())
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