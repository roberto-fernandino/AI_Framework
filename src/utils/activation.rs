#[derive(Clone)]
pub struct Activation<'a> {
    pub name: &'a str,
    pub function: &'a dyn Fn(f64) -> f64,
    pub derivative: &'a dyn Fn(f64) -> f64,
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}
pub const SIGMOID: Activation = Activation {
    name: "Sigmoid",
    function: &sigmoid,
    derivative: &|x| {
        let y = sigmoid(x);
        y * (1.0 - y)
    },
};

pub const RELU: Activation = Activation {
    name: "ReLU",
    function: &|x| x.max(0.0),
    derivative: &|x| if x > 0.0 { 1.0 } else { 0.0 },
};

pub const LEAKY_RELU: Activation = Activation {
    name: "Leaky ReLU",
    function: &|x| x.max(0.01 * x),
    derivative: &|x| if x > 0.0 { 1.0 } else { 0.01 },
};

pub const INPUT: Activation = Activation {
    name: "Input",
    function: &|x| x,
    derivative: &|x| x,
};
