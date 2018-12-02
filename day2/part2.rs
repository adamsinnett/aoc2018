fn main() {
    for (i, line) in include_str!("day2.txt").lines().enumerate() {
      for test_line in include_str!("day2.txt").lines().skip(i+1) {
        let count = line.chars().zip(test_line.chars()).filter(|(c1, c2)| c1 != c2).count();
        if count == 1 {
          println!("{:?}", line
            .chars()
            .zip(test_line.chars())
            .filter(|(c1, c2)| c1 == c2)
            .map(|c| c.0)
            .collect::<String>());
        }
      }
    }
    

}