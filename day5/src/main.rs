fn main() {
//     let example = "dabAcCaCBAcCcaDA";
//     println!("part1: {}", part2(example));
    println!("part1: {}", part1(include_str!("day5.txt")));
    println!("part2: {}", part2(include_str!("day5.txt")));
}

fn part1(input: &str) -> usize {
    let mut polymer: String = input.to_owned();
    let mut polymer_length: usize = std::usize::MAX;
    while polymer.len() != polymer_length {
        polymer_length = polymer.len();

        let mut temp: Vec<char> = Vec::new();
        for character in polymer.chars() {
            if temp.is_empty() {
                temp.push(character);
                continue;
            } 
            let previous = temp[temp.len()-1].to_string();
            let current = character.to_string();
            
            if previous != current && previous.to_lowercase() == current.to_lowercase() {
                temp.pop();
            } else {
                temp.push(character);
            }
        }

        polymer = temp.into_iter().collect();
    }
    
    polymer_length
}

fn part2(input: &str) -> usize {
    "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .fold(std::usize::MAX, |answer, character| {
            let polymer: String = input
                .chars()
                .filter(|x| x.to_string().to_lowercase() != character.to_string().to_lowercase())
                .collect();
            let polymer_len = part1(&polymer);
            if (polymer_len<answer) {
                polymer_len
            } else {
                answer
            }
        })
}