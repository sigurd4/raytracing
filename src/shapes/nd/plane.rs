use core::ops::{AddAssign, MulAssign};

use array_math::{ArrayMath, ArrayOps};
use num::{traits::real::Real, Float};
use option_trait::MaybeCell;

use crate::{shapes::Shape, Ray, Raytrace};

#[derive(Debug, Clone, Copy)]
pub struct Plane<F, const D: usize>
where
    F: Float
{
    pub r: [F; D],
    pub n: [F; D]
}

impl<F, const D: usize> Plane<F, D>
where
    F: Float
{
    pub const fn new(r: [F; D], n: [F; D]) -> Self
    {
        Self {
            r,
            n
        }
    }
}

impl<F> Plane<F, 3>
where
    F: Float
{
    pub fn new_from_vertices(v: [[F; 3]; 3]) -> Self
    where
        F: MulAssign
    {
        let r = v[0];
        let n = v[1].sub_each(v[0]).mul_cross([&v[2].sub_each(v[0])]);
        Self::new(r, n)
    }
}

impl<F, const D: usize> Shape<F, D> for Plane<F, D>
where
    F: Float + AddAssign
{
    fn raytrace<const N: bool>(&self, ray: &Ray<F, D>) -> Raytrace<F, D, N>
    where
        [(); N as usize]:
    {
        let vn = ray.v.mul_dot(self.n);
        let t = self.r.sub_each(ray.r).mul_dot(self.n)/vn;
        if t >= F::zero()
        {
            return Raytrace {
                t,
                n: MaybeCell::from_fn(|| Some(self.n.normalize_to(vn.signum())))
            };
        }
        Raytrace::miss()
    }
}

#[cfg(test)]
mod test
{
    use crate::tests;

    use super::Plane;

    #[test]
    fn test()
    {
        let shape = Plane::new([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);

        const D: f64 = 1.0;
        const A: f64 = 0.1;

        tests::project_3d_spin(&shape, [0.0, 0.0, -1.0], D, A);
    }
}