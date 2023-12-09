use std::io::stdin;
use std::iter::zip;
use aoc23::run;

fn main() {
    run!({
        let lines: [String; 2] = stdin().lines().collect::<Result<Vec<_>, _>>().expect("Bad input").try_into().expect("Bad input");
        let times = lines[0]
            .split_whitespace()
            .filter(|x| !x.is_empty())
            .skip(1)
            .map(str::trim)
            .map(str::parse::<u32>)
            .collect::<Result<Vec<_>, _>>()
            .expect("Bad input");
        let distances = lines[1]
            .split_whitespace()
            .filter(|x| !x.is_empty())
            .skip(1)
            .map(str::trim)
            .map(str::parse::<u32>)
            .collect::<Result<Vec<_>, _>>()
            .expect("Bad input");
        let prod = zip(times.into_iter(), distances.into_iter())
            .map(|(t, l)| {
                let d = ((t * t - 4 * l) as f64).sqrt();
                let p1 = ((t as f64 - d) / 2.0).floor() as u32;
                let p2 = ((t as f64 + d) / 2.0).floor() as u32;
                p2 - p1 - if d.fract() == 0.0 { 1 } else { 0 }
            })
            .product::<u32>();
        let t = lines[0]
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<u64>()
            .expect("Bad input");
        let l = lines[1]
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<u64>()
            .expect("Bad input");
        let d = ((t * t - 4 * l) as f64).sqrt();
        let p1 = ((t as f64 - d) / 2.0).floor() as u64;
        let p2 = ((t as f64 + d) / 2.0).floor() as u64;
        let options = p2 - p1 - if d.fract() == 0.0 { 1 } else { 0 };
        (prod, options)
    });
}