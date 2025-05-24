use core::ops::AddAssign;

use num::Float;

use crate::{shapes::Shape, Ray, Raytrace, RaytraceWithNorm};

use super::HyperRectangle;

#[derive(Debug, Clone, Copy)]
pub struct HyperCube<F, const D: usize>
where
    F: Float
{
    pub center: [F; D],
    pub radius: F
}

impl<F, const D: usize> From<HyperCube<F, D>> for HyperRectangle<F, D>
where
    F: Float
{
    fn from(cube: HyperCube<F, D>) -> Self
    {
        let c1 = cube.center.map(|c| c - cube.radius);
        let c2 = cube.center.map(|c| c + cube.radius);
        Self {
            c1,
            c2
        }
    }
}

impl<F, const D: usize> Shape<F, D> for HyperCube<F, D>
where
    F: Float + AddAssign
{
    fn raytrace(&self, ray: &Ray<F, D>) -> Raytrace<F, D>
    {
        HyperRectangle::from(*self).raytrace(ray)
    }

    fn raytrace_with_norm(&self, ray: &Ray<F, D>) -> RaytraceWithNorm<F, D>
    {
        HyperRectangle::from(*self).raytrace_with_norm(ray)
    }
}