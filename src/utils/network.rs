use crate::utils::activation::Activation;
use crate::utils::matrix::Matrix;
use std::vec;

pub struct Layers<'a> {
    pub layers_vec: Vec<usize>,
    pub activation_vec: Vec<Activation<'a>>,
}

impl Layers<'_> {
    pub fn new(layers_vec: Vec<usize>, activation: Vec<Activation<'_>>) -> Layers<'_> {
        Layers {
            layers_vec: layers_vec,
            activation_vec: activation,
        }
    }
}
pub struct Network<'a> {
    pub layers: Layers<'a>,
    pub weights: Vec<Matrix>,
    pub biases: Vec<Matrix>,
    pub data: Vec<Matrix>,
    pub learning_rate: f64,
}

impl Network<'_> {
    pub fn new<'a>(layers: Layers<'a>, learning_rate: f64) -> Network<'a> {
        // Start the weights and biases
        let mut weights = Vec::new();
        let mut biases = Vec::new();
        let layers_size = layers.layers_vec.len();
        for layer in 0..layers_size - 1 {
            // Create the weights matrix for every layer where the number of columns is the number of neurons in the actual
            // layer and the number of rows is the number of neurons in the next layer
            weights.push(Matrix::random_less_one_to_one(
                layers.layers_vec[layer + 1],
                layers.layers_vec[layer],
            ));

            biases.push(Matrix::random_less_one_to_one(
                layers.layers_vec[layer + 1],
                1,
            ));
        }
        Network {
            layers: layers,
            weights: weights,
            biases: biases,
            data: Vec::new(),
            learning_rate: learning_rate,
        }
    }

    pub fn feed_forward(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        if inputs.len() != self.layers.layers_vec[0] {
            panic!("Input must have the same size as the first layer");
        }
        // Current input to the network
        let mut current = Matrix::from(vec![inputs]).transpose();
        // Save the current input to the network
        self.data = vec![current.clone()];
        // Loop through the layers
        for layer in 0..self.layers.layers_vec.len() - 1 {
            current = self.weights[layer]
                .multiply(&current) // Multiply the inputs by the weights
                .add(&self.biases[layer]) // add the biases
                .map(self.layers.activation_vec[layer].function); // apply the activation function
            self.data.push(current.clone()); // Save the current input to the network
        }
        current.transpose().data[0].to_owned() // Transpose the matrix and return the first element
    }

    pub fn back_propagation(&mut self, outputs: Vec<f64>, targets: Vec<f64>) {
        if targets.len() != self.layers.layers_vec[self.layers.layers_vec.len() - 1] {
            panic!("Targets must have the same size as the last layer");
        }
        let parsed = Matrix::from(vec![outputs]); // Parse outputs to a matrix
        let mut errors = Matrix::from(vec![targets]).subtract(&parsed); // Subtract targets from outputs and save to errors
        let mut gradients: Matrix =
            parsed.map(&self.layers.activation_vec.last().unwrap().derivative); // Calculate gradients
        for i in (0..self.layers.layers_vec.len() - 1).rev() {
            gradients = gradients
                .dot_multiply(&errors)
                .map(&|x| x * self.learning_rate);
            self.weights[i] = self.weights[i].add(&gradients.multiply(&self.data[i].transpose()));
            self.biases[i] = self.biases[i].add(&gradients);
            errors = self.weights[i].transpose().multiply(&errors);
            gradients = self.data[i].map(self.layers.activation_vec[i].derivative);
        }
    }

    pub fn train(&mut self, inputs: Vec<Vec<f64>>, targets: Vec<Vec<f64>>, epochs: u64) {
        // Train the network
        // epochs is the number of times the network will be trained
        for run in 1..=epochs {
            // Loop through the epochs
            if epochs < 100 || run % (epochs / 10) == 0 {
                // Print the epoch number every 1% of the epochs
                println!("run {} of {}", run, epochs);
            }
            for i in 0..inputs.len() {
                // for each input
                let outputs = self.feed_forward(inputs[i].clone()); // feed forward
                self.back_propagation(outputs, targets[i].clone()); // backpropagation
            }
        }
    }
}
