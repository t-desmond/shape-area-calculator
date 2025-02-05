use core::f64;

pub trait Area {
  fn area(&self) -> f64;
}
pub struct Cicle{
  pub raduis: f64
}

impl Cicle{
  pub fn new(raduis: f64) -> Self{
    Self{raduis}
  }
}

pub struct Rectangle{
  pub width: f64,
  pub height: f64
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self{
      Self{width, height}
    }
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