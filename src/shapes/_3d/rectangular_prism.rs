use crate::shapes::nd::HyperRectangle;

pub type RectangularPrism<F> = HyperRectangle<F, 3>;

#[cfg(test)]
mod test
{
    use core::f64::consts::FRAC_PI_4;

    use crate::{shapes::Transform, tests};

    use super::RectangularPrism;

    #[test]
    #[ignore]
    fn test()
    {
        let shape = Transform::new(RectangularPrism {
            c1: [-2.0, -1.0, -1.0],
            c2: [2.0, 1.0, 1.0]
        }).rotate([1.0, 0.0, 0.0], -FRAC_PI_4)
        .rotate([0.0, 1.0, 0.0], FRAC_PI_4);

        const D: f64 = 3.0;
        const A: f64 = 0.0;

        tests::project_3d_spin(&shape, [0.0, 0.0, -3.0], D, A);
    }
}