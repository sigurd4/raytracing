use num::Float;

use crate::{shapes::Shape, Ray, Raytrace, RaytraceWithNorm};

#[derive(Debug, Clone)]
pub struct HyperRectangle<F, const D: usize>
where
    F: Float
{
    pub c1: [F; D],
    pub c2: [F; D]
}

impl<F, const D: usize> HyperRectangle<F, D>
where
    F: Float
{
    fn _raytrace<U>(&self, ray: &Ray<F, D>, n_init: U, n: impl Fn(usize, bool) -> U) -> (Raytrace<F, D>, U)
    {
        let eps = F::epsilon();

        let inside = false && (ray.r.into_iter()
            .zip(self.c1)
            .zip(self.c2)
            .all(|((r, c1), c2)| r >= c1 && r <= c2));

        let mut t_min = F::infinity();
        let mut n_min = n_init;

        for k in 0..D
        {
            if ray.v[k] != F::zero()
            {
                let t = (self.c1[k] - ray.r[k])/ray.v[k];
                if t >= F::zero() && t < t_min && (inside || {
                    let x = ray.propagate(t);
                    (1..D).map(|i| (k + i) % D)
                        .all(|n| x[n] >= self.c1[n] - eps && x[n] <= self.c2[n] + eps)
                })
                {
                    n_min = n(k, !inside);
                    t_min = t;
                }
                
                let t = (self.c2[k] - ray.r[k])/ray.v[k];
                if t >= F::zero() && t < t_min && (inside || {
                    let x = ray.propagate(t);
                    (1..D).map(|i| (k + i) % D)
                        .all(|n| x[n] >= self.c1[n] - eps && x[n] <= self.c2[n] + eps)
                })
                {
                    n_min = n(k, inside);
                    t_min = t;
                }
            }
        } 
        
        (
            Raytrace {
                t: t_min
            },
            n_min
        )
    }
}

impl<F, const D: usize> Shape<F, D> for HyperRectangle<F, D>
where
    F: Float
{
    fn raytrace(&self, ray: &Ray<F, D>) -> Raytrace<F, D>
    {
        let (raytrace, ()) = self._raytrace(ray, (), |_, _| ());
        raytrace
    }

    fn raytrace_with_norm(&self, ray: &Ray<F, D>) -> RaytraceWithNorm<F, D>
    {
        let (raytrace, n) = self._raytrace(ray, None, |k, inside| {
            let mut n = [F::zero(); D];
            n[k] = if inside {F::one()} else {-F::one()};
            Some(n)
        });
        
        RaytraceWithNorm {
            raytrace,
            n
        }
    }
}