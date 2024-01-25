use core::{f64::EPSILON, ops::{AddAssign, MulAssign, Range}};

use array_math::{ArrayMath, ArrayOps};
use num::Float;
use option_trait::MaybeCell;

use crate::{shapes::{nd::Plane, Shape}, Ray, Raytrace};

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
    pub fn new(center: [F; 3], diameters: [F; 4]) -> Self
    {
        Self {
            center,
            diameters
        }
    }
}

impl<F> Shape<F, 3> for Tetrahedron<F>
where
    F: Float + AddAssign + MulAssign
{
    fn raytrace<const N: bool>(&self, ray: &Ray<F, 3>) -> Raytrace<F, 3, N>
    where
        [(); N as usize]:
    {
        const V: [[f64; 3]; 4] = [
            [0.94280904158206336586779248280647, -1.0/3.0, 0.0],
            [-0.47140452079103168293389624140323, -1.0/3.0, 0.81649658092772603273242802490196],
            [-0.47140452079103168293389624140323, -1.0/3.0, -0.81649658092772603273242802490196],
            [0.0, 1.0, 0.0]
        ];

        let v = V.comap(self.diameters, |v, d| v.map(|v| F::from(v).unwrap()*d*F::from(0.5).unwrap()));

        let sv = [
            [v[0], v[1], v[2]],
            [v[1], v[2], v[3]],
            [v[2], v[3], v[0]],
            [v[3], v[0], v[1]],
        ];

        let s = sv.map(|v| Plane::new_from_vertices(v));
        
        let mut inside = true;

        for k in 0..4
        {
            let v1 = v[(k + 1) % 4];
            let v4 = v[(k) % 4];

            let n = s[(k + 1) % 4].n;
            let dv4 = n.mul_dot(v4.sub_each(v1));
            let dx = n.mul_dot(ray.r.sub_each(v1));

            if (dv4 >= F::zero()) != (dx >= F::zero())
            {
                inside = false;
                break;
            }
        }

        let mut t_min = F::infinity();
        let mut n_min = MaybeCell::from_fn(|| None);

        'lp:
        for (i, si) in s.into_iter()
            .enumerate()
        {
            let t = {
                let vn = ray.v.mul_dot(si.n);
                si.r.sub_each(ray.r).mul_dot(si.n)/vn
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
                let dv4 = n.mul_dot(v4.sub_each(v1));
                let dx = n.mul_dot(x.sub_each(v1));

                if (dv4 >= F::zero()) != (dx >= F::zero())
                {
                    continue 'lp;
                }
            }

            t_min = t;
            n_min = MaybeCell::from_fn(|| Some(si.n.normalize_to(if inside {F::one()} else {-F::one()})));
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
    use core::f64::consts::{FRAC_PI_2, FRAC_PI_4};

    use crate::{shapes::Transform, tests};

    use super::Tetrahedron;

    #[test]
    fn test()
    {
        let shape = Transform::new(Tetrahedron::new([0.0, 0.0, 0.0], [2.0, 2.0, 2.0, 2.0]))
            .rotate([1.0, 0.0, 0.0], -FRAC_PI_4)
            .rotate([0.0, 1.0, 0.0], FRAC_PI_4);

        const D: f64 = 1.0;
        const A: f64 = 0.0;

        tests::project_3d_spin(&shape, [0.0, 0.0, -3.0], D, A);
    }
}