use std::io::stdin;
use std::num::Wrapping;
use itertools::Itertools;
use smol_str::SmolStr;
use aoc23::run;

fn hash<S>(chars: &S) -> u8
    where
        S: AsRef<str>
{
    chars.as_ref().chars()
        .fold(Wrapping(0u8), |mut state, c| {
            state += Wrapping(c.try_into().expect("Infallible"));
            state *= Wrapping(17);
            state
        }).0
}

fn main() {
    run!({
        let mut line = String::new();
        stdin().read_line(&mut line).expect("Bad input");
        let commands = line.chars()
            .filter(|&c| !c.is_whitespace())
            .group_by(|&c| c == ',')
            .into_iter()
            .filter_map(|(comma, group)| {
                if comma {
                    None
                } else {
                    let chars = group.collect::<String>();
                    assert!(chars.is_ascii());
                    Some(chars)
                }
            })
            .collect_vec();
        let sum = commands.iter()
            .map(|chars| {
                hash(chars)
            })
            .map(|n| n as u64)
            .sum::<u64>();
        let focusing_power = commands.into_iter()
            .fold(vec![Vec::<(SmolStr, u8)>::new(); 256], |mut boxes, command| {
                if let Some((label, focal_length)) = command.split_once('=') {
                    let focal_length = focal_length.parse::<u8>().expect("Bad input");
                    let lenses = &mut boxes[hash(&label) as usize];
                    if let Some(position) = lenses.iter()
                        .find_position(|&(lens, _)| {
                            lens == label
                        }).map(|(position, _)| position) {
                        lenses[position].1 = focal_length;
                    } else {
                        lenses.push((SmolStr::new(label), focal_length));
                    }
                } else if let Some((label, "")) = command.split_once('-') {
                    let lenses = &mut boxes[hash(&label) as usize];
                    if let Some(position) = lenses.iter()
                        .find_position(|&(lens, _)| {
                            lens == label
                        }).map(|(position, _)| position) {
                        lenses.remove(position);
                    }
                }
                boxes
            })
            .into_iter()
            .enumerate()
            .map(|(box_id, lens_box)| {
                lens_box.into_iter()
                    .enumerate()
                    .map(move |(lens_position, (_, focal_length))| focal_length as usize * (box_id + 1) * (lens_position + 1))
            })
            .flatten()
            .sum::<usize>();
        (sum, focusing_power)
    });
}