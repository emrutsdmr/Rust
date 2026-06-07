use std::fs;

fn main() {
  let input = fs::read_to_string("input.txt").unwrap();
  let mut result: u32  = 0;
  let mut count:  i16 = 50;
  let mut number: i16;

  for line in input.lines() {
    number = (&line[1..]).parse::<i16>().unwrap();
    number %= 100;
    if line.chars().next().unwrap() == 'L'{ //line.as_bytes()[0] as char
      number *= -1;
    }
    count += number;
    if count > 99 {
      count -= 100;
    }
    else if count < 0 {
      count += 100;
    }
    if count == 0 {
      result += 1;
    }
  }
  println!("{}", result);
}
