use std::fs;

fn parse(path: &str) -> Vec<Vec<char>> {
  let contents = fs::read_to_string(path)
      .expect("Failed to read file");

  contents
      .lines()
      .map(|line| line.chars().collect())
      .collect()
}

fn part1(grid: &Vec<Vec<char>>) -> usize {
  let row = grid.len();
  let col = grid[0].len();

  let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1),
                    (0, 1), (1, -1), (1, 0), (1, 1),];

  let mut accessible = 0;
  let mut i = 0;

  while i < row {
    let mut j = 0;
    while  j < col {
      if grid[i][j] != '@' {
        j += 1;
        continue;
      }
      let mut neighbors = 0;

      for (di, dj) in directions {
        let ni = i as isize + di;
        let nj = j as isize + dj;

        if ni >= 0 && ni < row as isize && nj >= 0 && nj < col as isize {
          if grid[ni as usize][nj as usize] == '@' {
            neighbors += 1;
          }
        }
      }

      if neighbors < 4 {
        accessible += 1;
      }
      j += 1;
    }
    i += 1;
  }

  accessible
}

fn main() {
  let grid = parse("input.txt");
//  println!("{:?}", grid);
  println!("Solution: {}", part1(&grid));
}
