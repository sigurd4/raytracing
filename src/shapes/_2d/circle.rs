use core::ops::AddAssign;

use array_math::{ArrayMath, ArrayOps};
use num::Float;

use crate::{Ray, shapes::Shape};

#[derive(Debug, Clone, Copy)]
pub struct Circle<F>
where
    F: Float
{
    pub r0: [F; 2],
    pub r: F
}

impl<F> Circle<F>
where
    F: Float
{
    pub fn new(r0: [F; 2], r: F) -> Self
    {
        Self {
            r0,
            r
        }
    }
}

impl<F> Shape<F, 2> for Circle<F>
where
    F: Float + AddAssign
{
    fn raytrace(&self, ray: &Ray<F, 2>) -> F
    {
        let r2 = self.r*self.r;
        let v_abs = ray.v.magnitude();
        let d = self.r0.sub_each(ray.r);
        let dsq = d.magnitude_squared();
        let a = d.mul_dot(ray.v)/v_abs;
        if a >= F::zero()
        {
            let f = r2 - dsq + a*a;

            if f >= F::zero()
            {
                return if dsq < r2
                {
                    (a + f.sqrt())/v_abs
                }
                else
                {
                    (a - f.sqrt())/v_abs
                }
            }
        }

        F::infinity()
    }

    fn raytrace_norm(&self, ray: &Ray<F, 2>) -> (F, Option<[F; 2]>)
    {
        let r2 = self.r*self.r;
        let v_abs = ray.v.magnitude();
        let d = self.r0.sub_each(ray.r);
        let dsq = d.magnitude_squared();
        let a = d.mul_dot(ray.v)/v_abs;
        if a >= F::zero()
        {
            let f = r2 - dsq + a*a;

            if f >= F::zero()
            {
                let t = if dsq < r2
                {
                    (a + f.sqrt())/v_abs
                }
                else
                {
                    (a - f.sqrt())/v_abs
                };
                return (
                    t,
                    Some(
                        ray.r.add_each(ray.v.mul_all(t))
                            .sub_each(self.r0)
                            .normalize()
                    )
                )
            }
        }

        (F::infinity(), None)
    }
}

#[cfg(test)]
mod test
{
    use crate::{shapes::Transform, tests};

    use super::Circle;

    #[test]
    fn test()
    {
        let shape = Circle::new([0.0, 0.0], 1.0);
    }
}