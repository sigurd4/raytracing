use core::{iter::Sum, ops::{AddAssign, DivAssign, MulAssign}};

use num::Float;

use crate::{shapes::Shape, vec3, Ray, Raytrace, RaytraceWithNorm};

#[derive(Debug, Clone, Copy)]
pub struct HyperPlane<F, const D: usize>
where
    F: Float
{
    pub r: [F; D],
    pub n: [F; D]
}

impl<F> HyperPlane<F, 3>
where
    F: Float
{
    pub fn from_vertices(v: [[F; 3]; 3]) -> Self
    where
        F: MulAssign
    {
        let [r, v1, v2] = v;
        let n = vec3::mul_cross(vec3::sub(v1, r), vec3::sub(v2, r));
        
        Self {
            r,
            n
        }
    }
}

impl<F, const D: usize> HyperPlane<F, D>
where
    F: Float
{
    fn _raytrace(&self, ray: &Ray<F, D>) -> Option<(Raytrace<F, D>, F)>
    where
        F: Sum
    {
        let vn = ray.v.into_iter()
            .zip(self.n)
            .map(|(v, n)| v*n)
            .sum();
        let t = self.r.into_iter()
            .zip(ray.r)
            .zip(self.n)
            .map(|((r0, r), n)| (r0 - r)*n)
            .sum::<F>()/vn;
        if t.is_sign_positive()
        {
            return Some((
                Raytrace {
                    t
                },
                vn
            ));
        }
        None
    }
}

impl<F, const D: usize> Shape<F, D> for HyperPlane<F, D>
where
    F: Float + AddAssign + DivAssign + Sum
{
    fn raytrace(&self, ray: &Ray<F, D>) -> Raytrace<F, D>
    {
        self._raytrace(ray)
            .map(|(ray, _)| ray)
            .unwrap_or_else(Raytrace::miss)
    }

    fn raytrace_with_norm(&self, ray: &Ray<F, D>) -> RaytraceWithNorm<F, D>
    {
        self._raytrace(ray)
            .map(|(ray, vn)| ray.with_norm(|| {
                    let mut n = self.n;
                    let n_norm = n.into_iter()
                        .map(|n| n*n)
                        .sum::<F>()
                        .sqrt()*vn.signum();
                    for n in n.iter_mut()
                    {
                        *n /= n_norm
                    }
                    n
                })
            ).unwrap_or_else(RaytraceWithNorm::miss)
    }
}