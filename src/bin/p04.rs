use std::collections::{HashSet, VecDeque};
use std::io::stdin;
use std::iter::repeat;
use aoc23::run;

fn main() {
    run!({
        let winning_counts = stdin()
            .lines()
            .map(|line| {
                let line = line.unwrap();
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
            }).collect::<Vec<_>>();
        let points = winning_counts.iter()
            .map(|winnings| {
                match winnings {
                    0 => 0,
                    1.. => 1 << (winnings - 1),
                    _ => panic!("Impossible: negative usize")
                }
            }).sum::<usize>();
        let cards = winning_counts.into_iter()
            .fold((0, VecDeque::new()), |(total_cards, mut forward_winnings), numbers_won| {
                let cards = forward_winnings.pop_front().unwrap_or(0usize) + 1;
                forward_winnings.extend(repeat(0).take(numbers_won.saturating_sub(forward_winnings.len())));
                forward_winnings
                    .iter_mut()
                    .take(numbers_won)
                    .for_each(|forward_duplicate| *forward_duplicate += cards);
                (total_cards + cards, forward_winnings)
            }).0;
        (points, cards)
    })
}