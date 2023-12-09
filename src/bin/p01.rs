use std::cell::OnceCell;
use std::collections::HashMap;
use std::io::stdin;
use regex::Regex;
use aoc23::run;

fn parse<S>(v: S) -> u32 where S: AsRef<str> {
    static mut MAP: OnceCell<HashMap<&str, u32>> = OnceCell::new();
    let v = v.as_ref();
    if v.len() == 1 {
        v.chars().next().expect("Infallible").to_digit(10).expect("Infallible")
    } else {
        let map = unsafe { MAP.get_or_init(|| [("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)].into_iter().collect::<HashMap<_, _>>()) };
        *map.get(v).expect("Infallible")
    }
}

fn main() {
    run!({
        let re_first = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|[1-9]").expect("infallible");
        let re_last = Regex::new(r"(.*)(one|two|three|four|five|six|seven|eight|nine|[1-9])").expect("infallible");
        let lines = stdin().lines().collect::<Result<Vec<_>, _>>().unwrap();
        let num_sum = lines.iter().map(|line| {
            let first = line.chars()
                .find_map(|c| c.to_digit(10))
                .unwrap();
            let last = line.chars()
                .rev()
                .find_map(|c| c.to_digit(10))
                .unwrap();
            first * 10 + last
        }).sum::<u32>();
        let all_sum = lines.into_iter().map(|line| {
            let first = re_first
                .find(line.as_ref())
                .map(|m| parse(m.as_str()))
                .unwrap();
            let last = re_last
                .captures(line.as_ref())
                .map(|c| c.get(2).map(|m| parse(m.as_str())))
                .flatten()
                .unwrap();
            first * 10 + last
        }).sum::<u32>();
        (num_sum, all_sum)
    });
}
