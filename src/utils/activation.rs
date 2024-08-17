#[derive(Clone)]
pub struct Activation<'a> {
    pub function: &'a dyn Fn(f64) -> f64,
    pub derivative: &'a dyn Fn(f64) -> f64,
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}
pub const SIGMOID: Activation = Activation {
    function: &sigmoid, // Sigmoid function
    derivative: &|x| {
        let y = sigmoid(x);
        y * (1.0 - y) // Sigmoid derivative
    },
};
