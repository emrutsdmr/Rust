use std::fs;

fn parse_instruction(line: &str) -> Vec<(u64, u64)> {
  line.trim()
    .split(',')
    .map(|s| {
        let (start, end) = s.split_once('-').unwrap();
        (
          start.parse().unwrap(),
          end.parse().unwrap(),
        )
    })
    .collect()
}

fn part1(ranges: &Vec<(u64, u64)>) -> u64 {

  let mut sum: u64 = 0;
  for &(start, end) in ranges {
    for id in start..=end {
      if is_invalid_part1(id) {
        sum += id;
      }
    }
  }

  sum
}

fn is_invalid_part1(id: u64) -> bool {
  let id_str = id.to_string();

  if id_str.len() % 2 == 0 {
    let mid: usize = id_str.len() / 2;

    if id_str[..mid] == id_str[mid..] {
      return true;
    }
  }
  false
}

fn is_invalid_part2(id: u64) -> bool {
}


fn part2(input: &str) -> u64 {
}

fn main() {
  let input = fs::read_to_string("input.txt").expect("Failed to read input file");
  let ranges: Vec<(u64, u64)> = parse_instruction(&input);
//  println!("Part 1 Solution: {}", part1(&ranges));
  println!("Part 2 Solution: {}", part2(&ranges));
}
