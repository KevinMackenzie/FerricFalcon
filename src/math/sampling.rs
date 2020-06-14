use nalgebra::base::{Dim, DimName, Matrix3, Unit, Vector3, VectorN};
use std::f64::consts::PI;

use rand::prelude::*;

// Generate a random vector on the unit-hypersphere
//  TODO: Make this generic over type (it was hard)
pub fn uniform_sphere<R: Rng + ?Sized, D: Dim + DimName>(rng: &mut R) -> Unit<VectorN<f64, D>>
where
    nalgebra::base::default_allocator::DefaultAllocator:
        nalgebra::base::allocator::Allocator<f64, D>,
{
    let mut v: VectorN<f64, D>;
    loop {
        v = rng.gen();
        if v.magnitude() > 1.0 {
            break;
        }
    }
    Unit::new_normalize(v)
}

pub fn uniform_hemisphere<R: Rng + ?Sized>(
    norm: &Unit<Vector3<f64>>,
    rng: &mut R,
) -> Unit<Vector3<f64>> {
    let v: Unit<Vector3<f64>> = uniform_sphere(rng);
    if v.dot(norm.as_ref()) > 0.0 {
        v
    } else {
        -v
    }
}

pub fn cos_hemisphere<R: Rng + ?Sized>(
    norm: &Unit<Vector3<f64>>,
    rng: &mut R,
) -> Unit<Vector3<f64>> {
    let rand0: f64 = rng.gen();
    let rand1: f64 = rng.gen();
    let v = super::fromSphereN(rand0.sqrt().acos(), 2.0 * PI * rand1);
    let tan = super::getOrthoVec(&norm);
    let bit = Unit::new_unchecked(norm.cross(&tan));
    let tf = Matrix3::from_columns(&[tan.into_inner(), bit.into_inner(), norm.into_inner()]);
    return Unit::new_unchecked(tf * v.into_inner())
}
