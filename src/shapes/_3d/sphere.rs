use core::ops::AddAssign;

use array_math::{ArrayMath, ArrayOps};
use num::Float;
use option_trait::MaybeCell;

use crate::{shapes::Shape, Ray, Raytrace};

#[derive(Debug, Clone, Copy)]
pub struct Sphere<F>
where
    F: Float
{
    pub r0: [F; 3],
    pub r: F
}

impl<F> Sphere<F>
where
    F: Float
{
    pub fn new(r0: [F; 3], r: F) -> Self
    {
        Self {
            r0,
            r
        }
    }
}

impl<F> Shape<F, 3> for Sphere<F>
where
    F: Float + AddAssign
{
    fn raytrace<const N: bool>(&self, ray: &Ray<F, 3>) -> Raytrace<F, 3, N>
    where
        [(); N as usize]:
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

                return Raytrace {
                    t,
                    n: MaybeCell::from_fn(|| Some(
                        ray.r.add_each(ray.v.mul_all(t))
                            .sub_each(self.r0)
                            .normalize()
                    ))
                }
            }
        }

        Raytrace::miss()
    }
}

#[cfg(test)]
mod test
{
    use crate::{shapes::Transform, tests};

    use super::Sphere;

    #[test]
    fn test()
    {
        let shape = Transform::new(Sphere::new([0.0, 0.0, 0.0], 1.0))
            .scale([2.0, 2.0, 2.0]);

        const D: f64 = 3.0;
        const A: f64 = 0.0;

        tests::project_3d_spin(&shape, [0.0, 0.0, -5.0], D, A);
    }
}