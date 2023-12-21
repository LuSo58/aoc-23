use std::collections::{HashMap, VecDeque};
use aoc23::{Coord, Direction, Grid, some, stdin_lines, xy};

fn bfs_memoize(grid: &Grid<char>, start: Coord, start_steps: usize, step_limit: usize) -> &HashMap<Coord, usize> {
    static mut CACHE: HashMap<(Grid<char>, Coord, usize, usize), HashMap<Coord, usize>>
}

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
        .filter(|(_, steps)| *steps % 2 == step_limit % 2)
        .count()
}

fn main() {
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
    let cell_count = visited.into_iter()
        .filter(|(_, steps)| *steps % 2 == step_limit_1 % 2)
        .count();
    println!("{cell_count}");
    let step_limit_2 = 26501365;
    let visited_all = bfs(&grid, start, 0, step_limit_2);
    let x = grid.width() - 1;
    let y = grid.height() - 1;
    let size = grid.width();
    let axis_start_steps = step_limit_2.saturating_sub(size);
    let diag_start_small = step_limit_2.saturating_sub(size / 2 + 1);
    let diag_start_big = step_limit_2.saturating_sub(size + size / 2);
    let visited_left = bfs(&grid, xy!(x, start.y), axis_start_steps, step_limit_2);
    let visited_right = bfs(&grid, xy!(0, start.y), axis_start_steps, step_limit_2);
    let visited_top = bfs(&grid, xy!(start.x, y), axis_start_steps, step_limit_2);
    let visited_down = bfs(&grid, xy!(start.x, 0), axis_start_steps, step_limit_2);
    let visited_top_left_small = bfs(&grid, xy!(x, y), diag_start_small, step_limit_2);
    let visited_top_left_big = bfs(&grid, xy!(x, y), diag_start_big, step_limit_2);
    let visited_top_right_small = bfs(&grid, xy!(0, y), diag_start_small, step_limit_2);
    let visited_top_right_big = bfs(&grid, xy!(0, y), diag_start_big, step_limit_2);
    let visited_bottom_right_small = bfs(&grid, xy!(0, 0), diag_start_small, step_limit_2);
    let visited_bottom_right_big = bfs(&grid, xy!(0, 0), diag_start_big, step_limit_2);
    let visited_bottom_left_small = bfs(&grid, xy!(x, 0), diag_start_small, step_limit_2);
    let visited_bottom_left_big = bfs(&grid, xy!(x, 0), diag_start_big, step_limit_2);
    let cell_count_big = if step_limit_2 <= start.x {
        count_cells(&visited_all, step_limit_2)
    } else if step_limit_2 <= size {
        count_cells(&visited_all, step_limit_2) +
            count_cells(&visited_left, step_limit_2) +
            count_cells(&visited_right, step_limit_2) +
            count_cells(&visited_top, step_limit_2) +
            count_cells(&visited_down, step_limit_2)
    } else {
        let filled_maps_left = (step_limit_2 - start.x) / grid.width();
        let filled_maps = (filled_maps_left * 2 - 1).pow(2) / 2 + 1;
        println!("filled: {} {} = {}", count_cells(&visited_all, step_limit_2), filled_maps, count_cells(&visited_all, step_limit_2) * filled_maps);
        println!("small-tl: {}", count_cells(&visited_top_left_small, step_limit_2));
        println!("small-tr: {}", count_cells(&visited_top_right_small, step_limit_2));
        println!("small-bl: {}", count_cells(&visited_bottom_right_small, step_limit_2));
        println!("small-br: {}", count_cells(&visited_bottom_left_small, step_limit_2));
        println!("small: {} = {}", filled_maps_left, (count_cells(&visited_top_left_small, step_limit_2) +
            count_cells(&visited_top_right_small, step_limit_2) +
            count_cells(&visited_bottom_right_small, step_limit_2) +
            count_cells(&visited_bottom_left_small, step_limit_2)) * filled_maps_left);
        println!("big-tl: {}", count_cells(&visited_top_left_big, step_limit_2));
        println!("big-tr: {}", count_cells(&visited_top_right_big, step_limit_2));
        println!("big-bl: {}", count_cells(&visited_bottom_right_big, step_limit_2));
        println!("big-br: {}", count_cells(&visited_bottom_left_big, step_limit_2));
        println!("big: {} = {}", filled_maps_left.saturating_sub(1), (count_cells(&visited_top_left_big, step_limit_2) +
            count_cells(&visited_top_right_big, step_limit_2) +
            count_cells(&visited_bottom_right_big, step_limit_2) +
            count_cells(&visited_bottom_left_big, step_limit_2)) * filled_maps_left.saturating_sub(1));
        println!("left: {}", count_cells(&visited_left, step_limit_2));
        println!("right: {}", count_cells(&visited_right, step_limit_2));
        println!("top: {}", count_cells(&visited_top, step_limit_2));
        println!("down: {}", count_cells(&visited_down, step_limit_2));
        count_cells(&visited_all, step_limit_2) * filled_maps +
            (count_cells(&visited_top_left_small, step_limit_2) +
                count_cells(&visited_top_right_small, step_limit_2) +
                count_cells(&visited_bottom_right_small, step_limit_2) +
                count_cells(&visited_bottom_left_small, step_limit_2)) * filled_maps_left +
            (count_cells(&visited_top_left_big, step_limit_2) +
                count_cells(&visited_top_right_big, step_limit_2) +
                count_cells(&visited_bottom_right_big, step_limit_2) +
                count_cells(&visited_bottom_left_big, step_limit_2)) * filled_maps_left.saturating_sub(1) +
            count_cells(&visited_left, step_limit_2) +
            count_cells(&visited_right, step_limit_2) +
            count_cells(&visited_top, step_limit_2) +
            count_cells(&visited_down, step_limit_2)
    };
    println!("{cell_count_big}");
}