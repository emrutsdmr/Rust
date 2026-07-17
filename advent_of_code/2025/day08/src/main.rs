use std::fs;


fn parse(path: &str) -> Vec<(i32, i32, i32)> {
  fs::read_to_string(path)
    .expect("Failed to read input file")
    .lines()
    .map(|line| {
      let mut parts = line.split(',');

      let a = parts.next().unwrap().parse::<i32>().unwrap();
      let b = parts.next().unwrap().parse::<i32>().unwrap();
      let c = parts.next().unwrap().parse::<i32>().unwrap();

      (a, b, c)
    })
    .collect()
}

fn build_dist_table(points: &Vec<(i64, i64, i64)>) -> Vec<((usize, usize), i64)> {
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


fn main() {


}
