use lazy_static::lazy_static;
use regex::Regex;
use chrono::prelude::*;
use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut actions = include_str!("day4.txt")
        .lines()
        .map(GuardAction::from_str)
        .map(Result::unwrap)
        .collect::<Vec<GuardAction>>();

    actions.sort();

    let mut current_guard = 0;
    let mut falls_asleep = 0;
    let mut guard_by_minutes = vec![];
    let mut total_guard_minutes = HashMap::new();

    for action in actions {
        if action.guard != 0 {
            current_guard = action.guard;
        }

        if action.action == 2 {
            falls_asleep = action.minute;
        }

        if action.action == 3 {
            let minutes = action.minute - falls_asleep;
            *total_guard_minutes.entry(current_guard).or_insert(0) += minutes;
            for n in falls_asleep..action.minute {
                guard_by_minutes.push((current_guard, n));
            }
        }
    }

    let mut it = total_guard_minutes.iter_mut().collect::<Vec<_>>();
    it.sort_by(|(_a, b), (_x, y)| b.cmp(y) );

    let sleepy_guard = it.pop().unwrap().0;

    let mut hmap = guard_by_minutes
        .into_iter()
        .filter(|(a,_)| a == sleepy_guard)
        .fold(HashMap::new(), |mut acc, (_, b)| {
            *acc.entry(b).or_insert(0) += 1;
            acc
        });
    

    let mut minutes_vec = hmap.iter_mut().collect::<Vec<_>>();

    minutes_vec.sort_by(|(_a, b), (_x, y)| b.cmp(y));

    let best_minute = minutes_vec.pop().unwrap().0;

    println!("Part1: Sleepy guard is {:?} at minute {}", sleepy_guard, best_minute);
    println!("Part1: Answer is {}", sleepy_guard *best_minute);
}

fn part2() {
    let mut actions = include_str!("day4.txt")
        .lines()
        .map(GuardAction::from_str)
        .map(Result::unwrap)
        .collect::<Vec<GuardAction>>();

    actions.sort();

    let mut current_guard = 0;
    let mut falls_asleep = 0;
    let mut guard_minute = HashMap::new();


    for action in actions {
        if action.guard != 0 {
            current_guard = action.guard;
        }

        if action.action == 2 {
            falls_asleep = action.minute;
        }

        if action.action == 3 {
            for n in falls_asleep..action.minute {
                *guard_minute.entry((current_guard, n)).or_insert(0) += 1;
            }
        }
    }

    let mut minutes_vec = guard_minute.iter_mut().collect::<Vec<_>>();

    minutes_vec.sort_by(|(_a, b), (_x, y)| b.cmp(y));

    let sleepy_guard_minute = minutes_vec.pop().unwrap().0;

    println!("Part2: Sleepy guard is {:?} at minute {}", sleepy_guard_minute.0, sleepy_guard_minute.1);
    println!("Part2: Answer is {}", sleepy_guard_minute.0 * sleepy_guard_minute.1);
}

// This should go somewhere else?
#[derive(Debug, Clone, Copy, Eq)]
pub struct GuardAction {
    pub guard: usize,
    pub time: i64,
    pub minute: usize,
    pub action: usize, // 1 = start, 2 = sleep, 3 = wake
}

impl Ord for GuardAction {
    fn cmp(&self, other: &GuardAction) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for GuardAction {
    fn partial_cmp(&self, other: &GuardAction) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for GuardAction {
    fn eq(&self, other: &GuardAction) -> bool {
        self.time == other.time
    }
}

impl FromStr for GuardAction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        lazy_static! {
            static ref ACTION_REGEX: Regex = Regex::new(r"\[([\d\-\s:]+)\]([\w\s\d#]+)").unwrap();
            static ref MINUTE_REGEX: Regex = Regex::new(r"\d+\-\d+-\d+\s\d+:(\d+)").unwrap();
            static ref GUARD_REGEX: Regex = Regex::new(r"Guard #(\d+)").unwrap();
        }
        let captures = ACTION_REGEX.captures(s).unwrap();
        let time = Utc.datetime_from_str(&captures[1], "%Y-%m-%d %H:%M").unwrap().timestamp();
        let minute = MINUTE_REGEX.captures(&captures[1]).unwrap()[1].parse().unwrap();
        let guard = match GUARD_REGEX.captures(&captures[2]) {
            Some(guard) => guard[1].parse::<usize>().unwrap(),
            None => 0,
        };

        let action_int = if captures[2].contains("begins shift") {
            1
        } else if captures[2].contains("falls asleep") {
            2
        } else {
            3
        };
 
        Ok(GuardAction {
            guard: guard,
            time: time,
            minute: minute,
            action: action_int,
        })
    }
}