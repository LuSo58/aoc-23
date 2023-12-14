use std::io::stdin;
use itertools::Itertools;
use num::range_step_inclusive;
use aoc23::run;

fn calculate_load(grid: &Vec<Vec<char>>) -> usize {
    let height = grid.len();
    grid.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .map(move |&c| {
                    if c == 'O' {
                        height - y
                    } else {
                        0
                    }
                })
        })
        .flatten()
        .sum::<usize>()
}

fn tilt_vertical(grid: &mut Vec<Vec<char>>, north: bool) {
    let height = grid.len();
    let width = grid[0].len();
    let ys = if north {
        range_step_inclusive(height as isize - 1, 0, -1)
    } else {
        range_step_inclusive(0, height as isize - 1, 1)
    };
    for x in 0..width {
        let mut count = 0usize;
        for y in ys.clone() {
            let y = y as usize;
            match grid[y][x] {
                '#' => {
                    for i in 0..count {
                        let y = if north {
                            y + i + 1
                        } else {
                            y - (i + 1)
                        };
                        grid[y][x] = 'O';
                    }
                    count = 0;
                }
                'O' => {
                    grid[y][x] = '.';
                    count += 1;
                }
                _ => {}
            }
        }
        if count > 0 {
            let ys = if north {
                range_step_inclusive(0, count - 1, 1)
            } else {
                range_step_inclusive(height - count, height - 1, 1)
            };
            for i in ys {
                grid[i][x] = 'O';
            }
        }
    }
}

fn tilt_horizontal(grid: &mut Vec<Vec<char>>, west: bool) {
    let height = grid.len();
    let width = grid[0].len();
    let xs = if west {
        range_step_inclusive(width as isize - 1, 0, -1)
    } else {
        range_step_inclusive(0, width as isize - 1, 1)
    };
    for y in 0..height {
        let mut count = 0usize;
        for x in xs.clone() {
            let x = x as usize;
            match grid[y][x] {
                '#' => {
                    for i in 0..count {
                        let x = if west {
                            x + i + 1
                        } else {
                            x - (i + 1)
                        };
                        grid[y][x] = 'O';
                    }
                    count = 0;
                }
                'O' => {
                    grid[y][x] = '.';
                    count += 1;
                }
                _ => {}
            }
        }
        if count > 0 {
            let xs = if west {
                range_step_inclusive(0, count - 1, 1)
            } else {
                range_step_inclusive(width - count, width - 1, 1)
            };
            for i in xs {
                grid[y][i] = 'O';
            }
        }
    }
}

#[allow(unused)]
fn print_grid(grid: &Vec<Vec<char>>) {
    grid.iter().for_each(|line| {
        line.iter().for_each(|c| print!("{c}"));
        println!()
    });
    println!()
}

fn main() {
    run!({
        let grid = stdin().lines()
            .map(|line| {
                let line = line.expect("Bad input");
                line.chars()
                    .collect_vec()
            })
            .collect_vec();
        assert!(!grid.is_empty());
        assert!(!grid[0].is_empty());
        assert!(grid.iter().map(Vec::len).all_equal());
        let mut tilted_grid = grid.clone();
        tilt_vertical(&mut tilted_grid, true);
        let load = calculate_load(&tilted_grid);
        let mut rotating_grid = grid;
        let mut rotating_grid_past: Vec<Vec<Vec<char>>> = vec![];
        let limit = 1000000000usize;
        for i in 0..limit {
            tilt_vertical(&mut rotating_grid, true);
            tilt_horizontal(&mut rotating_grid, true);
            tilt_vertical(&mut rotating_grid, false);
            tilt_horizontal(&mut rotating_grid, false);
            if let Some((past_i, _)) = rotating_grid_past.iter()
                .enumerate()
                .rev()
                .find(|(_, past_grid)| {
                    **past_grid == rotating_grid
                }) {
                let repetition_len = i - past_i;
                let remaining_rounds = limit - i;
                rotating_grid = rotating_grid_past[past_i + remaining_rounds % repetition_len - 1].clone();
                break;
            } else {
                rotating_grid_past.push(rotating_grid.clone());
            }
        }
        (load, calculate_load(&rotating_grid))
    })
}