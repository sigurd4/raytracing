use core::{iter::Sum, ops::{AddAssign, DivAssign}};

use num::Float;

use crate::{shapes::Shape, Ray, Raytrace, RaytraceWithNorm};

#[derive(Debug, Clone, Copy)]
pub struct HyperSphere<F, const D: usize>
where
    F: Float
{
    pub r0: [F; D],
    pub r: F
}

impl<F, const D: usize> HyperSphere<F, D>
where
    F: Float
{
    pub fn new(r0: [F; D], r: F) -> Self
    {
        Self {
            r0,
            r
        }
    }
}

impl<F, const D: usize> Shape<F, D> for HyperSphere<F, D>
where
    F: Float + AddAssign + DivAssign + Sum
{
    fn raytrace(&self, ray: &Ray<F, D>) -> Raytrace<F, D>
    {
        let r2 = self.r*self.r;
        let v_abs = ray.v.into_iter()
            .map(|v| v*v)
            .sum::<F>()
            .sqrt();
        let d = unsafe {
            self.r0.into_iter()
            .zip(ray.r)
            .map(|(r0, r)| r0 - r)
            .next_chunk::<D>()
            .unwrap_unchecked()
        };
        let dsq = d.into_iter()
            .map(|d| d*d)
            .sum();
        let a = d.into_iter()
            .zip(ray.v)
            .map(|(d, v)| d*v)
            .sum::<F>()/v_abs;
        if a >= F::zero()
        {
            let f = r2 - dsq + a*a;

            if f >= F::zero()
            {
                let t = (a + f.sqrt()*(r2 - dsq).signum())/v_abs;
                return Raytrace {
                    t
                }
            }
        }

        Raytrace::miss()
    }

    fn raytrace_with_norm(&self, ray: &Ray<F, D>) -> RaytraceWithNorm<F, D>
    {
        let raytrace @ Raytrace {t} = self.raytrace(ray);

        raytrace.with_norm(|| {
            let mut n = unsafe {
                ray.r.into_iter()
                    .zip(ray.v)
                    .zip(self.r0)
                    .map(|((r, v), r0)| r + v*t - r0)
                    .next_chunk()
                    .unwrap_unchecked()
            };
            let n_norm = n.into_iter()
                .map(|n| n*n)
                .sum::<F>()
                .sqrt();
            for n in n.iter_mut()
            {
                *n /= n_norm
            }
            n
        })
    }
}