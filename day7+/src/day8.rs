use std::collections::{HashMap, VecDeque};

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<usize> {
  input.split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<usize>>()
}

#[aoc(day8, part1)]
fn part1(input: &Vec<usize>) -> usize {
  let mut data = input.iter().map(|b| *b).collect::<VecDeque<usize>>();
  walk(&mut data).0
}

fn walk(input: &mut VecDeque<usize>) -> (usize, usize) {
  let children = input.pop_front().unwrap_or(0);
  let metadata = input.pop_front().unwrap_or(0);


  let mut totals = 0;
  let mut values = HashMap::new();
  for n in 0..children {
    let (total, value) = walk(input);
    totals += total;
    values.entry(n).or_insert(value);
  }

  let mut meta_entry = Vec::new();
  for _ in 0..metadata {
    meta_entry.push(input.pop_front().unwrap_or(0));
  }
      
  let meta_total = meta_entry.iter().fold(0, |a,t| a+t);

  return if children == 0 {
    (totals + meta_total, meta_total)
  } else {
    let mut value = 0;
    for n in meta_entry.iter() {
      value += values.get(&(n-1)).unwrap_or(&0);
    }
    (totals + meta_total, value)
  }
}

#[aoc(day8, part2)]
fn part2(input: &Vec<usize>) -> usize {
  let mut data = input.iter().map(|b| *b).collect::<VecDeque<usize>>();
  walk(&mut data).1
}