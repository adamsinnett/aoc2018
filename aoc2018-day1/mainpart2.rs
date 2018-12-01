#![feature(cell_update)]

const INPUT: &str = include_str!("day1.txt");
use std::cell::Cell;
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();

    let frequency = Cell::new(0);

    INPUT
        .lines()
        .flat_map(|s| s.parse::<isize>().ok())
        .cycle()
        .take_while(|_| set.insert(frequency.get()))
        .for_each(|n| {
            frequency.update(|old| old + n);
        });

    println!("{:?}", frequency);
}