extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;


fn main() {
    part1();
    part2();
}

fn part1() {
    let mut grid = vec![vec![0; 1000]; 1000];

    let re = Regex::new(r"#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)").unwrap();

    for line in include_str!("day3.txt").lines() {
      let cap = re.captures(line).unwrap();
      let id = cap[1].parse::<usize>().unwrap();
      let x = cap[2].parse::<usize>().unwrap();
      let y = cap[3].parse::<usize>().unwrap();
      let width = &cap[4].parse::<usize>().unwrap();
      let height = &cap[5].parse::<usize>().unwrap();
      for row in &mut grid[x..x+width] {
            for square in &mut row[y..y + height] {
                *square += 1;

            }
        }
    }

    let count = grid.iter().fold(0, |memo, row| {
        memo + row.iter().filter(|&&val| val > 1).count()
    });

    println!("part1: {}", count)
}

fn part2() {
    let mut grid = HashMap::new();
    
    let mut all = HashSet::new();
    let mut overlap = HashSet::new();

    let re = Regex::new(r"#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)").unwrap();

    for line in include_str!("day3.txt").lines() {
      let cap = re.captures(line).unwrap();
      let id = cap[1].parse::<usize>().unwrap();
      let x = cap[2].parse::<usize>().unwrap();
      let y = cap[3].parse::<usize>().unwrap();
      let width = &cap[4].parse::<usize>().unwrap();
      let height = &cap[5].parse::<usize>().unwrap();
      for row in &mut (x..x+width) {
            for col in &mut (y..y + height) {
                let key = (row, col);
                all.insert(id);
                if grid.contains_key(&key)  {
                    overlap.insert(grid[&key]);
                    overlap.insert(id);
                } else {
                    grid.insert((row, col), id);
                }
            }
        }
    }

    let good_ids = all.difference(&overlap).collect::<Vec<_>>();

    println!("part2: {:?}", good_ids)
}
