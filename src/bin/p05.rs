use std::collections::BTreeSet;
use std::io::{Read, stdin};
use std::ops::{Range, RangeBounds};
use itertools::Itertools;
use rangemap::{RangeMap, RangeSet};
use aoc23::run;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct RangeMapping {
    src: u64,
    dst: u64,
    len: u64,
}

impl RangeMapping {
    fn new(src: u64, dst: u64, len: u64) -> Self {
        Self { src, dst, len }
    }

    fn search_only(src: u64) -> Self {
        Self { src, dst: 0, len: 0 }
    }

    fn search_range(start: u64, end: u64) -> impl RangeBounds<RangeMapping> {
        Self::search_only(start)..=Self::search_only(end)
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
    run!({
        let mut input = String::new();
        stdin().read_to_string(&mut input).expect("Bad input");
        let mut segments = input.split("\n\n");
        let seeds = segments
            .next()
            .expect("Bad input")
            .split(' ')
            .skip(1)
            .map(|s| s.parse::<u64>())
            .collect::<Result<Vec<_>, _>>().expect("Bad input");
        let seed_ranges = seeds.iter()
            .cloned()
            .tuples()
            .map(|(start, len)| {
                start..start + len
            })
            .collect::<RangeSet<_>>();
        let (s1, s2) = segments.tee();
        let closest1 = s1
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
                        RangeMapping::new(src, dst, len)
                    }).collect::<BTreeSet<_>>()
            })
            .fold(seeds, |idxs, range_set| {
                idxs.into_iter()
                    .map(|idx| {
                        range_set
                            .range(RangeMapping::search_range(0, idx))
                            .next_back()
                            .map(|mapping| {
                                mapping.map(idx)
                            })
                            .flatten()
                            .unwrap_or(idx)
                    })
                    .collect()
            })
            .into_iter()
            .min()
            .expect("Infallible");
        let closest2 = s2
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
            .fold(seed_ranges, |ranges, range_set| {
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
        (closest1, closest2)
    });
}