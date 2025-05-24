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
    pub fn new_from_to(r_from: [F; D], r_to: [F; D]) -> Self
    {
        Self {
            r: r_from,
            v: unsafe {
                r_to.into_iter()
                    .zip(r_from)
                    .map(|(r_to, r_from)| r_to - r_from)
                    .next_chunk()
                    .unwrap_unchecked()
            }
        }
    }

    pub fn propagate(&self, t: F) -> [F; D]
    {
        unsafe {
            self.r.into_iter()
                .zip(self.v)
                .map(|(r, v)| r + v*t)
                .next_chunk()
                .unwrap_unchecked()
        }
    }

    pub fn r_to(&self) -> [F; D]
    {
        unsafe {
            self.r.into_iter()
                .zip(self.v)
                .map(|(r, v)| r + v)
                .next_chunk()
                .unwrap_unchecked()
        }
    }
}