use std::io::stdin;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut line = String::new();
    stdin().read_line(&mut line).expect("Bad input");
    let t = line
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<u64>()
        .expect("Bad input");
    line.clear();
    stdin().read_line(&mut line).expect("Bad input");
    let l = line
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<u64>()
        .expect("Bad input");
    let d = ((t * t - 4 * l) as f64).sqrt();
    let p1 = ((t as f64 - d) / 2.0).floor() as u64;
    let p2 = ((t as f64 + d) / 2.0).floor() as u64;
    let options = p2 - p1 - if d.fract() == 0.0 { 1 } else { 0 };
    let time = start.elapsed();
    println!("{options}");
    println!("{}ns", time.as_nanos());
}