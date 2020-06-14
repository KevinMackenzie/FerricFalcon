use nalgebra::base::{Unit, Vector3};
use std::fmt;

pub struct Ray {
    pub pos: Vector3<f64>,
    pub dir: Unit<Vector3<f64>>,
}

impl Ray {
    pub fn derive(self: &Ray, p: f64) -> Ray {
        Ray {
            pos: self.eval_param(p),
            dir: self.dir,
        }
    }
    pub fn eval_param(self: &Ray, p: f64) -> Vector3<f64> {
        self.pos + p * self.dir.as_ref()
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}    |\n    v{})", self.pos, self.dir.as_ref())
    }
}
