use core::ops::{AddAssign, DivAssign, SubAssign};

use array_math::{max_len, Array2dOps, ArrayMath, ArrayOps, MatrixMath, SquareMatrixMath};
use num::{Float, Signed};

use crate::Ray;

use super::Shape;

#[derive(Debug, Clone, Copy)]
pub struct Transform<F, S, const D: usize>
where
    S: Shape<F, D>,
    F: Float,
{
    s: S,
    t: [[F; D]; D],
    t_inv: [[F; D]; D]
}

impl<F, S, const D: usize> Transform<F, S, D>
where
    S: Shape<F, D>,
    F: Float,
    [(); D + 1]:
{
    pub fn new(s: S) -> Self
    {
        Self {
            s,
            t: MatrixMath::identity_matrix(),
            t_inv: MatrixMath::identity_matrix()
        }
    }

    pub fn transform_pos(&self, r: [F; D]) -> [F; D]
    where
        F: AddAssign,
        [(); D + 1 - D]:
    {
        self.t.mul_matrix(&r.map(|r| [r]))
            .map(|[r]| r)
    }

    pub fn inv_transform_pos(&self, r: [F; D]) -> [F; D]
    where
        F: AddAssign,
        [(); D + 1 - D]:
    {
        self.t_inv.mul_matrix(&r.map(|r| [r]))
            .map(|[r]| r)
    }


    pub fn transform(mut self, (t, t_inv): ([[F; D]; D], [[F; D]; D])) -> Self
    where
        F: AddAssign + Signed + SubAssign + DivAssign,
    {
        self.t = t.mul_matrix(&self.t);
        self.t_inv = t_inv.mul_matrix(&self.t_inv);
        self
    }

    pub fn scale(self, scale: [F; D]) -> Self
    where
        F: AddAssign + Default + Signed + SubAssign + DivAssign,
        [(); D - D]:,
    {
        self.transform((scale.diagonal(), scale.map(|scale| scale.recip()).diagonal()))
    }
}

impl<F, S> Transform<F, S, 2>
where
    S: Shape<F, 2>,
    F: Float
{
    pub fn rotate(self, theta: F) -> Self
    where
        F: AddAssign + Signed + SubAssign + DivAssign
    {
        let c = theta.cos();
        let s = theta.sin();

        self.transform((
            [
                [c, -s],
                [s, c ],
            ],
            [
                [c,  s],
                [-s, c]
            ]
        ))
    }
}

impl<F, S> Transform<F, S, 3>
where
    S: Shape<F, 3>,
    F: Float
{
    pub fn rotate(self, axis: [F; 3], theta: F) -> Self
    where
        F: AddAssign + Signed + SubAssign + DivAssign
    {
        let [x, y, z] = axis;

        let c = theta.cos();
        let s = theta.sin();
        let c1m = F::one() - c;

        self.transform((
            [
                [x*x*c1m + c,   y*x*c1m - z*s, z*x*c1m + y*s],
                [x*y*c1m + z*s, y*y*c1m + c,   z*y*c1m - x*s],
                [x*z*c1m - y*s, y*z*c1m + x*s, z*z*c1m + c  ],
            ],
            [
                [x*x*c1m + c,   y*x*c1m + z*s, z*x*c1m - y*s],
                [x*y*c1m - z*s, y*y*c1m + c,   z*y*c1m + x*s],
                [x*z*c1m + y*s, y*z*c1m - x*s, z*z*c1m + c  ],
            ]
        ))
    }

    pub fn mirror(self, n: [F; 3]) -> Self
    where
        F: AddAssign + Signed + SubAssign + DivAssign
    {
        let [a, b, c] = n.normalize();

        let _1 = F::one();
        let _2 = F::from(2.0).unwrap();

        self.transform((
            [
                [_1 - _2*a*a, -_2*a*b,     -_2*a*c],
                [-_2*a*b,     _1 - _2*b*b, -_2*b*c],
                [-_2*a*c,     -_2*b*c,     _1 - _2*c*c]
            ],
            [
                [_1 - _2*a*a, -_2*a*b,     -_2*a*c],
                [-_2*a*b,     _1 - _2*b*b, -_2*b*c],
                [-_2*a*c,     -_2*b*c,     _1 - _2*c*c]
            ]
        ))
    }
}

impl<F, S, const D: usize> Shape<F, D> for Transform<F, S, D>
where
    S: Shape<F, D>,
    F: Float + AddAssign,
    [(); D + 1]:,
    [(); D + 1 - D]:
{
    fn raytrace(&self, ray: &Ray<F, D>) -> F
    {
        let ray = Ray::new_from_to(self.inv_transform_pos(ray.r), self.inv_transform_pos(ray.r.add_each(ray.v)));
        self.s.raytrace(&ray)
    }
    fn raytrace_norm(&self, ray: &Ray<F, D>) -> (F, Option<[F; D]>)
    {
        let ray = Ray::new_from_to(self.inv_transform_pos(ray.r), self.inv_transform_pos(ray.r.add_each(ray.v)));
        let (t, n) = self.s.raytrace_norm(&ray);
        let n = if let Some(n) = n
        {
            let n0 = ray.propagate(t);
            let n1 = n0.add_each(n);

            Some(self.transform_pos(n1).sub_each(self.transform_pos(n0)))
        }
        else
        {
            None
        };
        (t, n)
    }
}