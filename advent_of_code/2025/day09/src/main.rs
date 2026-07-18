fn parse(path: &str) -> Vec<(i64, i64)> {
  std::fs::read_to_string(path)
    .expect("Failed to read the file")
    .lines()
    .map(|line| {
      let mut parts = line.split(',');
      let x = parts.next().unwrap().parse::<i64>().unwrap();
      let y = parts.next().unwrap().parse::<i64>().unwrap();
      (x, y)
    })
    .collect()
}

fn part1(red_tiles: &[(i64, i64)]) -> i64 {
  let mut result = 0;

  for (i, (x1, y1)) in red_tiles.iter().enumerate() {
    for (x2, y2) in red_tiles.iter().skip(i + 1) {
      let area = ((x1 - x2 + 1) * (y1 - y2 + 1)).abs();
      if result < area {
        result = area;
      }
    }
  }

  result
}

fn main() {
  let input = parse("input.txt");
  let answer = part1(&input);

  println!("Part 1: {}", answer);
}
