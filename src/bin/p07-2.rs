use std::io::stdin;
use std::str::FromStr;
use std::time::Instant;
use itertools::Itertools;
use crate::Card::*;
use crate::HandType::{FiveOfKind, FourOfKind, FullHouse, HighCard, OnePair, ThreeOfKind, TwoPair};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
    bet: u64,
}

impl TryFrom<char> for Card {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Two),
            '3' => Ok(Three),
            '4' => Ok(Four),
            '5' => Ok(Five),
            '6' => Ok(Six),
            '7' => Ok(Seven),
            '8' => Ok(Eight),
            '9' => Ok(Nine),
            'T' => Ok(Ten),
            'J' => Ok(Joker),
            'Q' => Ok(Queen),
            'K' => Ok(King),
            'A' => Ok(Ace),
            _ => Err(value),
        }
    }
}

impl TryFrom<&[Card; 5]> for HandType {
    type Error = ();

    fn try_from(value: &[Card; 5]) -> Result<Self, Self::Error> {
        let counts = value.iter().cloned().counts();
        let jokers = value.iter().filter(|&&card| card == Joker).count();
        match (counts.len(), jokers) {
            (1, _) => Ok(FiveOfKind),
            (2, 0) => {
                if counts.values().any(|&x| x == 4) {
                    Ok(FourOfKind)
                } else {
                    Ok(FullHouse)
                }
            }
            (2, _) => Ok(FiveOfKind),
            (3, 0) => {
                if counts.values().any(|&x| x == 3) {
                    Ok(ThreeOfKind)
                } else {
                    Ok(TwoPair)
                }
            }
            (3, 2 | 3) => Ok(FourOfKind),
            (3, 1) => {
                if counts.values().any(|&x| x == 3) {
                    Ok(FourOfKind)
                } else {
                    Ok(FullHouse)
                }
            }
            (4, 0) => Ok(OnePair),
            (4, 1 | 2) => Ok(ThreeOfKind),
            (5, 0) => Ok(HighCard),
            (5, 1) => Ok(OnePair),
            _ => panic!("Infallible"),
        }
    }
}

impl FromStr for Hand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            Some((cards, bet)) => {
                let cards: [Card; 5] = cards
                    .chars()
                    .map(Card::try_from)
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|_| "Invalid card character")?
                    .try_into()
                    .map_err(|_| "Bad number of cards")?;
                let bet = bet.parse().map_err(|_| "Bad input")?;
                Ok(Hand {
                    hand_type: (&cards).try_into().map_err(|_| "Infallible")?,
                    cards,
                    bet,
                })
            }
            None => Err("No space")
        }
    }
}

fn main() {
    let start = Instant::now();
    let winnings = stdin()
        .lines()
        .map(|line| {
            line.map(|line| {
                line.parse::<Hand>().expect("Bad input")
            }).expect("Reading failed")
        })
        .sorted()
        .zip(1..)
        .map(|(hand, rank)| {
            rank * hand.bet
        }).sum::<u64>();
    let time = start.elapsed();
    println!("{winnings}");
    println!("{}ns", time.as_nanos());
}