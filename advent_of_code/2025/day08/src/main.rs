struct UnionFind {
  parent: Vec<usize>,
  size: Vec<usize>,
  components: usize,
}

impl UnionFind {
  pub fn new(n: usize) -> UnionFind {
    let mut parent = Vec::with_capacity(n);
    let mut size = Vec::with_capacity(n);
    let components = n;

    for i in 0..n {
//      parent[i] = i; size[i] = 1; doesn't work
      parent.push(i);
      size.push(1);
    }
    UnionFind{parent, size, components}
  }

  fn find(&mut self, x: usize) -> usize {
    if self.parent[x] != x {
      self.parent[x] =  self.find(self.parent[x]);
    }
    self.parent[x]
  }

//  fn find_slow(&self, x: usize) -> usize {
//    if self.parent[x] == x {
//      x
//    } else {
//      self.find_slow(self.parent[x]) // No mutation happening here!
//    }
//  }

  pub fn unite(&mut self, mut a: usize, mut b: usize) -> bool {
    a = self.find(a);
    b = self.find(b);

    if a == b {
      return false;
    }

    if self.size[a] < self.size[b] {
      std::mem::swap(&mut a, &mut b);
    }

    self.parent[b] = a;
    self.size[a] += self.size[b];
    self.components -= 1;

    true
  }

  pub fn get_components(&self) -> usize {
    self.components
  }

}

fn parse(path: &str) -> Vec<(i64, i64, i64)> {
  std::fs::read_to_string(path)
    .expect("Failed to read input file")
    .lines()
    .map(|line| {
      let mut parts = line.split(',');

      let a = parts.next().unwrap().parse::<i64>().unwrap();
      let b = parts.next().unwrap().parse::<i64>().unwrap();
      let c = parts.next().unwrap().parse::<i64>().unwrap();

      (a, b, c)
    })
    .collect()
}

fn build_dist_table(points: &[(i64, i64, i64)]) -> Vec<((usize, usize), i64)> {
  let mut dist_table = Vec::new();

  for (i, &(x1, y1, z1)) in points.iter().enumerate() {
    for (j_offset, &(x2, y2, z2)) in points.iter().skip(i + 1).enumerate() {
      let j = i + 1 + j_offset;

      let dist = (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2) + (z1 - z2) * (z1 - z2);

      dist_table.push(((i, j), dist));
    }
  }
  dist_table.sort_by_key(|&(_, dist)| dist);
  dist_table
}

fn part1(points: &[(i64, i64, i64)], dist_table: &[((usize, usize), i64)]) -> i64 {
  let mut uf = UnionFind::new(points.len());

  for (i, &item) in dist_table.iter().enumerate() {
    if i > 1000 {
      break;
    }
    let ((first, second), _) = item;
    uf.unite(first, second);
  }

  let mut groups = vec![0; points.len()];

  for i in 0..points.len() {
    groups[uf.find(i)] += 1;
  }

  groups.sort_by(|a,b| b.cmp(a));

  groups[0] * groups[1] * groups[2]
}

fn part2(points: &[(i64, i64, i64)], dist_table: &[((usize, usize), i64)]) -> Option<i64> {
  let mut uf = UnionFind::new(points.len());
  let mut result = None;

  for (i, &item) in dist_table.iter().enumerate() {
    let ((first, second), _) = item;

    if uf.unite(first, second) && uf.get_components() == 1 {
      let (x1, y1, z1) = points[first];
      let (x2, y2, z2) = points[second];

      result = Some(x1 * x2);
    }
  }
  result
}

fn main() {
  let points = parse( "input.txt");
  let dist_table = build_dist_table(&points);

  let p1 = part1(&points, &dist_table);
  println!("Part 1: {}", p1);

  match part2(&points, &dist_table) {
    Some(answer) => println!("Part 2: {}", answer),
    None => println!("Part 2: No solution"),
  }
}
