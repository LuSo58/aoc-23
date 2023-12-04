use std::collections::HashSet;
use std::io::stdin;
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
                                let numbers_won = winning_numbers
                                    .split(' ')
                                    .filter_map(|number| {
                                        number.parse::<u32>().ok()
                                    })
                                    .filter(|number| {
                                        betting_numbers.contains(number)
                                    }).count();
                                match numbers_won {
                                    0 => 0,
                                    1.. => 1 << (numbers_won - 1),
                                    _ => panic!("Impossible: negative usize")
                                }
                            }
                            None => panic!("Bad input: pipe")
                        }
                    }
                    None => panic!("Bad input: semicolon")
                }
            }).expect("Failed reading from input")
        }).sum::<u32>();
    let time = start.elapsed();
    println!("{sum}");
    println!("{}ns", time.as_nanos());
}