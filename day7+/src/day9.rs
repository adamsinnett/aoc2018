use std::collections::{ HashMap, VecDeque };

#[aoc_generator(day9)]
pub fn input_generator(_input: &str) -> Vec<(usize, usize)> {
  [(424, 71144)].to_vec()
}

#[aoc(day9, part1)]
fn part1(input: &Vec<(usize, usize)>) -> usize {
  let nplayers = input[0].0;
  let total_marbles = input[0].1;
  let mut game = Vec::<usize>::new();
  let mut scores = HashMap::<usize, usize>::new();
  game.push(0);
  for current_marble in 1..=total_marbles {
    if current_marble % 23 == 0 {
      game.rotate_right(7);
      *scores.entry(current_marble % nplayers).or_insert(0) += current_marble;
      *scores.entry(current_marble % nplayers).or_insert(0) += game.pop().unwrap_or(0);
      game.rotate_left(1);
    } else {
      game.rotate_left(1);
      game.push(current_marble);
    }
  }

  *scores.iter().map(|(_k,v)| v).max().unwrap_or(&0)
}

#[aoc(day9, part2)]
fn part2(input: &Vec<(usize, usize)>) -> usize {
  let nplayers = input[0].0;
  let total_marbles = input[0].1 * 100;
  let mut game = VecDeque::<usize>::new();
  let mut scores = HashMap::<usize, usize>::new();
  game.push_back(0);
  for current_marble in 1..=total_marbles {
    if current_marble % 23 == 0 {
      rotate_right(&mut game, 7);
      *scores.entry(current_marble % nplayers).or_insert(0) += current_marble;
      *scores.entry(current_marble % nplayers).or_insert(0) += game.pop_back().unwrap_or(0);
      rotate_left(&mut game, 1);
    } else {
      rotate_left(&mut game, 1);
      game.push_back(current_marble);
    }
  }

  *scores.iter().map(|(_k,v)| v).max().unwrap_or(&0)
}

fn rotate_right(game: &mut VecDeque<usize>, k:usize) {
  for _ in 0..k {
    let back = game.pop_back().unwrap();
    game.push_front(back);
  }
}

fn rotate_left(game: &mut VecDeque<usize>, l:usize) {
  for _ in 0..l {
    let back = game.pop_front().unwrap();
    game.push_back(back);
  }
}