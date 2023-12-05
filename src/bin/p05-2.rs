use std::io::{Read, stdin};
use std::ops::Range;
use std::time::Instant;
use itertools::Itertools;
use rangemap::{RangeMap, RangeSet};

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct RangeMapping {
    src: u64,
    dst: u64,
    len: u64,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct SimpleRange {
    start: u64,
    end: u64,
}

impl RangeMapping {
    fn new(src: u64, dst: u64, len: u64) -> Self {
        Self { src, dst, len }
    }

    fn map(&self, n: u64) -> Option<u64> {
        if (self.src..self.src + self.len).contains(&n) {
            Some(n + self.dst - self.src)
        } else {
            None
        }
    }

    fn map_range(&self, range: Range<u64>) -> Option<Range<u64>> {
        Some(self.map(range.start)?..self.map(range.end - 1)? + 1)
    }
}

fn union<T>(lhs: &Range<T>, rhs: &Range<T>) -> Range<T> where T: Ord + Copy {
    lhs.start.max(rhs.start)..lhs.end.min(rhs.end)
}

fn main() {
    let start = Instant::now();
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect("Bad input");
    let mut segments = input.split("\n\n");
    let seeds = segments
        .next()
        .expect("Bad input")
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<u64>().expect("Bad input"))
        .tuples()
        .map(|(start, len)| {
            start..start + len
        })
        .collect::<RangeSet<_>>();
    let closest = segments
        .map(|segment| {
            segment
                .lines()
                .skip(1)
                .map(|line| {
                    let mut nums = line.split(' ').map(|s| s.parse::<u64>().expect("Bad input"));
                    let dst = nums.next().expect("Bad input");
                    let src = nums.next().expect("Bad input");
                    let len = nums.next().expect("Bad input");
                    if !nums.next().is_none() {
                        panic!("Bad input")
                    }
                    (src..src + len, RangeMapping::new(src, dst, len))
                }).collect::<RangeMap<_, _>>()
        })
        .fold(seeds, |ranges, range_set| {
            ranges
                .into_iter()
                .map(|range| {
                    range_set
                        .overlapping(&range)
                        .map(|(mapping_range, mapping)| {
                            mapping.map_range(union(&range, &mapping_range)).expect("Infallible")
                        })
                        .chain(
                            range_set
                                .gaps(&range)
                        ).collect::<RangeSet<_>>()
                })
                .flatten()
                .collect()
        })
        .into_iter()
        .next()
        .expect("Infallible")
        .start;
    let time = start.elapsed();
    println!("{closest}");
    println!("{}ns", time.as_nanos());
}