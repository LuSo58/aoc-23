use std::convert::Infallible;
use std::fmt::Debug;
use std::io::{Read, stdin};
use std::str::FromStr;
use itertools::Itertools;
use aoc23::run;
use crate::HandType::{FiveOfKind, FourOfKind, FullHouse, HighCard, OnePair, ThreeOfKind, TwoPair};

trait Card {}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
enum Card1 {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
enum Card2 {
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

impl Card for Card1 {}
impl Card for Card2 {}

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
struct Hand<T> where T: Card {
    hand_type: HandType,
    cards: [T; 5],
    bet: u64,
}

impl TryFrom<char> for Card1 {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Card1::Two),
            '3' => Ok(Card1::Three),
            '4' => Ok(Card1::Four),
            '5' => Ok(Card1::Five),
            '6' => Ok(Card1::Six),
            '7' => Ok(Card1::Seven),
            '8' => Ok(Card1::Eight),
            '9' => Ok(Card1::Nine),
            'T' => Ok(Card1::Ten),
            'J' => Ok(Card1::Jack),
            'Q' => Ok(Card1::Queen),
            'K' => Ok(Card1::King),
            'A' => Ok(Card1::Ace),
            _ => Err(value),
        }
    }
}

impl TryFrom<char> for Card2 {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'J' => Ok(Card2::Joker),
            '2' => Ok(Card2::Two),
            '3' => Ok(Card2::Three),
            '4' => Ok(Card2::Four),
            '5' => Ok(Card2::Five),
            '6' => Ok(Card2::Six),
            '7' => Ok(Card2::Seven),
            '8' => Ok(Card2::Eight),
            '9' => Ok(Card2::Nine),
            'T' => Ok(Card2::Ten),
            'Q' => Ok(Card2::Queen),
            'K' => Ok(Card2::King),
            'A' => Ok(Card2::Ace),
            _ => Err(value),
        }
    }
}

impl TryFrom<&[Card1; 5]> for HandType {
    type Error = Infallible;

    fn try_from(value: &[Card1; 5]) -> Result<Self, Self::Error> {
        let counts = value.iter().cloned().counts();
        match counts.len() {
            1 => Ok(FiveOfKind),
            2 => {
                if counts.values().any(|&x| x == 4) {
                    Ok(FourOfKind)
                } else {
                    Ok(FullHouse)
                }
            }
            3 => {
                if counts.values().any(|&x| x == 3) {
                    Ok(ThreeOfKind)
                } else {
                    Ok(TwoPair)
                }
            }
            4 => Ok(OnePair),
            5 => Ok(HighCard),
            _ => panic!("Infallible"),
        }
    }
}

impl TryFrom<&[Card2; 5]> for HandType {
    type Error = Infallible;

    fn try_from(value: &[Card2; 5]) -> Result<Self, Self::Error> {
        let counts = value.iter().cloned().counts();
        let jokers = value.iter().filter(|&&card| card == Card2::Joker).count();
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

impl<T> FromStr for Hand<T>
    where
        T: Card + TryFrom<char>,
        for<'a> HandType: TryFrom<&'a [T; 5], Error = Infallible>, {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            Some((cards, bet)) => {
                let cards: [T; 5] = cards
                    .chars()
                    .map(T::try_from)
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|_| "Invalid card character")?
                    .try_into()
                    .map_err(|_| "Bad number of cards")?;
                let bet = bet.parse().map_err(|_| "Bad input")?;
                let hand_type = (&cards).try_into().unwrap();
                Ok(Hand {
                    hand_type,
                    cards,
                    bet,
                })
            }
            None => Err("No space")
        }
    }
}

fn main() {
    run!({
        let mut lines = String::new();
        stdin().read_to_string(&mut lines).expect("Bad input");
        let winnings1 = lines.lines()
            .map(|line| line.parse::<Hand<Card1>>().expect("Bad input"))
            .sorted()
            .zip(1..)
            .map(|(hand, rank)| {
                rank * hand.bet
            }).sum::<u64>();
        let winnings2 = lines.lines()
            .map(|line| line.parse::<Hand<Card2>>().expect("Bad input"))
            .sorted()
            .zip(1..)
            .map(|(hand, rank)| {
                rank * hand.bet
            }).sum::<u64>();
        (winnings1, winnings2)
    });
}