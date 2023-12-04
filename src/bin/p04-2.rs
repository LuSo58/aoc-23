use std::collections::{HashSet, VecDeque};
use std::io::stdin;
use std::iter::repeat;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let sum = stdin()
        .lines()
        .map(|line| {
            line.map(|line| {
                match line.split_once(':') {
                    Some((_, numbers)) => {
                        match numbers.split_once('|') {
                            Some((winning_numbers, betting_numbers)) => {
                                let betting_numbers = betting_numbers
                                    .split(' ')
                                    .filter_map(|number| {
                                        number.parse::<u32>().ok()
                                    })
                                    .collect::<HashSet<_>>();
                                winning_numbers
                                    .split(' ')
                                    .filter_map(|number| {
                                        number.parse::<u32>().ok()
                                    })
                                    .filter(|number| {
                                        betting_numbers.contains(number)
                                    }).count()
                            }
                            None => panic!("Bad input: pipe")
                        }
                    }
                    None => panic!("Bad input: semicolon")
                }
            }).expect("Failed reading from input")
        })
        .fold((0, VecDeque::new()), |(total_cards, mut forward_winnings), numbers_won| {
            let cards = forward_winnings.pop_front().unwrap_or(0usize) + 1;
            forward_winnings.extend(repeat(0).take(numbers_won.saturating_sub(forward_winnings.len())));
            forward_winnings
                .iter_mut()
                .take(numbers_won)
                .for_each(|forward_duplicate| *forward_duplicate += cards);
            (total_cards + cards, forward_winnings)
        }).0;
    let time = start.elapsed();
    println!("{sum}");
    println!("{}ns", time.as_nanos());
}