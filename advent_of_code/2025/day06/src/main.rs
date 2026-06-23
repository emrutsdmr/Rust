use std::fs;

struct Problem {
  op: char,
  rows: Vec<String>,
}

fn parse(path: &str) -> Vec<Problem> {
  let input = fs::read_to_string(path).unwrap();
  let rows: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();

  let h = rows.len();
  //Find longest row
  let w = rows.iter().map(|r| r.len()).max().unwrap();

  let mut problems = Vec::new();
  let mut start = None;


  for c in 0..=w {
//    let mut blank = true;
//
//    if c != w {
//      for r in 0..h {
//        let ch = rows[r].get(c).copied().unwrap_or(' ');
//    
//        if ch != ' ' {
//          blank = false;
//          break;
//        }
//      }
//    }

    //.all(...) : Is the condition true for every value?

    let blank = c == w || (0..h).all(|r| rows[r].get(c).copied().unwrap_or(' ') == ' ');

    match (start, blank) {
      (None, false) => start = Some(c),
      (Some(s), true) => {
        let op = (s..c)
          .find_map(|x| {//find_map means: Search until closure returns Some(...)
            let ch = rows[h - 1].get(x).copied().unwrap_or(' ');
            matches!(ch, '+' | '*').then_some(ch)
          })
          .unwrap();

        let block = (0..h - 1)
          .map(|r| {
            (s..c)
              .map(|x| rows[r].get(x).copied().unwrap_or(' '))
              .collect()
          })
          .collect();

        problems.push(Problem { op, rows: block });
        start = None;
      }
      _ => {}
    }
  }

  problems
}

fn eval_part1(problem: &Problem) -> i64 {
}

fn part1(data: &[Problem]) -> i64 {
  data.iter().map(eval_part1).sum()
}

fn eval_part2(problem: &Problem) -> i64 {
}

fn part2(data: &[Problem]) -> i64 {
  data.iter().map(eval_part2).sum()
}

fn main() {
  let data = parse("input.txt");

  println!("Part 1: {}", part1(&data));
  println!("Part 2: {}", part2(&data));
}
