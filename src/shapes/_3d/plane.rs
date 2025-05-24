use crate::shapes::nd::HyperPlane;

pub type Plane<F> = HyperPlane<F, 3>;

#[cfg(test)]
mod test
{
    use crate::tests;

    use super::Plane;

    #[test]
    fn test()
    {
        let shape = Plane {
            r: [0.0, 0.0, 0.0],
            n: [1.0, 1.0, 1.0]
        };

        const D: f64 = 1.0;
        const A: f64 = 0.1;

        tests::project_3d_spin(&shape, [0.0, 0.0, -1.0], D, A);
    }
}