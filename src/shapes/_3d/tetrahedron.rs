use core::{f64::EPSILON, ops::{AddAssign, MulAssign, Range}};

use array_math::{ArrayMath, ArrayOps};
use num::Float;

use crate::{Ray, shapes::{nd::Plane, Shape}};

#[derive(Debug, Clone)]
pub struct Tetrahedron<F>
where
    F: Float
{
    pub center: [F; 3],
    pub diameter: F
}

impl<F> Tetrahedron<F>
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

impl<F> Shape<F, 3> for Tetrahedron<F>
where
    F: Float + AddAssign + MulAssign
{
    fn raytrace(&self, ray: &Ray<F, 3>) -> F
    {
        let eps = F::epsilon();

        const V: [[f64; 3]; 4] = [
            [0.47140452079103168293389624140323, 0.0, -1.0/3.0],
            [-0.47140452079103168293389624140323, 0.81649658092772603273242802490196, -1.0/3.0],
            [-0.47140452079103168293389624140323, -0.81649658092772603273242802490196, -1.0/3.0],
            [0.0, 0.0, 1.0]
        ];

        let radius = self.diameter*F::from(0.5).unwrap();

        let s: [_; V.len()] = ArrayOps::fill(|i| Plane::new_from_vertices(ArrayOps::fill(|k| 
            V[(i + k) % V.len()].map(|x| F::from(x).unwrap()*radius)
        )));

        let t = s.into_iter().raytrace(ray);

        let x = ray.propagate(t);

        if x.sub_each(self.center).magnitude_squared() > radius*radius + eps
        {
            return F::infinity()
        }

        t
    }

    fn raytrace_norm(&self, ray: &Ray<F, 3>) -> (F, Option<[F; 3]>)
    {
        const V: [[f64; 3]; 4] = [
            [0.47140452079103168293389624140323, 0.0, -1.0/3.0],
            [-0.47140452079103168293389624140323, 0.81649658092772603273242802490196, -1.0/3.0],
            [-0.47140452079103168293389624140323, -0.81649658092772603273242802490196, -1.0/3.0],
            [0.0, 0.0, 1.0]
        ];

        let radius = self.diameter*F::from(0.5).unwrap();
        let v = V.map(|v| v.map(|v| F::from(v).unwrap()*radius));

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
            let v2 = v[(k + 2) % 4];
            let v3 = v[(k + 3) % 4];
            let v4 = v[(k) % 4];

            let n = v2.sub_each(v1).mul_cross([&v3.sub_each(v1)]);
            let dv4 = n.mul_dot(v4.sub_each(v1));
            let dx = n.mul_dot(ray.r.sub_each(v1));

            if (dv4 >= F::zero()) != (dx >= F::zero())
            {
                inside = false;
                break;
            }
        }

        let mut t_min = F::infinity();
        let mut n_min = None;

        'lp:
        for (i, s) in s.into_iter()
            .enumerate()
        {
            let t = {
                let vn = ray.v.mul_dot(s.n);
                s.r.sub_each(ray.r).mul_dot(s.n)/vn
            };

            if t < F::zero() || t >= t_min
            {
                continue;
            }

            let x = ray.propagate(t);
            for k in 0..3
            {
                let v1 = v[(i + k + 1) % 4];
                let v2 = v[(i + k + 2) % 4];
                let v3 = v[(i + k + 3) % 4];
                let v4 = v[(i + k) % 4];

                let n = v2.sub_each(v1).mul_cross([&v3.sub_each(v1)]);
                let dv4 = n.mul_dot(v4.sub_each(v1));
                let dx = n.mul_dot(x.sub_each(v1));

                if (dv4 >= F::zero()) != (dx >= F::zero())
                {
                    continue 'lp;
                }
            }

            t_min = t;
            n_min = Some(s.n.normalize_to(if inside {F::one()} else {-F::one()}));
        }

        (t_min, n_min)
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
        let shape = Transform::new(Tetrahedron::new([0.0, 0.0, 0.0], 2.0))
            .rotate([1.0, 0.0, 0.0], -FRAC_PI_4)
            .rotate([0.0, 1.0, 0.0], FRAC_PI_4);

        const D: f64 = 1.0;
        const A: f64 = 0.0;

        tests::project_3d_spin(&shape, [0.0, 0.0, -3.0], D, A);
    }
}