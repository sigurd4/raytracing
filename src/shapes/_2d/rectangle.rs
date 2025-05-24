use crate::shapes::nd::HyperRectangle;

pub type Rectangle<F> = HyperRectangle<F, 2>;

#[cfg(test)]
mod test
{
    use super::Rectangle;

    #[test]
    fn test()
    {
        let shape = Rectangle {
            c1: [-1.0, -1.0],
            c2: [1.0, 1.0]
        };

        const D: f64 = 2.0;
        const A: f64 = 0.0;
    }
}