use std::io::stdin;
use itertools::Itertools;

fn main() {
    let sum = stdin().lines()
        .map(|line| {
            let line = line.unwrap();
            let history = line.split(' ')
                .map(str::parse::<i64>)
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            let last_values = (0..)
                .scan(history, |history, _| {
                    let last = if history.iter().any(|&x| x != 0) {
                        Some(*history.last().unwrap())
                    } else {
                        None
                    };
                    *history = history.windows(2)
                        .map(|window| {
                            window[1] - window[0]
                        })
                        .collect::<Vec<_>>();
                    last
                })
                .collect_vec();
            last_values.into_iter().rev()
                .fold(0, |prediction, history| {
                    prediction + history
                })
        })
        .sum::<i64>();
    println!("{sum}");
}