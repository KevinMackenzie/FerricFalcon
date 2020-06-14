use nalgebra::base::*;
mod math;

fn main() {
    println!("Hello, world!");
    let v0 = Vector3::new(1.0, 2.0, 3.0);
    let n = 2.0 * v0;
    let r = math::ray::Ray{pos: v0, dir: Unit::new_normalize(Vector3::new(0.0, 1.0, 1.0))};
    let s = r.derive(4.0);
    println!("{}", n);
    println!("{}", s);
    println!("{}", r);
}
