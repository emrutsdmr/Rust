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

fn part2(ranges: &Vec<(u64, u64)>) -> u64 {
  let mut sum: u64 = 0;

  for &(start, end) in ranges {
    for id in start..=end {
      if is_invalid_part2(id) {
        sum += id;
      }
    }
  }
  sum
}

fn is_invalid_part2(id: u64) -> bool {
  let id_str = id.to_string();
  let len = id_str.len();
  let mut group = 1;
  let mut index = 0;
  let mut invalid: bool;

  while group != len {
    if len % group != 0 {
      group += 1;
      continue;
    }
    invalid = true;

    while (index + group + group) <= len {
      if &id_str[index..(index + group)] == &id_str[(index + group)..(index + group + group)] {
        index += group;
      }
      else {
        invalid = false;
        break;
      }
    }
    if invalid { return true}
    group += 1;
    index = 0;
  }
  false
}

fn main() {
  let input = fs::read_to_string("input.txt").expect("Failed to read input file");
  let ranges: Vec<(u64, u64)> = parse_instruction(&input);
  println!("Part 1 Solution: {}", part1(&ranges));
  println!("Part 2 Solution: {}", part2(&ranges));
}
