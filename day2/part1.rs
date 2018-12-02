use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let mut hash_count = HashMap::new();

    include_str!("day2.txt")
        .lines()
        .map(|s| {
          let mut letter_count = HashSet::new();
          for c in s.chars() {
            letter_count.insert(s.matches(c).count()); 
          }
          return letter_count; 
        })
        .for_each(|mut l| {
          for i in l.drain() {
              let count = hash_count.entry(i).or_insert(0);
              *count += 1;
          }
        });

    println!("calculate final value by hand -> {:?}", hash_count);
}