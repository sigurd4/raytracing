use array_math::ArrayOps;
use num::Float;

#[derive(Debug, Clone, Copy)]
pub struct Ray<F, const D: usize>
where
    F: Float
{
    pub r: [F; D],
    pub v: [F; D]
}

impl<F, const D: usize> Ray<F, D>
where
    F: Float
{
    pub const fn new(r: [F; D], v: [F; D]) -> Self
    {
        Self {
            r,
            v
        }
    }

    pub fn new_from_to(r_from: [F; D], r_to: [F; D]) -> Self
    {
        Self::new(r_from, r_to.sub_each(r_from))
    }

    pub fn propagate(&self, t: F) -> [F; D]
    {
        self.r.add_each(self.v.mul_all(t))
    }
}