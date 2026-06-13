use std::fs;

fn parse(path: &str) -> Vec<Vec<u8>> {
  fs::read_to_string(path)
      .expect("failed to read input")
      .lines()
      .map(|line| {
          line.chars()
              .map(|c| c.to_digit(10).unwrap() as u8)
              .collect()
      })
      .collect()
}

fn part1(banks: &Vec<Vec<u8>>) -> u64 {
  let mut sum: u64 = 0;
  let mut first: Option<u8>;
  let mut second: Option<u8>;

  for bank in banks {
    let len = bank.len();

    if len > 2 {
      first  = Some(bank[0]);
      second = Some(bank[1]);
    }
    else {
      continue;
    }

    let mut ind = 1;
    while (ind + 1) < len {
      if second > first {
        first = second;
        second = Some(bank[ind + 1]);
      }
      else if second < Some(bank[ind + 1]) {
        second = Some(bank[ind + 1]);
      }
      ind += 1;
    }
    sum += (first.unwrap() * 10 + second.unwrap()) as u64;
  }
  sum
}

fn part2(banks: &Vec<Vec<u8>>) -> u64 {
  let mut sum = 0;

  for bank in banks {
    let mut number: u64 = 0;
    let mut start = 0;

    for digits_left in (1..=12).rev() {
      let end = bank.len() - digits_left;

      let mut best_digit = bank[start];
      let mut best_index = start;

      for i in start..=end {
        if bank[i] > best_digit {
          best_digit = bank[i];
          best_index = i;
        }
      }

      number = number * 10 + best_digit as u64;
      start = best_index + 1;
    }

    println!("{}", number);
    sum += number;
  }

  sum
}

fn main() {
  let banks = parse("input.txt");
  println!("Sum: {}", part2(&banks));
}
