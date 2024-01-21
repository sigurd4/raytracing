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

use num::Float;

use crate::Ray;

pub trait Shape<F, const DIMENSIONS: usize>
where
    F: Float
{
    fn raytrace(&self, ray: &Ray<F, DIMENSIONS>) -> F;
    fn raytrace_norm(&self, ray: &Ray<F, DIMENSIONS>) -> (F, Option<[F; DIMENSIONS]>);
}

impl<F, const D: usize, I> Shape<F, D> for I
where
    F: Float,
    I: IntoIterator<Item: Shape<F, D>> + Clone
{
    fn raytrace(&self, ray: &Ray<F, D>) -> F
    {
        self.clone().into_iter()
            .map(|shape| shape.raytrace(ray))
            .reduce(Float::min)
            .unwrap_or(F::infinity())
    }
    fn raytrace_norm(&self, ray: &Ray<F, D>) -> (F, Option<[F; D]>)
    {
        self.clone().into_iter()
            .map(|shape| shape.raytrace_norm(ray))
            .reduce(|a, b| if a.0 < b.0 {a} else {b})
            .unwrap_or((F::infinity(), None))
    }
}