#![cfg_attr(not(test), no_std)]
#![feature(more_float_constants)]
#![feature(let_chains)]
#![feature(const_trait_impl)]
#![feature(iter_next_chunk)]
#![feature(slice_as_array)]
#![feature(specialization)]

moddef::moddef!(
    pub mod {
        shapes,
        matrix,
        vec2,
        vec3
    },
    flat(pub) mod {
        ray,
        raytrace
    }
);

#[cfg(test)]
mod tests {
    use core::f64::consts::{FRAC_1_SQRT_2, FRAC_1_SQRT_3, TAU};
    use std::time::SystemTime;

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

        let [lens_x, lens_y, lens_z] = lens_pos;

        let t: Box<[[_; N]; N]> = (0..N).map(|y| core::array::from_fn(|x| {
                let x = x as f64/(N - 1) as f64*2.0 - 1.0;
                let y = 1.0 - y as f64/(N - 1) as f64*2.0;

                let ray = Ray {
                    r: [
                        x*lens_size + lens_x,
                        y*lens_size + lens_y,
                        lens_z
                    ],
                    v: vec3::normalize([
                        x*lens_bend,
                        y*lens_bend,
                        1.0
                    ])
                };

                shape.raytrace_with_norm(&ray)
            })).collect::<Vec<_>>()
            .into_boxed_slice()
            .into_array::<N>()
            .unwrap();

        /*let t_min = t.iter()
            .flat_map(|t| t.iter().map(|raytrace| raytrace.t))
            .filter(|t| t.is_finite())
            .reduce(f64::min)
            .unwrap_or(0.0);
        let t_max = t.iter()
            .flat_map(|t| t.iter().map(|raytrace| raytrace.t))
            .filter(|t| t.is_finite())
            .reduce(f64::max)
            .unwrap_or(1.0);*/

        const BACKGROUND: Rgb<u8> = Rgb([255, 255, 255]);

        const DIR_RED: [f64; 3] = [-FRAC_1_SQRT_3, -FRAC_1_SQRT_3, -FRAC_1_SQRT_3];
        const DIR_GREEN: [f64; 3] = [-FRAC_1_SQRT_3, -FRAC_1_SQRT_3, FRAC_1_SQRT_3];
        const DIR_BLUE: [f64; 3] = [FRAC_1_SQRT_2, -FRAC_1_SQRT_2, 0.0];

        const BRIGHTNESS: f64 = 100.0;

        image::RgbImage::from_fn(N as u32, N as u32, move |x, y| {
            let raytrace = t[y as usize][x as usize];
            if raytrace.t.is_finite() && let Some(n) = raytrace.n
            {
                let l = (-raytrace.t/BRIGHTNESS).exp();

                let r = l*(vec3::mul_dot(DIR_RED, n)*0.5 + 0.5);
                let g = l*(vec3::mul_dot(DIR_GREEN, n)*0.5 + 0.5);
                let b = l*(vec3::mul_dot(DIR_BLUE, n)*0.5 + 0.5);
                
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
