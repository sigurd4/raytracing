use core::{f64::EPSILON, ops::{AddAssign, Range}};

use array_math::{ArrayMath, ArrayOps};
use num::Float;
use option_trait::MaybeCell;

use crate::{shapes::Shape, Ray, Raytrace};

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
    fn raytrace<const N: bool>(&self, ray: &Ray<F, 3>) -> Raytrace<F, 3, N>
    where
        [(); N as usize]:
    {
        let eps = F::epsilon();

        let c1 = self.center.map(|c| c - self.diameter*F::from(0.5).unwrap());
        let c2 = self.center.map(|c| c + self.diameter*F::from(0.5).unwrap());

        let inside = false && (ray.r.into_iter()
            .zip(c1.zip(c2))
            .all(|(r, (c1, c2))| r >= c1 && r <= c2));

        let mut t_min = F::infinity();
        let mut n_min = MaybeCell::from_fn(|| None);

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
                    n_min = MaybeCell::from_fn(|| {
                        let mut n = [F::zero(); 3];
                        n[k] = if inside {-F::one()} else {F::one()};
                        Some(n)
                    });
                    t_min = t;
                }
                
                let t = (c2[k] - ray.r[k])/ray.v[k];
                if t >= F::zero() && t < t_min && (inside || {
                    let x = ray.propagate(t);
                    (0..2).map(|i| (k + i + 1) % 3)
                        .all(|n| x[n] >= c1[n] - eps && x[n] <= c2[n] + eps)
                })
                {
                    n_min = MaybeCell::from_fn(|| {
                        let mut n = [F::zero(); 3];
                        n[k] = if inside {F::one()} else {-F::one()};
                        Some(n)
                    });
                    t_min = t;
                }
            }
        }
        
        Raytrace {
            t: t_min,
            n: n_min
        }
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::{FRAC_PI_2, FRAC_PI_4, TAU};

    use crate::{shapes::Transform, tests};

    use super::Cube;

    #[test]
    fn test()
    {
        let shape = Transform::new(Cube::new([0.0, 0.0, 0.0], 2.0))
            .rotate([1.0, 0.0, 0.0], -FRAC_PI_4)
            .rotate([0.0, 1.0, 0.0], FRAC_PI_4);

        const D: f64 = 0.0;
        const A: f64 = 0.1;

        tests::project_3d_spin(&shape, [0.0, 0.0, -20.0], D, A);
    }
}