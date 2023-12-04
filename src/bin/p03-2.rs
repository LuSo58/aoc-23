use std::cmp::Ordering;
use std::io::stdin;
use std::time::Instant;
use itertools::Itertools;
use once_cell::unsync::Lazy;

fn main() {
    let start = Instant::now();
    let input = stdin().lines().map(|line| line.map(String::into_bytes)).collect::<Result<Vec<_>, _>>().expect("Bad input - Error while reading");
    let width = input.first().map(Vec::len).expect("Bad input - Empty input");
    assert!(input.iter().all(|line| line.len() == width));
    let height = input.len();
    let sum = input.iter().enumerate().map(|(row, line)| {
        line.into_iter().enumerate().map(move |(col, c)| (row, col, c))
    }).flatten().group_by(|&(row, _, c)| {
        if c.is_ascii_digit() {
            Some(row)
        } else {
            None
        }
    }).into_iter().filter_map(|(is_digit, rhs)| {
        is_digit.map(|_| rhs)
    }).map(|group| {
        let group = group.collect::<Vec<_>>();
        let first = *group.first().expect("Infallible");
        let last = *group.last().expect("Infallible");
        let n = Lazy::new(move || group.iter().map(|(_, _, &c)| c as char).collect::<String>().parse::<u32>().expect("Infallible"));
        let start = (first.0.saturating_sub(1), first.1.saturating_sub(1));
        let end = (last.0 + 1, last.1 + 1);
        (start.0..=end.0).cartesian_product(start.1..=end.1).filter_map({
            let input = &input;
            move |(row, col)| {
                if row < height && col < width && !(row == first.0 && col >= first.1 && col <= last.1) {
                    Some((row, col, input[row][col]))
                } else {
                    None
                }
            }
        }).filter_map(move |(row, col, c)| {
            if c == '*' as u8 {
                Some(((row, col), *n))
            } else {
                None
            }
        })
    }).flatten().sorted_by(|lhs, rhs| {
        match lhs.0.0.cmp(&rhs.0.0) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => lhs.0.1.cmp(&rhs.0.1),
            Ordering::Greater => Ordering::Greater,
        }
    }).group_by(|&(index, _)| index).into_iter().map(|(_, group)| {
        group.fold(Some((0, 1)), |acc, (_, n)| {
            acc.map(|(count, product)| {
                match count {
                    0 | 1 => {
                        Some((count + 1, product * n))
                    }
                    _ => None
                }
            }).flatten()
        })
    }).filter_map(|gear| {
        gear.map(|(count, product)| {
            if count == 2 {
                Some(product)
            } else {
                None
            }
        }).flatten()
    }).sum::<u32>();
    let time = start.elapsed();
    println!("{sum}");
    println!("{}ns", time.as_nanos());
}