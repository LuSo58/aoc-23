use std::collections::HashMap;
use std::convert::Infallible;
use std::io::stdin;
use std::iter::Sum;
use std::ops::{Range, Sub};
use std::str::FromStr;
use itertools::Itertools;
use rangemap::RangeSet;
use aoc23::run;
use crate::Target::*;

#[derive(Clone, Eq, PartialEq, Debug)]
enum Target {
    Accept,
    Reject,
    Redirect(String),
}

impl FromStr for Target {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Accept,
            "R" => Reject,
            s => Redirect(s.to_string()),
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Variable {
    Extreme,
    Musical,
    Aerodynamic,
    Shiny,
}

impl FromStr for Variable {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Variable::Extreme),
            "m" => Ok(Variable::Musical),
            "a" => Ok(Variable::Aerodynamic),
            "s" => Ok(Variable::Shiny),
            _ => Err(Self::Err::default())
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Bound {
    less_then: bool,
    bound: u64,
    variable: Variable,
    target: Target,
}

impl Bound {
    fn check_part(&self, part: &Part) -> bool {
        if self.less_then {
            part.select(self.variable) < self.bound
        } else {
            part.select(self.variable) > self.bound
        }
    }

    fn opposite(mut self) -> Self {
        self.less_then = !self.less_then;
        if self.less_then {
            self.bound += 1;
        } else {
            self.bound -= 1;
        }
        self
    }
}

impl FromStr for Bound {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const BRACKETS: [char; 2] = ['<', '>'];
        let (rest, target) = s.split_once(':')
            .ok_or(Self::Err::default())?;
        let angle_position = rest.find(BRACKETS)
            .ok_or(Self::Err::default())?;
        let less_then = &rest[angle_position..angle_position + 1] == "<";
        let (variable, bound) = rest.split_once(BRACKETS)
            .ok_or(Self::Err::default())?;
        let variable = variable.parse()?;
        let bound = bound.parse()
            .map_err(|_| Self::Err::default())?;
        let target = target.parse()
            .map_err(|_| Self::Err::default())?;
        Ok(Bound {
            less_then,
            bound,
            variable,
            target,
        })
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Workflow {
    name: String,
    bounds: Vec<Bound>,
    default: Target,
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once('{') {
            Some((name, rest)) => {
                match rest.strip_suffix('}') {
                    Some(workflow) => {
                        let mut bounds = workflow.split(',')
                            .collect_vec();
                        let default = bounds.pop()
                            .ok_or(())?
                            .parse()
                            .map_err(|_| ())?;
                        let bounds = bounds.into_iter()
                            .map(str::parse::<Bound>)
                            .collect::<Result<Vec<_>, _>>()?;
                        Ok(Workflow {
                            name: name.to_string(),
                            bounds,
                            default,
                        })
                    }
                    _ => Err(Self::Err::default())
                }
            }
            _ => Err(Self::Err::default())
        }
    }
}

struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn select(&self, variable: Variable) -> u64 {
        match variable {
            Variable::Extreme => self.x,
            Variable::Musical => self.m,
            Variable::Aerodynamic => self.a,
            Variable::Shiny => self.s,
        }
    }
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix('{')
            .ok_or(Self::Err::default())?;
        let variables = s.strip_suffix('}')
            .ok_or(Self::Err::default())?
            .split(',')
            .map(|segment| {
                segment.split_once('=')
                    .map(|(variable, value)| {
                        value.parse::<u64>()
                            .map(|value| {
                                (variable, value)
                            })
                            .ok()
                    })
            })
            .flatten()
            .try_fold((None, None, None, None), |(x, m, a, s), variable| {
                variable.map(|(variable, value)| {
                    match variable {
                        "x" => match x {
                            None => Some((Some(value), m, a, s)),
                            Some(_) => None,
                        }
                        "m" => match m {
                            None => Some((x, Some(value), a, s)),
                            Some(_) => None,
                        }
                        "a" => match a {
                            None => Some((x, m, Some(value), s)),
                            Some(_) => None,
                        }
                        "s" => match s {
                            None => Some((x, m, a, Some(value))),
                            Some(_) => None,
                        }
                        _ => None
                    }
                }).flatten()
            })
            .ok_or(Self::Err::default())?;
        match variables {
            (Some(x), Some(m), Some(a), Some(s)) => Ok(Part { x, m, a, s }),
            _ => Err(Self::Err::default())
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Configurations {
    x: RangeSet<u64>,
    m: RangeSet<u64>,
    a: RangeSet<u64>,
    s: RangeSet<u64>,
}

fn range_len<T>(range_set: &RangeSet<T>) -> T
    where T: Ord + Clone + Copy + Sub + Sum<<T>::Output> {
    range_set.iter()
        .map(|x| {
            x.end - x.start
        })
        .sum::<T>()
}

fn range_intersection<T>(lhs: &RangeSet<T>, rhs: &Range<T>) -> RangeSet<T>
    where T: Ord + Clone + Copy {
    lhs.overlapping(rhs)
        .filter_map(|range| {
            let range = range.start.max(rhs.start)..range.end.min(rhs.end);
            if range.is_empty() {
                None
            } else {
                Some(range)
            }
        })
        .collect()
}

impl Configurations {
    fn count_possibilities(self, target: &Target, workflows: &HashMap<String, Workflow>) -> u64 {
        if self.is_empty() {
            0
        } else {
            match target {
                Accept => range_len(&self.x) * range_len(&self.m) * range_len(&self.a) * range_len(&self.s),
                Reject => 0,
                Redirect(workflow_name) => {
                    let workflow = workflows.get(workflow_name)
                        .expect("Bad input");
                    let (count, configuration) = workflow.bounds
                        .iter()
                        .cloned()
                        .fold((0, self), |(mut count, configuration), bound| {
                            count += configuration.intersection(bound.clone())
                                .count_possibilities(&bound.target, workflows);
                            (count, configuration.intersection(bound.opposite()))
                        });
                    count + configuration.count_possibilities(&workflow.default, workflows)
                }
            }
        }
    }
    fn is_empty(&self) -> bool {
        self.x.is_empty() && self.m.is_empty() && self.a.is_empty() && self.s.is_empty()
    }
    fn intersection(&self, bound: Bound) -> Self {
        let mut config = self.clone();
        let range = if bound.less_then {
            u64::MIN..bound.bound
        } else {
            bound.bound + 1..u64::MAX
        };
        let variable = match bound.variable {
            Variable::Extreme => &mut config.x,
            Variable::Musical => &mut config.m,
            Variable::Aerodynamic => &mut config.a,
            Variable::Shiny => &mut config.s,
        };
        *variable = range_intersection(variable, &range);
        config
    }
}

fn main() {
    run!({
        let segments = stdin().lines()
            .map(|line| line.expect("Bad input"))
            .group_by(String::is_empty);
        let mut segments = segments.into_iter()
            .filter_map(|(empty, group)| {
                if empty {
                    None
                } else {
                    Some(group.into_iter())
                }
            });
        let workflows = segments.next()
            .expect("Bad input")
            .map(|line| line.parse::<Workflow>().expect("Bad input"))
            .map(|workflow| {
                (workflow.name.clone(), workflow)
            })
            .collect::<HashMap<_, _>>();
        let parts = segments.next()
            .expect("Bad input")
            .map(|line| line.parse::<Part>())
            .collect::<Result<Vec<_>, _>>()
            .expect("Bad input");
        let sum = parts.into_iter()
            .filter_map(|part| {
                let mut current_target = Redirect("in".to_string());
                while let Redirect(workflow_name) = current_target {
                    let workflow = workflows.get(&workflow_name)
                        .expect("Bad input");
                    if let Some(target) = workflow.bounds
                        .iter()
                        .filter_map(|bound| {
                            if bound.check_part(&part) {
                                Some(bound.target.clone())
                            } else {
                                None
                            }
                        })
                        .next() {
                        current_target = target;
                    } else {
                        current_target = workflow.default.clone();
                    }
                }
                if current_target == Accept {
                    Some(part.x + part.m + part.a + part.s)
                } else {
                    None
                }
            })
            .sum::<u64>();
        let mut initial_range = RangeSet::new();
        initial_range.insert(1..4001);
        let possibilities = Configurations {
            x: initial_range.clone(),
            m: initial_range.clone(),
            a: initial_range.clone(),
            s: initial_range.clone(),
        }
            .count_possibilities(&Redirect("in".to_string()), &workflows);
        (sum, possibilities)
    })
}