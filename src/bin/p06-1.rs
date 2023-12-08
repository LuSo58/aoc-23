use std::io::stdin;
use std::iter::zip;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut line = String::new();
    stdin().read_line(&mut line).expect("Bad input");
    let times = line
        .split_whitespace()
        .filter(|x| !x.is_empty())
        .skip(1)
        .map(str::trim)
        .map(str::parse::<u32>)
        .collect::<Result<Vec<_>, _>>()
        .expect("Bad input");
    line.clear();
    stdin().read_line(&mut line).expect("Bad input");
    let distances = line
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
    let time = start.elapsed();
    println!("{prod}");
    println!("{}ns", time.as_nanos());
}