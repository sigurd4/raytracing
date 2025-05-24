moddef::moddef!(
    flat(pub) mod {
        transform
    },
    pub mod {
        _2d,
        _3d,
        nd
    }
);

use core::ops::Deref;

use num::Float;

use crate::{Ray, Raytrace, RaytraceWithNorm};

pub trait Shape<F, const D: usize>
where
    F: Float
{
    fn raytrace(&self, ray: &Ray<F, D>) -> Raytrace<F, D>;

    fn raytrace_with_norm(&self, ray: &Ray<F, D>) -> RaytraceWithNorm<F, D>;
}

impl<F, const D: usize, I> Shape<F, D> for I
where
    F: Float,
    for<'a> &'a I: IntoIterator<Item: Deref<Target: Shape<F, D>>>
{
    fn raytrace(&self, ray: &Ray<F, D>) -> Raytrace<F, D>
    {
        self.into_iter()
            .map(|shape| shape.raytrace(ray))
            .reduce(Raytrace::min)
            .unwrap_or_else(Raytrace::miss)
    }

    fn raytrace_with_norm(&self, ray: &Ray<F, D>) -> RaytraceWithNorm<F, D>
    {
        self.into_iter()
            .map(|shape| shape.raytrace_with_norm(ray))
            .reduce(RaytraceWithNorm::min)
            .unwrap_or_else(RaytraceWithNorm::miss)
    }
}