use std::collections::BTreeSet;
use std::io::{Read, stdin};
use std::ops::RangeBounds;
use std::time::Instant;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
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
        .map(|s| s.parse::<u64>())
        .collect::<Result<Vec<_>, _>>().expect("Bad input");
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
    let time = start.elapsed();
    println!("{closest}");
    println!("{}ns", time.as_nanos());
}