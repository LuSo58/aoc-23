use std::collections::{HashMap, VecDeque};
use std::mem;
use std::str::FromStr;
use itertools::Itertools;
use smol_str::SmolStr;
use aoc23::stdin_lines;
use crate::ModuleType::{Broadcaster, Conjunction, FlipFlop};

#[derive(Clone, Debug)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction(HashMap<SmolStr, bool>),
    Broadcaster,
}

#[derive(Clone, Debug)]
struct Module {
    name: SmolStr,
    module_type: ModuleType,
    targets: Vec<SmolStr>,
}

impl FromStr for Module {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, targets) = s.split_once("->")
            .ok_or(Self::Err::default())?;
        let name = name.trim();
        let (module_type, name) = if let Some(name) = name.strip_prefix("%") {
            (FlipFlop(false), name)
        } else if let Some(name) = name.strip_prefix("&") {
            (Conjunction(HashMap::new()), name)
        } else if name == "broadcaster" {
            (Broadcaster, name)
        } else {
            Err(Self::Err::default())?
        };
        let name = name.into();
        let targets = targets.split(',')
            .map(str::trim)
            .map(<&str as Into<SmolStr>>::into)
            .collect_vec();
        Ok(Self {
            name,
            module_type,
            targets,
        })
    }
}

fn main() {
    let broadcaster_name = SmolStr::new("broadcaster");
    let modules = stdin_lines()
        .map(|line| line.parse::<Module>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Bad input");
    let mut modules = modules.into_iter()
        .map(|module| {
            (module.name.clone(), module)
        })
        .collect::<HashMap<_, _>>();
    let modules_ref = modules.clone();
    for (name, module) in &mut modules {
        match &mut module.module_type {
            Conjunction(ref mut inputs) => {
                *inputs = modules_ref.iter()
                    .filter_map(|(input_name, Module { targets, .. })| {
                        if targets.contains(name) {
                            Some((input_name.clone(), false))
                        } else {
                            None
                        }
                    })
                    .collect();
            }
            _ => {}
        }
    }
    drop(modules_ref);
    let mut pulses = VecDeque::new();
    let mut next_pulses = VecDeque::new();
    let mut high_pulse_count = 0u64;
    let mut low_pulse_count = 0u64;
    for _ in 0..1000 {
        next_pulses.push_back((broadcaster_name.clone(), broadcaster_name.clone(), false));
        while !next_pulses.is_empty() {
            mem::swap(&mut pulses, &mut next_pulses);
            while let Some((pulse_source, pulse_target, pulse_high)) = pulses.pop_front() {
                if pulse_high {
                    high_pulse_count += 1;
                } else {
                    low_pulse_count += 1;
                }
                if let Some(target) = modules.get_mut(&pulse_target) {
                    match &mut target.module_type {
                        FlipFlop(internal_state) => {
                            if !pulse_high {
                                *internal_state = !*internal_state;
                                for target in &target.targets {
                                    next_pulses.push_back((pulse_target.clone(), target.clone(), *internal_state));
                                }
                            }
                        }
                        Conjunction(inputs) => {
                            *inputs.get_mut(&pulse_source).expect("Bad input") = pulse_high;
                            let next_pulse_high = !inputs.iter().all(|(_, &high)| high);
                            for target in &target.targets {
                                next_pulses.push_back((pulse_target.clone(), target.clone(), next_pulse_high));
                            }
                        }
                        Broadcaster => {
                            for target in &target.targets {
                                next_pulses.push_back((pulse_target.clone(), target.clone(), pulse_high));
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{high_pulse_count} {low_pulse_count} {}", high_pulse_count * low_pulse_count);
}