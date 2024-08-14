#[derive(Clone)]
pub struct Activation<'a> {
    pub function: &'a dyn Fn(f64) -> f64,
    pub derivative: &'a dyn Fn(f64) -> f64,
}
pub const SIGMOID: Activation = Activation {
    function: &|x| 1.0 / (1.0 + (-x).exp()),
    derivative: &|x| x * (1.0 - x),
};
