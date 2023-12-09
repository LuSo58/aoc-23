#[macro_export]
macro_rules! run {
    ($x:tt) => {
        {
            let start = std::time::Instant::now();
            let (r1, r2) = $x;
            let time = start.elapsed().as_secs_f64() * 1000.0;
            println!("Part 1: {r1}\nPart 2: {r2}\nTime: {time}ms");
        }
    };
}