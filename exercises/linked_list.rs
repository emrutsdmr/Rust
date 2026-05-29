use std::fmt;

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

  pub fn get_size(&self) -> usize {
    self.size // (*self).size
  }

  pub fn is_empty(&self) -> bool {
    self.size == 0 // self.get_size() == 0
  }

  pub fn push(&mut self, value: u32){
    let new_node: Box<Node> = Box::new(Node::new(value, self.head.take()));
    self.head = Some(new_node);
    self.size += 1;
  }

  pub fn pop(&mut self) -> Option<u32> {
    let node: Box<Node> = self.head.take()?;
    self.head = node.next;
    self.size -= 1;
    Some(node.value)
  }

/*
  pub fn display(&self){
    let mut current: &Option<Box<Node>> = &self.head;
    let mut result = String::new();
    loop{
      match current{
        Some(node) => {
          result  = format!("{} {}", result, node.value);
          current = &node.next;
        },
        None => break,
      }
    }
    println!("{}", result);
  }

  pub fn push(&mut self, value: u32) {
    let new_node = Box::new(Node::new(value, None));
  
    match self.head.as_mut() {
      None => {
        self.head = Some(new_node);
      }
  
      Some(current) => {
        let mut current = current;
  
        while current.next.is_some() {
            current = current.next.as_mut().unwrap();
        }
  
        current.next = Some(new_node);
      }
    }
  
    self.size += 1;
  }
*/
}

impl fmt::Display for LinkedList {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut current: &Option<Box<Node>> = &self.head;
    let mut result = String::new();

    loop {
      match current {
        Some(node) => {
          result = format!("{} {}", result, node.value);
          current = &node.next;
        },
        None => break,
      }
    }

    write!(f, "{}", result)
  }
}

impl Drop for LinkedList {
  fn drop(&mut self) {
    let mut current = self.head.take();

    while let Some(mut node) = current {
      current = node.next.take();
    }
  }
}


fn main() {
  let mut list: LinkedList = LinkedList::new();
  println!("{}", list.is_empty());
  assert!(list.is_empty());
  assert_eq!(list.get_size(), 0);
  for i in 1..10 {
    list.push(i);
  }
//  list.display();
  println!("{}", list);
  println!("Top Element: {}", list.pop().unwrap());
//  list.display();
  println!("{}", list);
}
