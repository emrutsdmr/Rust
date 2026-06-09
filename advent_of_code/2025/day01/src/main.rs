use std::fs;

const START_POSITION: i32 = 50;
const DIAL_SIZE: i32 = 100;

fn part1(input: &str) -> u32 {
  let mut position = START_POSITION;
  let mut count = 0;

  for line in input.lines() {
    let (direction, mut distance) = parse_instruction(line);
    distance %= DIAL_SIZE;
    if direction == 'L'{ //line.as_bytes()[0] as char
      distance *= -1;
    }
    position += distance;
    if position >= DIAL_SIZE {
      position -= DIAL_SIZE;
    }
    else if position < 0 {
      position += DIAL_SIZE;
    }
    if position == 0 {
      count += 1;
    }
  }
  count
}

fn parse_instruction(line: &str) -> (char, i32) {
  let direction = line.chars().next().unwrap();
  let distance = line[1..].parse::<i32>().unwrap();

  (direction, distance)
}

fn main() {
  let input = fs::read_to_string("input.txt").unwrap();
  println!("Part 1 Solution: {}", part1(&input));
}
