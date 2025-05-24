use core::ops::{AddAssign, MulAssign};

use array_math::{ArrayMath, ArrayOps};
use num::Float;
use option_trait::MaybeCell;

use crate::{shapes::{nd::HyperPlane, Shape}, Ray, Raytrace};

#[derive(Debug, Clone, Copy)]
pub struct TriangularPlane<F>
where
    F: Float
{
    vertices: [[F; 3]; 3]
}

impl<F> Shape<F, 3> for TriangularPlane<F>
where
    F: Float + AddAssign + MulAssign
{
    fn raytrace(&self, ray: &Ray<F, 3>) -> Raytrace<F, 3, N>
    where
        [(); N as usize]:
    {
        let s = HyperPlane::new_from_vertices(self.vertices);

        let vn = ray.v.mul_dot(s.n);
        let t = s.r.sub_each(ray.r).mul_dot(s.n)/vn;

        if t < F::zero() || !t.is_finite()
        {
            return Raytrace::miss()
        }

        let x = ray.propagate(t);
        for k in 0..3
        {
            let a = self.vertices[k];
            let b = self.vertices[(k + 1) % 3];
            let c = self.vertices[(k + 2) % 3];
            let cb = c.sub_each(b);
            let ax = a.sub_each(x);
            let ab = a.sub_each(b);

            let ab_proj_cb = cb.normalize_to(cb.mul_dot(ab));
            let v = ab.sub_each(ab_proj_cb);
            
            if !(v.mul_dot(ax)/v.mul_dot(ab) > F::one())
            {
                return Raytrace::miss();
            }
        }

        Raytrace {
            t,
            n: MaybeCell::from_fn(|| Some(s.n.normalize_to(vn.signum())))
        }
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::FRAC_PI_4;

    use crate::{shapes::Transform, tests};

    use super::TriangularPlane;

    #[test]
    #[ignore]
    fn test()
    {
        let shape = Transform::new(TriangularPlane::new([
            [0.94280904158206336586779248280647, 0.0, 0.0],
            [-0.47140452079103168293389624140323, 0.0, 0.81649658092772603273242802490196],
            [-0.47140452079103168293389624140323, 0.0, -0.81649658092772603273242802490196]
        ]))
            .rotate([1.0, 0.0, 0.0], -FRAC_PI_4)
            .rotate([0.0, 1.0, 0.0], FRAC_PI_4);

        const D: f64 = 0.5;
        const A: f64 = 0.0;

        tests::project_3d_spin(&shape, [0.0, 0.0, -3.0], D, A);
    }
}