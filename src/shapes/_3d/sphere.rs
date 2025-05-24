use crate::shapes::nd::HyperSphere;

pub type Sphere<F> = HyperSphere<F, 3>;

#[cfg(test)]
mod test
{
    use crate::{shapes::Transform, tests};

    use super::Sphere;

    #[test]
    #[ignore]
    fn test()
    {
        let shape = Transform::new(Sphere::new([0.0, 0.0, 0.0], 1.0))
            .scale([2.0, 2.0, 2.0]);

        const D: f64 = 3.0;
        const A: f64 = 0.0;

        tests::project_3d_spin(&shape, [0.0, 0.0, -5.0], D, A);
    }
}