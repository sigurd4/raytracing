use core::ops::{Deref, DerefMut};

use num::Float;

use super::Raytrace;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RaytraceWithNorm<F, const D: usize>
where
    F: Float
{
    pub raytrace: Raytrace<F, D>,
    pub n: Option<[F; D]>
}

impl<F, const D: usize> Deref for RaytraceWithNorm<F, D>
where
    F: Float
{
    type Target = Raytrace<F, D>;

    fn deref(&self) -> &Self::Target
    {
        &self.raytrace
    }
}
impl<F, const D: usize> DerefMut for RaytraceWithNorm<F, D>
where
    F: Float
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.raytrace
    }
}

impl<F, const D: usize> PartialOrd for RaytraceWithNorm<F, D>
where
    F: Float
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering>
    {
        self.raytrace.partial_cmp(&other.raytrace)
    }
}

impl<F, const D: usize> RaytraceWithNorm<F, D>
where
    F: Float
{
    pub fn miss() -> Self
    {
        Self {
            raytrace: Raytrace::miss(),
            n: None
        }
    }

    pub fn is_hit(&self) -> bool
    {
        self.without_norm().is_hit() && self.n.is_some()
    }

    pub fn is_miss(&self) -> bool
    {
        !self.is_hit()
    }

    pub fn without_norm(&self) -> Raytrace<F, D>
    {
        let Self {raytrace, n: _} = self;   
        *raytrace
    }

    pub fn map_norm(self, map: impl FnOnce([F; D]) -> [F; D]) -> RaytraceWithNorm<F, D>
    {
        let Self {raytrace, n} = self;
        RaytraceWithNorm {
            raytrace,
            n: n.map(map)
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