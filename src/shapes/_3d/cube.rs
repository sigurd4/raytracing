use core::{f64::EPSILON, ops::{AddAssign, Range}};

use array_math::{ArrayMath, ArrayOps};
use num::Float;

use crate::{Ray, shapes::Shape};

#[derive(Debug, Clone)]
pub struct Cube<F>
where
    F: Float
{
    pub center: [F; 3],
    pub diameter: F
}

impl<F> Cube<F>
where
    F: Float
{
    pub fn new(center: [F; 3], diameter: F) -> Self
    {
        Self {
            center,
            diameter
        }
    }
}

impl<F> Shape<F, 3> for Cube<F>
where
    F: Float + AddAssign
{
    fn raytrace(&self, ray: &Ray<F, 3>) -> F
    {
        let eps = F::epsilon();

        let c1 = self.center.map(|c| c - self.diameter*F::from(0.5).unwrap());
        let c2 = self.center.map(|c| c + self.diameter*F::from(0.5).unwrap());

        let inside = false && (ray.r.into_iter()
            .zip(c1.zip(c2))
            .all(|(r, (c1, c2))| r >= c1 && r <= c2));

        let mut t_min = F::infinity();

        for k in 0..3
        {
            if ray.v[k] != F::zero()
            {
                let t = (c1[k] - ray.r[k])/ray.v[k];
                if t >= F::zero() && t < t_min && (inside || {
                    let x = ray.propagate(t);
                    (0..2).map(|i| (k + i + 1) % 3)
                        .all(|n| x[n] >= c1[n] - eps && x[n] <= c2[n] + eps)
                })
                {
                    t_min = t;
                }
                
                let t = (c2[k] - ray.r[k])/ray.v[k];
                if t >= F::zero() && t < t_min && (inside || {
                    let x = ray.propagate(t);
                    (0..2).map(|i| (k + i + 1) % 3)
                        .all(|n| x[n] >= c1[n] - eps && x[n] <= c2[n] + eps)
                })
                {
                    t_min = t;
                }
            }
        }
        
        t_min
    }

    fn raytrace_norm(&self, ray: &Ray<F, 3>) -> (F, Option<[F; 3]>)
    {
        let eps = F::epsilon();

        let c1 = self.center.map(|c| c - self.diameter*F::from(0.5).unwrap());
        let c2 = self.center.map(|c| c + self.diameter*F::from(0.5).unwrap());

        let inside = false && (ray.r.into_iter()
            .zip(c1.zip(c2))
            .all(|(r, (c1, c2))| r >= c1 && r <= c2));

        let mut t_min = F::infinity();
        let mut n_min = None;

        for k in 0..3
        {
            if ray.v[k] != F::zero()
            {
                let t = (c1[k] - ray.r[k])/ray.v[k];
                if t >= F::zero() && t < t_min && (inside || {
                    let x = ray.propagate(t);
                    (0..2).map(|i| (k + i + 1) % 3)
                        .all(|n| x[n] >= c1[n] - eps && x[n] <= c2[n] + eps)
                })
                {
                    let mut n = [F::zero(); 3];
                    n[k] = if inside {F::one()} else {-F::one()};
                    n_min = Some(n);
                    t_min = t;
                }
                
                let t = (c2[k] - ray.r[k])/ray.v[k];
                if t >= F::zero() && t < t_min && (inside || {
                    let x = ray.propagate(t);
                    (0..2).map(|i| (k + i + 1) % 3)
                        .all(|n| x[n] >= c1[n] - eps && x[n] <= c2[n] + eps)
                })
                {
                    let mut n = [F::zero(); 3];
                    n[k] = if inside {-F::one()} else {F::one()};
                    n_min = Some(n);
                    t_min = t;
                }
            }
        }
        
        (t_min, n_min)
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::{FRAC_PI_2, FRAC_PI_4};

    use crate::{shapes::Transform, tests};

    use super::Cube;

    #[test]
    fn test()
    {
        let shape = Transform::new(Cube::new([0.0, 0.0, 0.0], 2.0))
            .rotate([1.0, 0.0, 0.0], -FRAC_PI_4)
            .rotate([0.0, 1.0, 0.0], FRAC_PI_4);

        const D: f64 = 3.0;
        const A: f64 = 0.0;

        tests::project_3d_spin(&shape, [0.0, 0.0, -3.0], D, A);
    }
}