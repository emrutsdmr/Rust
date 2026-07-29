use std::collections::{BTreeMap, HashMap, HashSet};

fn parse(path: &str) -> Vec<(i64, i64)> {
  std::fs::read_to_string(path)
    .expect("Failed to read the file")
    .lines()
    .map(|line| {
      let mut parts = line.split(',');
      let x = parts.next().unwrap().parse::<i64>().unwrap();
      let y = parts.next().unwrap().parse::<i64>().unwrap();
      (x, y)
    })
    .collect()
}

fn part1(red_tiles: &[(i64, i64)]) -> i64 {
  let mut result = 0;

  for (i, (x1, y1)) in red_tiles.iter().enumerate() {
    for (x2, y2) in red_tiles.iter().skip(i + 1) {
      let area = ((x1 - x2 + 1) * (y1 - y2 + 1)).abs();
      if result < area {
        result = area;
      }
    }
  }

  result
}

struct Vertex {
  values: (u32,u32),
  neighbors: Vec<usize>,
}

impl Vertex {
  pub fn new(x: u32, y: u32) -> Self {
//    let values = (x, y);
//    let mut neighbors = Vec::new();
//    Self{values, neighbors}
    Self{values: (x, y), neighbors: Vec::new()}
  }
}

pub struct Field {
  size_x: usize,
  size_y: usize,
  cur_longest: Vec<usize>,
  longest: Vec<usize>,
  start: usize,
  visited: HashSet<usize>,

  vertices: Vec<Vertex>,
  vertex_map: HashMap<(u32, u32), usize>,

  x_map: BTreeMap<u32, Vec<usize>>,
  y_map: BTreeMap<u32, Vec<usize>>,
}

fn part2(red_tiles: &[(i64,i64)]) -> i64 {
  let mut result = 0;
  let size: usize = red_tiles.len();

  if size == 0 {
    return 0;
  }

  let mut xs = Vec::with_capacity(size);
  let mut ys = Vec::with_capacity(size);

  for &(x,y) in red_tiles.iter() {
    xs.push(x);
    ys.push(y);
  }

  xs.sort();
  xs.dedup();

  ys.sort();
  ys.dedup();

  let mut compressed = Vec::with_capacity(size);

  for &(x, y) in red_tiles.iter() {
    let cx = xs.binary_search(&x).unwrap();
    let cy = ys.binary_search(&y).unwrap();
    compressed.push((cx, cy));
  }

//  println!("{:?}", compressed);

  return result;
}

fn main() {
  let input = parse("input.txt");

//println!("Part 1: {}", part1(&input));
  println!("Part 2: {}", part2(&input));
}
