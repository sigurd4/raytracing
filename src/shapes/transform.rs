use core::{iter::Sum, ops::{AddAssign, DivAssign, SubAssign}};

use num::{Float, Signed};

use crate::{matrix, vec3, Ray, Raytrace, RaytraceWithNorm};

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
    F: Float
{
    pub fn new(s: S) -> Self
    {
        Self {
            s,
            t: matrix::identity(),
            t_inv: matrix::identity()
        }
    }

    pub fn transform_pos(&self, r: [F; D]) -> [F; D]
    where
        F: Sum
    {
        matrix::mul_matrix_collumn(self.t, r)
    }

    pub fn inv_transform_pos(&self, r: [F; D]) -> [F; D]
    where
        F: Sum
    {
        matrix::mul_matrix_collumn(self.t_inv, r)
    }

    pub fn transform(mut self, t: [[F; D]; D], t_inv: [[F; D]; D]) -> Self
    {
        self.t = matrix::mul_matrix_matrix(t, &self.t);
        self.t_inv = matrix::mul_matrix_matrix(t_inv, &self.t_inv);
        self
    }

    pub fn scale(self, scale: [F; D]) -> Self
    {
        self.transform(matrix::diagonal(scale), matrix::diagonal(scale.map(|scale| scale.recip())))
    }
}

impl<F, S> Transform<F, S, 2>
where
    S: Shape<F, 2>,
    F: Float
{
    pub fn rotate(self, theta: F) -> Self
    {
        let c = theta.cos();
        let s = theta.sin();

        self.transform(
            [
                [c, -s],
                [s, c],
            ],
            [
                [c,  s],
                [-s, c]
            ]
        )
    }
}

impl<F, S> Transform<F, S, 3>
where
    S: Shape<F, 3>,
    F: Float
{
    pub fn rotate(self, axis: [F; 3], theta: F) -> Self
    {
        let [x, y, z] = axis;

        let c = theta.cos();
        let s = theta.sin();
        let c1m = F::one() - c;

        self.transform(
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
        )
    }

    pub fn mirror(self, n: [F; 3]) -> Self
    {
        let [a, b, c] = vec3::normalize(n);

        let one = F::one();
        let two = F::from(2.0).unwrap();

        let t = [
            [one - two*a*a, -two*a*b,      -two*a*c],
            [-two*a*b,      one - two*b*b, -two*b*c],
            [-two*a*c,      -two*b*c,      one - two*c*c]
        ];

        self.transform(t, t)
    }
}

impl<F, S, const D: usize> Shape<F, D> for Transform<F, S, D>
where
    S: Shape<F, D>,
    F: Float + Sum
{
    fn raytrace(&self, ray: &Ray<F, D>) -> Raytrace<F, D>
    {
        let r_from = self.inv_transform_pos(ray.r);
        let r_to = self.inv_transform_pos(ray.r_to());
        let ray = Ray::new_from_to(r_from, r_to);
        self.s.raytrace(&ray)
    }

    fn raytrace_with_norm(&self, ray: &Ray<F, D>) -> RaytraceWithNorm<F, D>
    {
        let r_from = self.inv_transform_pos(ray.r);
        let r_to = self.inv_transform_pos(ray.r_to());
        let ray = Ray::new_from_to(r_from, r_to);
        let raytrace @ RaytraceWithNorm {raytrace: Raytrace {t}, n: _} = self.s.raytrace_with_norm(&ray);
        raytrace.map_norm(|n| {
                let mut n0 = ray.propagate(t);
                let mut n1 = unsafe {
                    n0.into_iter()
                        .zip(n)
                        .map(|(n0, n)| n0 + n)
                        .next_chunk()
                        .unwrap_unchecked()
                };
                n0 = self.transform_pos(n0);
                n1 = self.transform_pos(n1);
    
                unsafe {
                    n1.into_iter()
                        .zip(n0)
                        .map(|(n1, n0)| n1 - n0)
                        .next_chunk()
                        .unwrap_unchecked()
                }
            })
    }
}