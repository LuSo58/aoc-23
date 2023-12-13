use std::cell::RefCell;
use std::collections::HashMap;
use std::io::stdin;
use std::vec;
use itertools::Itertools;
use aoc23::run;
use rayon::prelude::*;

type CachePerSymbol = HashMap<(Option<usize>, Box<[CellState]>, Box<[usize]>), usize>;

#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
enum CellState {
    Working,
    Broken,
    Unknown,
}

impl TryFrom<char> for CellState {
    type Error = char;

    fn try_from(value: char) -> Result<CellState, Self::Error> {
        match value {
            '.' => Ok(CellState::Working),
            '#' => Ok(CellState::Broken),
            '?' => Ok(CellState::Unknown),
            _ => Err(value),
        }
    }
}

fn cache_adaptor(previous_run: Option<usize>, cells: &[CellState], runs: &[usize],
                 cache: &mut CachePerSymbol) -> usize {
    if let Some(result) = cache.get(&(previous_run, Box::from(cells), Box::from(runs))) {
        *result
    } else {
        let result = count_arrangements(previous_run, cells, runs, cache);
        cache.insert((previous_run, Box::from(cells), Box::from(runs)), result);
        result
    }
}

fn count_arrangements_current(current: CellState,
                              previous_run: Option<usize>, cells: &[CellState], runs: &[usize],
                              cache: &mut CachePerSymbol) -> usize {
    let next_run = runs[0];
    match (current, previous_run) {
        (CellState::Working, Some(previous_run)) => {
            if previous_run == next_run {
                cache_adaptor(None, cells, &runs[1..], cache)
            } else {
                0
            }
        }
        (CellState::Working, None) => {
            cache_adaptor(None, cells, runs, cache)
        }
        (CellState::Broken, Some(previous_run)) => {
            if next_run <= previous_run {
                0
            } else {
                cache_adaptor(Some(previous_run + 1), cells, runs, cache)
            }
        }
        (CellState::Broken, None) => {
            cache_adaptor(Some(1), cells, runs, cache)
        }
        (CellState::Unknown, _) => {
            count_arrangements_current(CellState::Working, previous_run, cells, runs, cache)
                + count_arrangements_current(CellState::Broken, previous_run, cells, runs, cache)
        }
    }
}

fn count_arrangements(previous_run: Option<usize>, cells: &[CellState], runs: &[usize],
                      cache: &mut CachePerSymbol) -> usize {
    match (cells.is_empty(), runs.is_empty()) {
        (true, true) => {
            if previous_run.is_none() {
                1
            } else {
                0
            }
        }
        (true, false) => {
            previous_run.map(|previous_run| {
                if runs.len() == 1 && runs[0] == previous_run {
                    1
                } else {
                    0
                }
            }).unwrap_or(0)
        }
        (false, true) => {
            if cells.into_iter().any(|state| *state == CellState::Broken) {
                0
            } else {
                1
            }
        }
        (false, false) => {
            if runs.iter().sum::<usize>() + runs.len() - 1 - previous_run.unwrap_or_default() > cells.len() {
                0
            } else {
                count_arrangements_current(cells[0], previous_run, &cells[1..], runs, cache)
            }
        }
    }
}

fn count_arrangements_entrypoint(cells: &Vec<CellState>, runs: &Vec<usize>,
                                 cache: &mut CachePerSymbol) -> usize {
    cache_adaptor(None, cells.as_slice(), runs.as_slice(), cache)
}

fn repeat_vec_delimiter<T>(vec: &Vec<T>, delimiter: &Vec<T>, count: usize) -> Vec<T> where T: Clone {
    let mut result = Vec::with_capacity((vec.len() + 1) * count);
    for _ in 0..count {
        result.extend_from_slice(vec);
        result.extend_from_slice(delimiter);
    }
    for _ in 0..delimiter.len() {
        result.pop();
    }
    result
}

fn main() {
    run!({
        thread_local!(static CACHE: RefCell<CachePerSymbol> = Default::default());
        let springs = stdin().lines()
            .map(|line| {
                let line = line.expect("Bad input");
                let (lhs, rhs) = line.split_once(' ').expect("Bad input");
                let cells = lhs.chars()
                    .map(<char as TryInto<CellState>>::try_into)
                    .collect::<Result<Vec<_>, _>>().expect("Bad input");
                let runs = rhs.split(',')
                    .map(str::parse)
                    .collect::<Result<Vec<usize>, _>>()
                    .expect("Bad input");
                (cells, runs)
            })
            .collect_vec();
        let arrangements1 = springs.iter()
            .map(|(cells, runs)| {
                CACHE.with(|cache| {
                    count_arrangements_entrypoint(cells, runs, &mut*cache.borrow_mut())
                })
            })
            .sum::<usize>();
        let arrangements2 = springs.par_iter()
            .map(|(cells, runs)| {
                CACHE.with(|cache| {
                    count_arrangements_entrypoint(&repeat_vec_delimiter(cells, &vec![CellState::Unknown], 5),
                                                  &repeat_vec_delimiter(runs, &vec![], 5),
                                                  &mut*cache.borrow_mut())
                })
            })
            .sum::<usize>();
        (arrangements1, arrangements2)
    });
}