use core::{iter::Sum, ops::{AddAssign, DivAssign, MulAssign}};

use num::Float;

use crate::{shapes::{Shape, _3d::Plane, nd::HyperPlane}, Ray, Raytrace, RaytraceWithNorm};

#[derive(Debug, Clone)]
pub struct Tetrahedron<F>
where
    F: Float
{
    pub center: [F; 3],
    pub diameters: [F; 4]
}

impl<F> Tetrahedron<F>
where
    F: Float
{
    fn _raytrace<I, N>(
        &self,
        ray: &Ray<F, 3>,
        inside: impl FnOnce(&[[F; 3]; 4], &[HyperPlane<F, 3>; 4]) -> I,
        n_init: N,
        n: impl Fn(HyperPlane<F, 3>, I) -> N
    ) -> (Raytrace<F, 3>, N)
    where
        F: MulAssign + Sum,
        I: Copy
    {
        let v = unsafe {
            V.into_iter()
                .zip(self.diameters)
                .map(|(v, d)| v.map(|v| d*F::from(v/2.0).unwrap()))
                .next_chunk::<4>()
                .unwrap_unchecked()
        };

        let s = core::array::from_fn::<_, 4, _>(|i| {
            let mut v = v;
            v.rotate_left(i);

            Plane::from_vertices(unsafe {
                *v.first_chunk()
                    .unwrap_unchecked()
            })
        });

        let inside = inside(&v, &s);
        let mut t_min = F::infinity();
        let mut n_min = n_init;

        'lp:
        for (i, si) in s.into_iter()
            .enumerate()
        {
            let t = {
                let vn = ray.v.into_iter()
                    .zip(si.n)
                    .map(|(v, n)| v*n)
                    .sum::<F>();
                si.r.into_iter()
                    .zip(ray.r)
                    .zip(si.n)
                    .map(|((r0, r), n)| (r0 - r)*n)
                    .sum::<F>()/vn
            };

            if t < F::zero() || t >= t_min
            {
                continue;
            }

            let x = ray.propagate(t);
            for k in 0..3
            {
                let v1 = v[(i + k + 1) % 4];
                let v4 = v[(i + k) % 4];

                let n = s[(i + k + 1) % 4].n;
                let dv4 = n.into_iter()
                    .zip(v4)
                    .zip(v1)
                    .map(|((n, v4), v1)| n*(v4 - v1))
                    .sum::<F>();
                let dx = n.into_iter()
                    .zip(x)
                    .zip(v1)
                    .map(|((n, x), v1)| n*(x - v1))
                    .sum::<F>();

                if (dv4 >= F::zero()) != (dx >= F::zero())
                {
                    continue 'lp;
                }
            }

            t_min = t;
            n_min = n(si, inside);
        }

        (
            Raytrace {
                t: t_min
            },
            n_min
        )
    }
}

const V: [[f64; 3]; 4] = [
    [0.94280904158206336586779248280647, -1.0/3.0, 0.0],
    [-0.47140452079103168293389624140323, -1.0/3.0, 0.81649658092772603273242802490196],
    [-0.47140452079103168293389624140323, -1.0/3.0, -0.81649658092772603273242802490196],
    [0.0, 1.0, 0.0]
];

impl<F> Shape<F, 3> for Tetrahedron<F>
where
    F: Float + AddAssign + MulAssign + DivAssign + Sum
{
    fn raytrace(&self, ray: &Ray<F, 3>) -> Raytrace<F, 3>
    {
        let (raytrace, ()) = self._raytrace(ray, |_, _| (), (), |_, _| ());
        raytrace
    }

    fn raytrace_with_norm(&self, ray: &Ray<F, 3>) -> RaytraceWithNorm<F, 3>
    {
        let (raytrace, n) = self._raytrace(
            ray,
            |v, s| {
                for k in 0..4
                {
                    let v1 = v[(k + 1) % 4];
                    let v4 = v[(k) % 4];

                    let n = s[(k + 1) % 4].n;
                    let dv4 = n.into_iter()
                        .zip(v4)
                        .zip(v1)
                        .map(|((n, v4), v1)| n*(v4 - v1))
                        .sum::<F>();
                    let dx = n.into_iter()
                        .zip(ray.r)
                        .zip(v1)
                        .map(|((n, r), v1)| n*(r - v1))
                        .sum::<F>();

                    if (dv4 >= F::zero()) != (dx >= F::zero())
                    {
                        return false
                    }
                }
                true
            },
            None,
            |si, inside| {
                let mut n = si.n;
                let mut n_norm = si.n.into_iter()
                    .map(|n| n*n)
                    .sum::<F>()
                    .sqrt();
                if !inside
                {
                    n_norm = -n_norm;
                }
                for n in n.iter_mut()
                {
                    *n /= n_norm
                }

                Some(n)
            }
        );

        RaytraceWithNorm {
            raytrace,
            n
        }
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::FRAC_PI_4;

    use crate::{shapes::Transform, tests};

    use super::Tetrahedron;

    #[test]
    #[ignore]
    fn test()
    {
        let shape = Transform::new(Tetrahedron {
                center: [0.0, 0.0, 0.0],
                diameters: [2.0, 2.0, 2.0, 2.0]
            }).rotate([1.0, 0.0, 0.0], -FRAC_PI_4)
            .rotate([0.0, 1.0, 0.0], FRAC_PI_4);

        const D: f64 = 1.0;
        const A: f64 = 0.0;

        tests::project_3d_spin(&shape, [0.0, 0.0, -3.0], D, A);
    }
}