use crate::shapes::nd::HyperCube;

pub type Cube<F> = HyperCube<F, 3>;

#[cfg(test)]
mod test
{
    use core::f64::consts::FRAC_PI_4;

    use crate::{shapes::Transform, tests};

    use super::Cube;

    #[test]
    fn test()
    {
        let shape = Transform::new(Cube {
                center: [0.0, 0.0, 0.0],
                radius: 1.0
            })
            .rotate([1.0, 0.0, 0.0], -FRAC_PI_4)
            .rotate([0.0, 1.0, 0.0], FRAC_PI_4);

        const D: f64 = 0.0;
        const A: f64 = 0.1;

        tests::project_3d_spin(&shape, [0.0, 0.0, -20.0], D, A);
    }
}