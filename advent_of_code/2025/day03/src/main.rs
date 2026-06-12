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
  let mut sum: u64 = 0;

  for bank in banks {
    let len = bank.len();

    assert!(len == 15);
//TODO go through the code for [2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]
    let mut num: u64 = 0;
    let mut count = 3;
    let mut ind = 0;
    let mut chosen = bank[0];
    let mut chosen_ind = 0;

    println!("{:?}", bank);
    println!("{}", "434234234278");
    while count > 0 && ind < 12{
      let tmp_count = count;
      for i in 1..=tmp_count {
        if chosen < bank[ind + i] {
          chosen = bank[ind + i];
          chosen_ind = ind + i;
          count -= 1;
          println!("chosen: {:?} count: {:?}", chosen, count);
        }
      }
      num = num * 10 + chosen as u64;
      println!("Number: {:?}", num);
      if count >= 0 {
        if  ind < chosen_ind && chosen_ind + 1 < 15{
          ind = chosen_ind + 1;
          chosen = bank[ind];
        }
        else{
          ind += 1;
          chosen = bank[ind];
        }
      }
    }
    let mut i = 0;
    while ind < 12 && ind +i < 15 {
      num = num * 10 + bank[ind + i] as u64;
      println!("{:?} {:?} {:?}", ind + i, bank[ind + i], num);
      i += 1;
    }
    println!("{:?}", num);
    sum += num;
  }
  sum
}

fn main() {
  let banks = parse("input.txt");
  println!("{}", part2(&banks));
}
