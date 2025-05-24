use num::Float;

pub fn magnitude_squared<F>(v: [F; 3]) -> F
where
    F: Float
{
    mul_dot(v, v)
}
pub fn magnitude<F>(v: [F; 3]) -> F
where
    F: Float
{
    magnitude_squared(v).sqrt()
}
pub fn sub<F>(lhs: [F; 3], rhs: [F; 3]) -> [F; 3]
where
    F: Float
{
    let [x_lhs, y_lhs, z_lhs] = lhs;
    let [x_rhs, y_rhs, z_rhs] = rhs;
    [x_lhs - x_rhs, y_lhs - y_rhs, z_lhs - z_rhs]
}
pub fn mul_dot<F>(lhs: [F; 3], rhs: [F; 3]) -> F
where
    F: Float
{
    let [x_lhs, y_lhs, z_lhs] = lhs;
    let [x_rhs, y_rhs, z_rhs] = rhs;
    x_lhs*x_rhs + y_lhs*y_rhs + z_lhs*z_rhs
}
pub fn normalize<F>(v: [F; 3]) -> [F; 3]
where
    F: Float
{
    let [x, y, z] = v;
    let m = magnitude(v);
    [x/m, y/m, z/m]
}
pub fn mul_cross<F>(lhs: [F; 3], rhs: [F; 3]) -> [F; 3]
where
    F: Float
{
    let [x_lhs, y_lhs, z_lhs] = lhs;
    let [x_rhs, y_rhs, z_rhs] = rhs;
    [
        y_lhs*z_rhs - z_lhs*y_rhs,
        z_lhs*x_rhs - x_lhs*z_rhs,
        x_lhs*y_rhs - y_lhs*x_rhs
    ]
}