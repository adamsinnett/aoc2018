#[macro_use] extern crate itertools;

use itertools::Itertools;

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    part1(include_str!("day6.txt"));
    part2(include_str!("day6.txt"));
}

fn part1(input: &str) {
    let coords = input
        .lines()
        .map(|line| {
            let coords = line.split(",").map(|s| s.trim()).map(|s| s.parse().unwrap()).collect::<Vec<_>>();
            (coords[0], coords[1])
        })
        .collect::<Vec<(i64, i64)>>();

    let x_max: i64 = coords.iter().map(|c| c.0).max().unwrap()+100;
    let x_min: i64 = coords.iter().map(|c| c.0).min().unwrap()-100;
    let y_max: i64 = coords.iter().map(|c| c.1).max().unwrap()+100;
    let y_min: i64= coords.iter().map(|c| c.1).min().unwrap()-100;

    let mut dist_map = HashMap::new();

    for x in x_min..=x_max {
        for y in y_min..=y_max {
            let mut distance_to_current = std::i64::MAX;
            for coord in coords.iter() {
                let current_distance = (coord.0 - x).abs() + (coord.1 - y).abs();
                //println!("{:?}: coord {:?} is {} current smallest {}",(x,y) ,coord, current_distance, distance_to_current);
                if current_distance < distance_to_current {
                    distance_to_current = current_distance;
                    dist_map.insert((x, y), Some(coord));
                } else if current_distance == distance_to_current {
                    dist_map.insert((x, y), None);
                }
            }
        }
    };

    let mut edges = HashSet::<(i64, i64)>::new();

    for ((x, y), b) in dist_map.iter() {
        if b.is_some() && (*x == x_min || *x == x_max || *y == y_min || *y == y_max) {
            edges.insert(*b.unwrap());
        }
    }

    let totals = dist_map
        .iter()
        .filter(|(_a,b)| b.is_some())
        .map(|(a, b)| (a, b.unwrap()))
        .filter(|(_a, b)| !edges.contains(*b))
        .fold(HashMap::<(i64, i64), usize>::new(), |mut accum, (_a,b)| {
            *accum.entry(*b).or_insert(0) +=1;
            accum
        });

    let mut answers = totals
        .iter()
        .map(|(a, b)| b)
        .collect::<Vec<_>>();

    answers.sort();

    println!("{:?}", answers);
}

fn part2(input: &str) {
    let coords = input
        .lines()
        .map(|line| {
            let coords = line.split(",").map(|s| s.trim()).map(|s| s.parse().unwrap()).collect::<Vec<_>>();
            (coords[0], coords[1])
        })
        .collect::<Vec<(i32, i32)>>();

    let x_max: i32 = coords.iter().map(|c| c.0).max().unwrap();
    let x_min: i32 = coords.iter().map(|c| c.0).min().unwrap();
    let y_max: i32 = coords.iter().map(|c| c.1).max().unwrap();
    let y_min: i32= coords.iter().map(|c| c.1).min().unwrap();
    
    let answer = iproduct!(x_min..=x_max, y_min..=y_max)
        .filter(|&(x, y)| {
            coords
                .iter()
                .map(|&(cx, cy)| (x - cx).abs() + (y - cy).abs())
                .sum::<i32>()
                < 10000
        })
        .count();

    println!("{}", answer);
}