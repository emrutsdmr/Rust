use std::fs;

type Range = (i64, i64);

fn parse(path: &str) -> (Vec<Range>, Vec<i64>) {
  let input = fs::read_to_string(path).expect("failed to read input");

  let mut sections = input.split("\n\n");

  let ranges = sections
      .next()
      .unwrap()
      .lines()
      .map(|line| {
          let (a, b) = line.split_once('-').unwrap();
          (a.parse().unwrap(), b.parse().unwrap())
      })
      .collect::<Vec<_>>();

  let ids = sections
      .next()
      .unwrap()
      .lines()
      .map(|line| line.parse().unwrap())
      .collect::<Vec<_>>();

  (ranges, ids)
}

fn part1(data: &(Vec<Range>, Vec<i64>)) -> usize {
  let (ranges, ids) = data;
  let mut fresh_count = 0;


//    for &id in ids {
//        for &(lo, hi) in ranges {
//            if lo <= id && id <= hi {
//                // ...
//            }
//        }
//    }
  for id in ids {
    let mut fresh = false;

    for (lo, hi) in ranges {
      if *lo <= *id && *id <= *hi {
        fresh = true;
        break;
      }
    }

    if fresh {
      fresh_count += 1;
    }
  }

  fresh_count
}

fn main() {
  let data = parse("input.txt");

  println!("Part 1: {}", part1(&data));
}
