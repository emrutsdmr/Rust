use std::fs;

fn parse(path: &str) -> Vec<Vec<char>> {
  let contents = fs::read_to_string(path)
      .expect("Failed to read file");

  contents
      .lines()
      .map(|line| line.chars().collect())
      .collect()
}

fn part1(grid: &[Vec<char>]) -> usize {
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

fn part2(grid: &mut Vec<Vec<char>>) -> usize {
  let row = grid.len();
  let col = grid[0].len();

  let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1),
                    (0, 1), (1, -1), (1, 0), (1, 1),];

//copy grid and modify it on the run
  let mut cp_grid = grid.clone();
  let mut sum_accessible = 0;
  let mut accessible = 1;
  while accessible > 0 {
    accessible = 0;
    let mut i = 0;
    while i < row {
      let mut j = 0;
      while  j < col {
        if grid[i][j] != '@' {
          cp_grid[i][j] = '.';
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
          cp_grid[i][j] = '.';
        }
        else {
          cp_grid[i][j] = '@';
        }
        j += 1;
      }
      i += 1;
    }
//    *grid = cp_grid.clone();
    std::mem::swap(grid, &mut cp_grid);
    sum_accessible += accessible;
  }

  sum_accessible
}

fn main() {
  let mut grid = parse("input.txt");
  println!("Solution 1: {}", part1(&grid));
  println!("Solution 2: {}", part2(&mut grid));
}
