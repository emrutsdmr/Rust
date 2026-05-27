struct Node{
  value: u32,
  next: Option<Box<Node>>,
}

impl Node{
  pub fn new(value: u32, next: Option<Box<Node>>) -> Node{
    Node{value: value, next: next}
  }
}

struct LinkedList{
  head: Option<Box<Node>>,
  size: usize,
}

impl LinkedList{
  pub fn new() -> LinkedList {
    LinkedList{head: None, size: 0}
  }
}


fn main() {
  println!("{}", "Hello");
}
