use std::fs;

fn is_invalid(id: u64) -> bool {
  let id_str = id.to_string();

  if id_str.len() % 2 == 0 {
    let mid: usize = id_str.len() / 2;

    if id_str[..mid] == id_str[mid..] {
      return true;
    }
  }
  false
}

fn main() {
  let input = fs::read_to_string("input.txt").expect("Failed to read input file");

  let ranges: Vec<(u64, u64)> = input
    .trim()
    .split(',')
    .map(|s| {
      let (start, end) = s.split_once('-').unwrap();
//      println!("start = {:?}", start);
//      println!("end   = {:?}", end);
      (
          start.parse().unwrap(),
          end.parse().unwrap(),
      )
    })
    .collect();

    let mut sum: u64 = 0;
    for (start, end) in ranges {
      for i in start..=end {
        if is_invalid(i) {
          sum += i;
        }
      }
    }

  println!("{}", sum);
}
