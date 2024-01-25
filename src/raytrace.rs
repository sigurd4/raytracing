use core::marker::ConstParamTy;

use num::Float;
use option_trait::MaybeCell;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Raytrace<F, const D: usize, const N: bool>
where
    F: Float,
    [(); N as usize]:,
{
    pub t: F,
    pub n: MaybeCell<Option<[F; D]>, N>
}

impl<F, const D: usize, const N: bool> Raytrace<F, D, N>
where
    F: Float,
    [(); N as usize]:,
{
    pub fn miss() -> Self
    {
        Self {
            t: F::infinity(),
            n: MaybeCell::from_fn(|| None)
        }
    }

    pub fn new(
        t: F,
        n: MaybeCell<Option<[F; D]>, N>
    ) -> Self
    {
        Self {
            t,
            n
        }
    }

    pub fn is_hit(&self) -> bool
    {
        self.t >= F::zero() && self.t.is_finite() && !self.n.get().is_some_and(|n| !n.is_some())
    }

    pub fn is_miss(&self) -> bool
    {
        !self.is_hit()
    }
}