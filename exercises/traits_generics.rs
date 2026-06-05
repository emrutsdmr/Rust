use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
  x: f64,
  y: f64
}

impl Point {
  pub fn new(x: f64, y: f64) -> Self {
    Point{x: x, y: y}
  }
}

pub trait ComputeNorm {
  fn compute_norm(&self) -> f64 {
    0.0
  }
}

impl ComputeNorm for Option<u32> {}

impl ComputeNorm for Point {
  fn compute_norm(&self) -> f64 {
    (self.x * self.x + self.y * self.y).sqrt()
  }
}

impl ComputeNorm for Vec<f64> {
  fn compute_norm(&self) -> f64 {
    // functional rust syntax
    self.iter().map(|x| {x * x}).sum::<f64>().sqrt()
  }
}

impl Add for Point {
  type Output = Self; // an associated type
  fn add(self, other: Self) -> Self {
    Point::new(self.x + other.x, self.y + other.y)
  }
}

fn main() {
  let the_origin = Point::new(0.0, 0.0);
  let mut p = the_origin; // copy
  println!("p: {:?}, the_origin: {:?}", p, the_origin);
  println!("Are they equal? => {}", p == the_origin);
  p.x += 10.0;
  println!("p: {:?}, the_origin: {:?}", p, the_origin);
  println!("Are they equal? => {}", p == the_origin);
  let some_opt = Some(110);
  println!("Norm of some_opt: {}", some_opt.compute_norm());
  let lil_vec: Vec<f64> = vec![3.0, 4.0];
  println!("Norm of lil_vec: {}", lil_vec.compute_norm());
  println!("Norm of (3,4) + the_origin: {}",
            (the_origin + Point::new(3.0, 4.0)).compute_norm());
}
