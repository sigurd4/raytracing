use core::iter::Sum;

use num::Float;

mod private
{
    use num::{traits::{ConstOne, ConstZero}, Float};

    #[const_trait]
    pub trait IdentitySpec: Float
    {
        fn _identity<const N: usize>() -> [[Self; N]; N];
    }
    impl<F> IdentitySpec for F
    where
        F: Float
    {
        default fn _identity<const N: usize>() -> [[Self; N]; N]
        {
            let mut matrix = [[F::zero(); N]; N];
            let one = F::one();
            for dst in matrix.as_flattened_mut()
                .iter_mut()
                .step_by(N + 1)
            {
                *dst = one
            }
            matrix
        }
    }
    impl<F> const IdentitySpec for F
    where
        F: Float + ConstOne + ConstZero
    {
        fn _identity<const N: usize>() -> [[Self; N]; N]
        {
            let mut matrix = [[F::ZERO; N]; N];
            let mut i = 0;
            while i < N
            {
                matrix[i][i] = F::ONE;
                i += 1
            }
            matrix
        }
    }
}

pub fn diagonal<F, const N: usize>(diagonal: [F; N]) -> [[F; N]; N]
where
    F: Float
{
    let mut matrix = [[F::zero(); N]; N];
    for (dst, src) in matrix.as_flattened_mut()
        .iter_mut()
        .step_by(N + 1)
        .zip(diagonal)
    {
        *dst = src
    }
    matrix
}

pub const fn identity<F, const N: usize>() -> [[F; N]; N]
where
    F: Float + ~const private::IdentitySpec
{
    F::_identity()
}

pub fn mul_matrix_collumn<F, const M: usize, const N: usize>(lhs: [[F; N]; M], rhs: [F; N]) -> [F; M]
where
    F: Float + Sum
{
    lhs.map(|lhs| lhs.into_iter()
        .zip(rhs)
        .map(|(lhs, rhs)| lhs*rhs)
        .sum()
    )
}

pub fn mul_matrix_matrix<F, const M: usize, const N: usize, const P: usize>(lhs: &[[F; N]; M], rhs: &[[F; P]; N]) -> [[F; P]; M]
where
    F: Float + Sum
{
    unsafe {
        lhs.iter()
            .map(|lhs| (0..P).map(|i| lhs.iter()
                    .zip(rhs)
                    .map(|(&lhs, rhs)| lhs*rhs[i])
                    .sum::<F>()
                ).next_chunk()
                .unwrap_unchecked()
            ).next_chunk()
            .unwrap_unchecked()
    }
}