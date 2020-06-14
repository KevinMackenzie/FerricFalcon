use nalgebra::base::{Unit, Vector3};
pub mod ray;
pub mod sampling;

pub fn fromSphere(r: f64, azimuth: f64, incl: f64) -> Vector3<f64> {
    let sinth = incl.sin();
    let x = r * sinth * azimuth.cos();
    let y = r * sinth * azimuth.sin();
    let z = r * incl.cos();
    Vector3::new(x, y, z)
}

pub fn fromSphereN(azimuth: f64, incl: f64) -> Unit<Vector3<f64>> {
    let sinth = incl.sin();
    let x = sinth * azimuth.cos();
    let y = sinth * azimuth.sin();
    let z = incl.cos();
    Unit::new_unchecked(Vector3::new(x, y, z))
}

pub fn getOrthoVec(v: &Vector3<f64>) -> Unit<Vector3<f64>> {
    let xAxis: Vector3<f64> = Vector3::new(1.0, 0.0, 0.0);
    let yAxis: Vector3<f64> = Vector3::new(0.0, 1.0, 0.0);
    let o = if v.cross(&xAxis).magnitude() < 0.000001 {
        yAxis
    } else {
        xAxis
    };
    Unit::new_normalize(v.cross(&o))
}
