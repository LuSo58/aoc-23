use std::cmp::Ordering;
use std::io::stdin;
use std::str::FromStr;
use std::time::Instant;

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.strip_prefix("Game ").map(|s| {
            s.chars().take_while(|c| c.is_numeric()).collect::<String>().parse::<u32>().map(|id| {
                s.split_once(':').map(|(_, rounds)| {
                    rounds.split(';').map(Round::from_str).collect::<Result<Vec<Round>, _>>().ok().map(|rounds| {
                        Game { id, rounds }
                    })
                })
            }).ok()
        }).flatten().flatten().flatten().ok_or(Self::Err::default())
    }
}

#[derive(Default, Debug, Eq, PartialEq, Ord)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl PartialOrd<Self> for Round {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else {
            if self.red >= other.red &&
                self.green >= other.green &&
                self.blue >= other.blue {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Less)
            }
        }
    }
}

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim().split(',').map(str::trim).map(|s| {
            s.split_once(' ').map(|(count, color)| { count.parse::<u32>().ok().map(|count| (color, count)) }).flatten()
        }).fold(Some(Round::default()), |round: Option<Round>, color_pair| {
            match (round, color_pair) {
                (Some(mut round), Some((color, count))) => {
                    let matched = match color {
                        "red" => {
                            round.red = count;
                            true
                        }
                        "green" => {
                            round.green = count;
                            true
                        }
                        "blue" => {
                            round.blue = count;
                            true
                        }
                        _ => false
                    };
                    if matched {
                        Some(round)
                    } else {
                        None
                    }
                }
                _ => None
            }
        }).ok_or(Self::Err::default())
    }
}

impl Round {
    fn maximum(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn main() {
    let start = Instant::now();
    let power_sum = stdin().lines().map(|line| line.map(|line| {
        Game::from_str(line.as_str()).ok()
    }).ok().flatten()).filter_map(|game| game.map(|game| {
        game.rounds.into_iter().fold(Round::default(), |acc, round| {
            acc.maximum(&round)
        }).power()
    })).sum::<u32>();
    let time = start.elapsed();
    println!("{power_sum}");
    println!("{}ns", time.as_nanos());
}