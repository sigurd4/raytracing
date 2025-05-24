use core::ops::BitOr;

use num::Float;

use super::RaytraceWithNorm;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Raytrace<F, const D: usize>
where
    F: Float
{
    pub t: F
}

impl<F, const D: usize> BitOr for Raytrace<F, D>
where
    F: Float
{
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output
    {
        if self.t < rhs.t {self} else {rhs}
    }
}

impl<F, const D: usize> Raytrace<F, D>
where
    F: Float
{
    pub fn miss() -> Self
    {
        Self {
            t: F::infinity()
        }
    }

    pub fn is_hit(&self) -> bool
    {
        self.t >= F::zero() && self.t.is_finite()
    }

    pub fn is_miss(&self) -> bool
    {
        !self.is_hit()
    }

    pub fn with_norm(self, norm: impl FnOnce() -> [F; D]) -> RaytraceWithNorm<F, D>
    {
        RaytraceWithNorm {
            raytrace: self,
            n: match self.is_hit()
            {
                true => Some(norm()),
                false => None
            }
        }
    }

    pub fn min(self, rhs: Self) -> Self
    {
        if self > rhs
        {
            rhs
        }
        else
        {
            self
        }
    }

    pub fn max(self, rhs: Self) -> Self
    {
        if self < rhs
        {
            rhs
        }
        else
        {
            self
        }
    }
}