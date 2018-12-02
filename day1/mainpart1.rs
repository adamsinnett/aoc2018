fn main() {
    let sum = include_str!("day1.txt")
        .lines()
        .filter_map(|s| s.parse::<isize>().ok())
        .sum::<isize>();
        
    println!("{}", sum);
}