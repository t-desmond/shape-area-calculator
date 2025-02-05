mod shapes;
use shapes;
use core::f64;

pub trait Area {
  fn area(&self) -> f64;
}

impl Area for Cicle {
  fn area(&self) -> f64 {
    std::f64::consts::PI * (self.raduis *self.raduis)
  }
}

impl Area for Rectangle {
  fn area(&self) -> f64 {
      self.height * self.width
  }
}