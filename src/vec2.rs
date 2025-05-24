use num::Float;

pub fn magnitude_squared<F>(v: [F; 2]) -> F
where
    F: Float
{
    mul_dot(v, v)
}
pub fn magnitude<F>(v: [F; 2]) -> F
where
    F: Float
{
    magnitude_squared(v).sqrt()
}
pub fn sub<F>(lhs: [F; 2], rhs: [F; 2]) -> [F; 2]
where
    F: Float
{
    let [x_lhs, y_lhs] = lhs;
    let [x_rhs, y_rhs] = rhs;
    [x_lhs - x_rhs, y_lhs - y_rhs]
}
pub fn mul_dot<F>(lhs: [F; 2], rhs: [F; 2]) -> F
where
    F: Float
{
    let [x_lhs, y_lhs] = lhs;
    let [x_rhs, y_rhs] = rhs;
    x_lhs*x_rhs + y_lhs*y_rhs
}
pub fn normalize<F>(v: [F; 2]) -> [F; 2]
where
    F: Float
{
    let [x, y] = v;
    let m = magnitude(v);
    [x/m, y/m]
}