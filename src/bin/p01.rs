use std::cell::OnceCell;
use std::collections::HashMap;
use std::io::stdin;
use std::time::Instant;
use regex::Regex;

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
    let start = Instant::now();
    let map = [("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)].into_iter().collect::<HashMap<_, _>>();
    let re_first = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|[1-9]").expect("infallible");
    let re_last = Regex::new(r"(.*)(one|two|three|four|five|six|seven|eight|nine|[1-9])").expect("infallible");

    stdin().lines().map(|line| line.map(|line| {
        let first = re_first.find(line.as_ref()).map(|m| parse(m.as_str()));
        let last = re_last.captures(line.as_ref()).map(|c| c.get(2).map(|m| parse(m.as_str()))).flatten();

        match (first, last) {
            (Some(first), Some(last)) => Some(first * 10 + last),
            _ => None,
        }
    }).ok().flatten()).sum::<Option<u32>>().map(|result| println!("{result}"));
    let time = start.elapsed();
    println!("{}ns", time.as_nanos());
}
