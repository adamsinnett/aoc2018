const INPUT: &str = include_str!("day1.txt");

fn main() {
    let sum = INPUT.lines().filter_map(|s| s.parse::<isize>().ok()).sum::<isize>();
    println!("{}", sum);
}