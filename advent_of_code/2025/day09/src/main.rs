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

impl Field {
  pub fn new() -> Self {
    Self {
      size_x: 0,
      size_y: 0,
      cur_longest: Vec::new(),
      longest: Vec::new(),
      start: 0,
      visited: HashSet::new(),
      vertices: Vec::new(),
      vertex_map: HashMap::new(),
      x_map: BTreeMap::new(),
      y_map: BTreeMap::new(),
    }
  }

  pub fn create_vertexes(&mut self, compressed: &[(u32, u32)]) {
    for &(x, y) in compressed {
      let idx = self.vertices.len();
      self.vertices.push(Vertex::new(x, y));

      self.vertex_map.insert((x, y), idx);
      self.x_map.entry(x).or_default().push(idx);
      self.y_map.entry(y).or_default().push(idx);
    }

    self.size_x = self.x_map.len();
    self.size_y = self.y_map.len();
  }

  pub fn connect_vertexes(&mut self) {
    let x_cols: Vec<Vec<usize>> = self.x_map.values().cloned().collect();
    for mut col in x_cols {
      if col.len() < 2 {
        continue;
      }
      col.sort_by_key(|&idx| self.vertices[idx].values.1);
      for i in 0..col.len() - 1 {
        let (a, b) = (col[i], col[i + 1]);
        self.vertices[a].neighbors.push(b);
        self.vertices[b].neighbors.push(a);
      }
    }

    let y_rows: Vec<Vec<usize>> = self.y_map.values().cloned().collect();
    for mut row in y_rows {
      if row.len() < 2 {
        continue;
      }
      row.sort_by_key(|&idx| self.vertices[idx].values.0);
      for i in 0..row.len() - 1 {
        let (a, b) = (row[i], row[i + 1]);
        self.vertices[a].neighbors.push(b);
        self.vertices[b].neighbors.push(a);
      }
    }
  }
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
