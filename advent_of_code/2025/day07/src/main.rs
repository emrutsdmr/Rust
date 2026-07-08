use std::fs;
use std::collections::HashMap;

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
  let mid = data[0].len() / 2;
  let mut cp_data = data.to_vec();
  let mut memo: HashMap<(usize, usize), usize> = HashMap::new();

  cp_data[1][mid] = '|';

  let rows = data.len();
  let cols = data[0].len();

  return part2_recursive(&cp_data, &mut memo, 1, mid, rows, cols);
}

fn part2_recursive(data: &[Vec<char>],
                   memo: &mut HashMap<(usize, usize), usize>,
                   i: usize, j: usize,
                   rows: usize, cols: usize) -> usize{
  if let Some(&value) = memo.get(&(i,j)) {
    return value;
  }

  if i == rows - 1 {
    return 1;
  }

  let mut ans;

  if data[i + 1][j] == '^' {
    ans = 0;
    if j > 0 {
      ans += part2_recursive(data, memo, i + 1, j - 1, rows, cols);
    }

    if j + 1 < cols {
      ans += part2_recursive(data, memo, i + 1, j + 1, rows, cols);
    }
  }
  else {
    ans = part2_recursive(data, memo, i + 1, j, rows, cols);
  }
  memo.insert((i,j), ans);
  ans
}

fn main() {
  let data = parse("input.txt");

  println!("Part 1: {}", part1(&data));
  println!("Part 2: {}", part2(&data));
}
