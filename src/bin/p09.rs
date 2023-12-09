use std::io::stdin;
use itertools::Itertools;
use ::aoc23::run;

fn main() {
    run!({
        let results = stdin().lines()
            .map(|line| {
                let line = line.unwrap();
                let history = line.split(' ')
                    .map(str::parse::<i64>)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();
                let end_values = (0..)
                    .scan(history, |history, _| {
                        let ret = if history.iter().any(|&x| x != 0) {
                            Some((*history.first().unwrap(), *history.last().unwrap()))
                        } else {
                            None
                        };
                        *history = history.windows(2)
                            .map(|window| {
                                window[1] - window[0]
                            })
                            .collect::<Vec<_>>();
                        ret
                    })
                    .collect_vec();
                end_values.into_iter().rev()
                    .fold((0, 0), |prediction, history| {
                        (history.0 - prediction.0, history.1 + prediction.1)
                    })
            })
            .reduce(|lhs, rhs| {
                (lhs.0 + rhs.0, lhs.1 + rhs.1)
            }).unwrap();
        (results.1, results.0)
    });
}