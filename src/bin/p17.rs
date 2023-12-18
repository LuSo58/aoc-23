use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::convert::identity;
use itertools::Itertools;
use aoc23::{Coord, Direction, Grid, run, stdin_lines};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord)]
struct AStarState {
    coord: Coord,
    direction: Direction,
    continuous_steps: u8,
    cost: u32,
    distance: usize,
}

impl PartialOrd for AStarState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cost.cmp(&other.cost).then_with(|| self.distance.cmp(&other.distance)))
    }
}

impl AStarState {
    fn new(coord: Coord, direction: Direction, continuous_steps: u8, cost: u32, distance: usize) -> Self {
        Self { coord, direction, continuous_steps, cost, distance }
    }

    fn next(&self, grid: &Grid<u32>, target: Coord) -> [Option<Self>; 3] {
        let sides = self.direction.orthogonal();
        self.next_helper(grid, target, [
            if self.continuous_steps < 3 { Some(self.direction) } else { None },
            Some(sides.0),
            Some(sides.1)
        ])
    }

    fn next_ultra(&self, grid: &Grid<u32>, target: Coord) -> [Option<Self>; 3] {
        let sides = self.direction.orthogonal();
        self.next_helper(grid, target, [
            if self.continuous_steps < 10 { Some(self.direction) } else { None },
            if self.continuous_steps >= 4 { Some(sides.0) } else { None },
            if self.continuous_steps >= 4 { Some(sides.1) } else { None },
        ])
    }

    fn next_helper(&self, grid: &Grid<u32>, target: Coord, possible_directions: [Option<Direction>; 3]) -> [Option<Self>; 3] {
        possible_directions.into_iter()
            .map(|direction| {
                direction.map(|direction| {
                    self.coord.next(direction)
                        .map(|coord| {
                            if coord.x < grid.width() && coord.y < grid.height() {
                                let continuous_steps = if self.direction == direction {
                                    self.continuous_steps
                                } else {
                                    0
                                };
                                Some(AStarState::new(coord, direction, continuous_steps + 1, self.cost + grid[coord], coord - target))
                            } else {
                                None
                            }
                        })
                })
                    .flatten()
                    .flatten()
            })
            .collect_vec()
            .try_into()
            .expect("Infallible")
    }
}

fn run_astar<F>(grid: &Grid<u32>, start: Coord, end: Coord, next_fn: F) -> u32
    where
        F: Fn(&AStarState, &Grid<u32>, Coord) -> [Option<AStarState>; 3]
{
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(AStarState::new(start, Direction::East, 0, 0, start - end)));
    let mut cost = 0;
    let mut visited = HashSet::new();
    while let Some(Reverse(state)) = queue.pop() {
        if state.coord == end {
            cost = state.cost;
            break;
        }
        if !visited.contains(&(state.coord, state.direction, state.continuous_steps)) {
            visited.insert((state.coord, state.direction, state.continuous_steps));
            next_fn(&state, &grid, end)
                .into_iter()
                .filter_map(identity)
                .for_each(|state| queue.push(Reverse(state)));
        }
    }
    cost
}

fn main() {
    run!({
        let grid = Grid::from_input(stdin_lines(), |c| c.to_digit(10))
            .expect("Bad input");
        let start = Coord::new(0, 0);
        let end = Coord::new(grid.width() - 1, grid.height() - 1);
        let simple_cost = run_astar(&grid, start, end, AStarState::next);
        let ultra_cost = run_astar(&grid, start, end, AStarState::next_ultra);
        (simple_cost, ultra_cost)
    })
}