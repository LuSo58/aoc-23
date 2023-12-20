use num::range_step_inclusive;
use aoc23::{Grid, Coord, run, some, stdin_lines, xy};

fn calculate_load(grid: &Grid<char>) -> usize {
    grid.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .map(move |&c| {
                    if c == 'O' {
                        grid.height() - y
                    } else {
                        0
                    }
                })
        })
        .flatten()
        .sum::<usize>()
}

fn tilt_vertical(grid: &mut Grid<char>, north: bool) {
    let ys = if north {
        range_step_inclusive(grid.height() as isize - 1, 0, -1)
    } else {
        range_step_inclusive(0, grid.height() as isize - 1, 1)
    };
    for x in 0..grid.width() {
        let mut count = 0usize;
        for y in ys.clone() {
            let y = y as usize;
            match grid[xy!(x, y)] {
                '#' => {
                    for i in 0..count {
                        let y = if north {
                            y + i + 1
                        } else {
                            y - (i + 1)
                        };
                        grid[xy!(x, y)] = 'O';
                    }
                    count = 0;
                }
                'O' => {
                    grid[xy!(x, y)] = '.';
                    count += 1;
                }
                _ => {}
            }
        }
        if count > 0 {
            let ys = if north {
                range_step_inclusive(0, count - 1, 1)
            } else {
                range_step_inclusive(grid.height() - count, grid.height() - 1, 1)
            };
            for i in ys {
                grid[xy!(x, i)] = 'O';
            }
        }
    }
}

fn tilt_horizontal(grid: &mut Grid<char>, west: bool) {
    let xs = if west {
        range_step_inclusive(grid.width() as isize - 1, 0, -1)
    } else {
        range_step_inclusive(0, grid.width() as isize - 1, 1)
    };
    for y in 0..grid.height() {
        let mut count = 0usize;
        for x in xs.clone() {
            let x = x as usize;
            match grid[xy!(x, y)] {
                '#' => {
                    for i in 0..count {
                        let x = if west {
                            x + i + 1
                        } else {
                            x - (i + 1)
                        };
                        grid[xy!(x, y)] = 'O';
                    }
                    count = 0;
                }
                'O' => {
                    grid[xy!(x, y)] = '.';
                    count += 1;
                }
                _ => {}
            }
        }
        if count > 0 {
            let xs = if west {
                range_step_inclusive(0, count - 1, 1)
            } else {
                range_step_inclusive(grid.width() - count, grid.width() - 1, 1)
            };
            for i in xs {
                grid[xy!(i, y)] = 'O';
            }
        }
    }
}

fn main() {
    run!({
        let grid = Grid::from_input(stdin_lines(), some).expect("Bad input");
        let mut tilted_grid = grid.clone();
        tilt_vertical(&mut tilted_grid, true);
        let load = calculate_load(&tilted_grid);
        let mut rotating_grid = grid;
        let mut rotating_grid_past: Vec<Grid<char>> = vec![];
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