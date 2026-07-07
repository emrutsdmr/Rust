use std::fs;

fn parse(path: &str) -> Vec<Vec<char>> {
  fs::read_to_string(path)
      .expect("Failed to read input file")
      .lines()
      .map(|line| line.chars().collect())
      .collect()
}

fn part1(data: &[Vec<char>]) -> usize {
  let mid = data[0].len() / 2;
  let mut cp_data = data.to_vec();
  let mut result = 0;

  cp_data[1][mid] = '|';

  for i in 1..cp_data.len() - 1 {
    for j in 0..cp_data[i].len() {
      if cp_data[i][j] == '|' {
        if cp_data[i + 1][j] == '^' {
          if j > 0 {
            cp_data[i + 1][j - 1] = '|';
          }

          if j + 1 < cp_data[i + 1].len() {
            cp_data[i + 1][j + 1] = '|';
          }

          result += 1;
        }
        else {
          cp_data[i + 1][j] = '|';
        }
      }
    }
  }

  result
}

fn part2(data: &[Vec<char>]) -> usize {
    todo!()
}

fn main() {
  let data = parse("input.txt");

  println!("Part 1: {}", part1(&data));
  println!("Part 2: {}", part2(&data));
}
