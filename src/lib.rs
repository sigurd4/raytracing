#![cfg_attr(not(test), no_std)]
#![feature(array_methods)]
#![feature(generic_arg_infer)]
#![feature(more_float_constants)]
#![feature(let_chains)]
#![feature(associated_type_bounds)]

#![feature(generic_const_exprs)]

use array_math::ArrayOps;
use num::Float;

moddef::moddef!(
    pub mod {
        shapes
    }
);

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

#[cfg(test)]
mod tests {
    use core::f64::consts::{FRAC_1_SQRT_2, FRAC_1_SQRT_3, SQRT_2, SQRT_3, TAU};
    use std::time::SystemTime;

    use array_math::ArrayMath;
    use image::Rgb;

    use self::shapes::{Shape, Transform};

    use super::*;

    pub fn project_3d_spin<S>(shape: &S, lens_pos: [f64; 3], lens_size: f64, lens_bend: f64)
    where
        S: Shape<f64, 3> + Clone
    {
        const SPIN_SPEED: f64 = TAU*0.02;

        let t0 = SystemTime::now();
        loop
        {
            let t = SystemTime::now();
            let dt = t.duration_since(t0).unwrap().as_secs_f64();

            let shape = Transform::new(shape.clone())
                .rotate([0.0, 1.0, 0.0], SPIN_SPEED*dt);

            project_3d_once(&shape, lens_pos, lens_size, lens_bend);
        }
    }

    pub fn project_3d_once<S>(shape: &S, lens_pos: [f64; 3], lens_size: f64, lens_bend: f64)
    where
        S: Shape<f64, 3>
    {
        const N: usize = 256;

        let t: Box<[Box<[_; N]>; N]> = ArrayOps::fill_boxed(|y| ArrayOps::fill_boxed(|x| {

            let x = x as f64/(N - 1) as f64*2.0 - 1.0;
            let y = y as f64/(N - 1) as f64*2.0 - 1.0;

            let ray = Ray::new(
                [
                    x*lens_size,
                    y*lens_size,
                    0.0
                ].add_each(lens_pos),
                [
                    x*lens_bend,
                    y*lens_bend,
                    1.0
                ].normalize()
            );

            shape.raytrace_norm(&ray)
        }));

        let t_min = t.iter()
            .flat_map(|t| t.iter().map(|(t, _)| t))
            .filter(|&t| t.is_finite())
            .map(|&t| t)
            .reduce(f64::min)
            .unwrap_or(0.0);
        let t_max = t.iter()
            .flat_map(|t| t.iter().map(|(t, _)| t))
            .filter(|&t| t.is_finite())
            .map(|&t| t)
            .reduce(f64::max)
            .unwrap_or(1.0);

        const BACKGROUND: Rgb<u8> = Rgb([255, 255, 255]);

        const DIR_RED: [f64; 3] = [-FRAC_1_SQRT_3, -FRAC_1_SQRT_3, -FRAC_1_SQRT_3];
        const DIR_GREEN: [f64; 3] = [-FRAC_1_SQRT_3, -FRAC_1_SQRT_3, FRAC_1_SQRT_3];
        const DIR_BLUE: [f64; 3] = [FRAC_1_SQRT_2, -FRAC_1_SQRT_2, 0.0];

        image::RgbImage::from_fn(N as u32, N as u32, |x, y| {
            let (t, n) = t[y as usize][x as usize];
            if t.is_finite() && let Some(n) = n
            {
                let t_norm = (t - t_min)/(t_max - t_min);
                let l = 1.0 - t_norm*0.1;

                let r = l*(DIR_RED.mul_dot(n)*0.5 + 0.5);
                let g = l*(DIR_GREEN.mul_dot(n)*0.5 + 0.5);
                let b = l*(DIR_BLUE.mul_dot(n)*0.5 + 0.5);
                
                Rgb([(r*255.0) as u8, (g*255.0) as u8, (b*255.0) as u8])
            }
            else
            {
                BACKGROUND
            }
        }).save("image.png").unwrap();
    }

    #[test]
    fn it_works()
    {

    }
}
