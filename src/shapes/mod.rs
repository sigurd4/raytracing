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

use crate::{Ray, Raytrace};

pub trait Shape<F, const D: usize>
where
    F: Float
{
    fn raytrace<const N: bool>(&self, ray: &Ray<F, D>) -> Raytrace<F, D, N>
    where
        [(); N as usize]:;
}

impl<F, const D: usize, I> Shape<F, D> for I
where
    F: Float,
    for<'a> &'a I: IntoIterator<Item: Deref<Target: Shape<F, D>>>
{
    fn raytrace<const N: bool>(&self, ray: &Ray<F, D>) -> Raytrace<F, D, N>
    where
        [(); N as usize]:
    {
        self.into_iter()
            .map(|shape| shape.raytrace(ray))
            .reduce(|a, b| if a.t < b.t {a} else {b})
            .unwrap_or(Raytrace::miss())
    }
}