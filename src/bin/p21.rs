use std::collections::{HashMap, VecDeque};
use itertools::Itertools;
use aoc23::{Coord, Direction, Grid, run, some, stdin_lines, xy};

fn bfs(grid: &Grid<char>, start: Coord, start_steps: usize, step_limit: usize) -> HashMap<Coord, usize> {
    if start_steps <= step_limit {
        let mut next_cells = VecDeque::new();
        let mut visited = HashMap::new();
        next_cells.push_back((start, start_steps));
        visited.insert(start, start_steps);
        while let Some((cell, steps)) = next_cells.pop_front() {
            if steps < step_limit {
                Direction::ALL.iter()
                    .for_each(|&direction| {
                        cell.next(direction)
                            .map(|next_cell| {
                                if next_cell.x < grid.width() && next_cell.y < grid.height() && grid[next_cell] != '#' {
                                    if !visited.contains_key(&next_cell) {
                                        next_cells.push_back((next_cell, steps + 1));
                                        visited.insert(next_cell, steps + 1);
                                    }
                                }
                            });
                    });
            }
        }
        visited
    } else {
        Default::default()
    }
}

fn count_cells(visited: &HashMap<Coord, usize>, step_limit: usize) -> usize {
    visited.into_iter()
        .filter(|(_, &steps)| steps % 2 == step_limit % 2 && steps <= step_limit)
        .count()
}

fn main() {
    run!({
        let mut grid = Grid::from_input(stdin_lines(), some)
            .expect("Bad input");
        let start = grid.iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .find_map(|(x, c)| {
                        if *c == 'S' {
                            Some(xy!(x, y))
                        } else {
                            None
                        }
                    })
            })
            .expect("Bad input");
        grid[start] = '.';
        let step_limit_1 = 64;
        let visited = bfs(&grid, start, 0, step_limit_1);
        let cell_count = count_cells(&visited, step_limit_1);
        let step_limit_2 = 26501365;
        let multiplier = 5;
        let mut grid_big = Grid::new(grid.width() * multiplier, grid.height() * multiplier);
        grid.iter()
            .enumerate()
            .for_each(|(y, row)| {
                row.iter()
                    .enumerate()
                    .for_each(|(x, &c)| {
                        (0..multiplier).cartesian_product(0..multiplier)
                            .for_each(|(x_mul, y_mul)| {
                                grid_big[xy!(x + grid.width() * x_mul, y + grid.height() * y_mul)] = c;
                            })
                    })
            });
        let start_big = xy!(grid_big.width() / 2, grid_big.height() / 2);
        let visited = bfs(&grid_big, start_big, 0, start.x + grid.width() * 2);
        let visited_0 = count_cells(&visited, start.x) as isize;
        let visited_1 = count_cells(&visited, start.x + grid.width()) as isize;
        let visited_2 = count_cells(&visited, start.x + grid.width() * 2) as isize;
        let count_filled_maps = ((step_limit_2 - grid.width() / 2) / grid.width()) as isize;
        let det_a_0 = -visited_0 + 2 * visited_1 - visited_2;
        let det_a_1 = 3 * visited_0 - 4 * visited_1 + visited_2;
        let det_a_2 = -2 * visited_0;
        let x_0 = det_a_0 / -2;
        let x_1 = det_a_1 / -2;
        let x_2 = det_a_2 / -2;
        let cell_count_big = x_0 * count_filled_maps * count_filled_maps + x_1 * count_filled_maps + x_2;
        (cell_count, cell_count_big)
    })
}