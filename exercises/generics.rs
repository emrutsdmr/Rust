use std::fmt;
pub struct MatchingPair<T> {
  first: T,
  second: T
}

pub enum MyOption<T> {
  Sumthin(T), Nuthin
}

impl fmt::Display for MyOption<u32> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      MyOption::Sumthin(num) => write!(f, "Sumthin({})", num),
      MyOption::Nuthin => write!(f, "Nuthin :("),
    }
  }
}

impl<T> MatchingPair<T> {
  pub fn new(first: T, second: T) -> Self {
    MatchingPair {first: first, second: second}
  }
}


impl fmt::Display for MatchingPair<char> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "({}, {})", self.first, self.second)
  }
}

fn main() {
  let ps: MatchingPair<char> = MatchingPair::new('p', 'p');
  println!("two ps: {}", ps);
  let my_some_five: MyOption<u32> = MyOption::Sumthin(5);
  let my_nuthin: MyOption<u32> = MyOption::Nuthin;
  println!("my_some_five: {}", my_some_five);
  println!("my_nuthin: {}", my_nuthin);
}
