use std::cmp::Ordering;
use std::str::FromStr;
use aoc23::{run, stdin_lines};

#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
struct Coord3D {
    x: f64,
    y: f64,
    z: f64,
}

impl FromStr for Coord3D {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim()
            .split(',')
            .map(str::trim)
            .map(str::parse::<i64>)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| Default::default())?;
        if parts.len() == 3 {
            Ok(Coord3D {
                x: parts[0] as f64,
                y: parts[1] as f64,
                z: parts[2] as f64,
            })
        } else {
            Err(())
        }
    }
}


#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
struct HailStone {
    position: Coord3D,
    velocity: Coord3D,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
struct CoordFloat {
    x: f64,
    y: f64,
}

impl CoordFloat {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl PartialOrd for CoordFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.x.partial_cmp(&other.x)
            .map(|x_ordering| {
                match x_ordering {
                    Ordering::Equal => self.y.partial_cmp(&other.y),
                    x_ordering => {
                        self.y.partial_cmp(&other.y)
                            .map(|y_ordering| {
                                match y_ordering {
                                    Ordering::Equal => Some(x_ordering),
                                    y_ordering => {
                                        if x_ordering == y_ordering {
                                            Some(x_ordering)
                                        } else {
                                            None
                                        }
                                    }
                                }
                            })
                            .flatten()
                    }
                }
            })
            .flatten()
    }
}

impl HailStone {
    fn intersection_flat(&self, other: &Self) -> Option<CoordFloat> {
        if self.velocity.x / other.velocity.x == self.velocity.y / other.velocity.y { // Parallel
            None
        } else {
            let xd = other.position.x - self.position.x;
            let yd = other.position.y - self.position.y;
            let a = self.velocity.y / self.velocity.x;
            let b = other.velocity.x / (other.velocity.y - other.velocity.x * a);
            let t1 = (xd * (1.0 + a * b) - yd * b) / self.velocity.x;
            let t2 = -(yd - xd * a) / (other.velocity.y - other.velocity.x * a);
            if t1 < 0.0 || t2 < 0.0 {
                None
            } else {
                Some(CoordFloat::new(
                    self.position.x + self.velocity.x * t1,
                    self.position.y + self.velocity.y * t1,
                ))
            }
        }
    }
}

impl FromStr for HailStone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (position, velocity) = s.split_once('@')
            .ok_or(Self::Err::default())?;
        Ok(HailStone {
            position: position.parse()?,
            velocity: velocity.parse()?,
        })
    }
}

fn main() {
    run!({
        let hailstones = stdin_lines()
            .map(|line| {
                line.parse::<HailStone>()
            })
            .collect::<Result<Vec<_>, _>>()
            .expect("Bad input");
        let text_area_min = 200000000000000.0;
        let test_area_max = 400000000000000.0;
        let test_area_min = CoordFloat::new(text_area_min, text_area_min);
        let test_area_max = CoordFloat::new(test_area_max, test_area_max);
        let flat_count = hailstones.iter()
            .enumerate()
            .map(|(idx, lhs)| {
                hailstones[idx + 1..hailstones.len()]
                    .iter()
                    .filter_map(|rhs| {
                        lhs.intersection_flat(rhs)
                            .map(|intersect| {
                                match (intersect.partial_cmp(&test_area_min), intersect.partial_cmp(&test_area_max)) {
                                    (Some(Ordering::Greater | Ordering::Equal), Some(Ordering::Less | Ordering::Equal)) => Some(()),
                                    _ => None,
                                }
                            })
                            .flatten()
                    })
            })
            .flatten()
            .count();
        (flat_count, 0)
    });
}




















