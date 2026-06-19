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

  ids.iter()
      .filter(|&&id| { // .filter(|id| **id > 3)
          ranges
              .iter()
              .any(|&(lo, hi)| lo <= id && id <= hi)
      })
      .count()

//  let mut fresh_count = 0;
//
//  for id in ids {
//    let mut fresh = false;
//
//    for (lo, hi) in ranges {
//      if *lo <= *id && *id <= *hi {
//        fresh = true;
//        break;
//      }
//    }
//
//    if fresh {
//      fresh_count += 1;
//    }
//  }
//
//  fresh_count

}

fn part2(ranges: &mut [Range]) -> i64 {
  ranges.sort_unstable_by_key(|&(lo, _)| lo);
  let mut merged: Vec<Range> = Vec::new();

  let &(mut first_lo, mut last_hi) = ranges.first().unwrap();

  for &(lo, hi) in ranges.iter().skip(1) {
    // last_hi = last_hi.max(hi);
    if lo <= last_hi + 1 {
      if last_hi < hi {
        last_hi = hi;
      }
    }
    else {
      merged.push((first_lo, last_hi));
      first_lo = lo;
      last_hi = hi;
    }
  }

  merged.push((first_lo, last_hi));

  // merged.iter().map(|&(lo, hi) hi - lo + 1).sum()
  let mut sum = 0;
  for (lo, hi) in merged {
    sum += hi - lo + 1;
  }

  sum
}

fn main() {
  let mut data = parse("input.txt");

  println!("Part 1: {}", part1(&data));
//  let (ranges, _) = &data;
//  println!("Part 2: {}", part2(ranges));
  println!("Part 2: {}", part2(&mut data.0));
}
