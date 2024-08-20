use crate::utils::activation::{Activation, LEAKY_RELU, RELU, SIGMOID};
use crate::utils::matrix::Matrix;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};
use std::{
    fs::File,
    io::{self, Read, Write},
    vec,
};

#[derive(Serialize, Deserialize)]
struct SaveData {
    biases: Vec<Vec<Vec<f64>>>,
    layers: Vec<usize>,
    layers_activation: Vec<String>,
    name: String,
    weights: Vec<Vec<Vec<f64>>>,
}
pub struct Layers<'a> {
    pub layers_vec: Vec<usize>,
    pub activation_vec: Vec<Activation<'a>>,
}

impl Layers<'_> {
    pub fn new(layers_vec: Vec<usize>, activation: Vec<Activation>) -> Layers {
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
    pub acurracy: f64,
}

impl Network<'_> {
    pub fn new(layers: Layers, learning_rate: f64) -> Network {
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
            acurracy: 0.0,
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
        let outputs = Matrix::from(vec![outputs]).transpose();
        let targets = Matrix::from(vec![targets]).transpose();
        let mut errors = targets.subtract(&outputs);
        let mut gradients: Matrix =
            outputs.map(&self.layers.activation_vec.last().unwrap().derivative); // Calculate gradients
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
        let mut total_loss = 0.0;
        let mut correct_predictions = 0;
        let mut total_predictions = 0;

        for epoch in 1..=epochs {
            for (input, target) in inputs.iter().zip(targets.iter()) {
                let outputs = self.feed_forward(input.clone());

                let loss = outputs
                    .iter()
                    .zip(target)
                    .map(|(o, t)| (o - t).powi(2))
                    .sum::<f64>()
                    / outputs.len() as f64;

                total_loss += loss;
                total_predictions += 1;

                // if ratio is in 90% to 100% of the target
                if outputs.iter().zip(target).all(|(o, t)| {
                    if t.abs() > 1e-6 {
                        // Evita divisão por zero ou muito próxima de zero
                        let ratio = o.abs() / t.abs();
                        ratio >= 0.9 && ratio <= 1.0
                    } else {
                        o.abs() < 1e-6 // Considera correto se ambos forem próximos de zero
                    }
                }) {
                    correct_predictions += 1;
                }
                self.back_propagation(outputs, target.clone());
            }

            let avg_loss = total_loss / total_predictions as f64;
            self.acurracy = correct_predictions as f64 / total_predictions as f64;

            // every 10%
            // prints the loss and accuracy every 10% of the epochs

            let ten_percent = (epochs as f64 * 0.1) as u64;
            if epoch % ten_percent as u64 == 0 {
                println!(
                    "Epoch: {} Loss: {} Accuracy: {}",
                    epoch, avg_loss, self.acurracy
                );
            }
        }
    }

    pub fn representation(&self) {
        println!("learning rate {}", self.learning_rate);
        for layer in 0..self.layers.layers_vec.len() {
            for _neuron in 0..self.layers.layers_vec[layer] {
                if layer == 0 {
                    print!("[input] ");
                } else {
                    print!("[{}] ", self.layers.activation_vec[layer].name);
                }
            }
            println!(); // Move to the next line for the next layer
        }
        println!();
    }
    pub fn ask_save(&self, name: String) {
        println!("Want to save the model? (y/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" {
            self.save_model(format!("models/{}.json", name.trim()), name.trim());
            println!("Model saved");
        } else {
            println!("Model not saved");
        }
    }
    pub fn save_model(&self, file: String, name: &str) {
        let mut file = File::create(file).expect("Unable to touch save file");

        file.write_all(
			json!({
                "name": name,
				"weights": self.weights.clone().into_iter().map(|matrix| matrix.data).collect::<Vec<Vec<Vec<f64>>>>(),
				"biases": self.biases.clone().into_iter().map(|matrix| matrix.data).collect::<Vec<Vec<Vec<f64>>>>(),
                "layers": self.layers.layers_vec.clone(),
                "layers_activation": self.layers.activation_vec.clone().into_iter().map(|activation| activation.name.to_string()).collect::<Vec<String>>(),
			}).to_string().as_bytes(),
		).expect("Unable to write to save file");
    }

    pub fn load_model<'a>(path: String) -> Result<Network<'a>, String> {
        let mut file = File::open(path).expect("Unable to open model file");
        let mut buffer = String::new();
        let mut weights: Vec<Vec<Vec<f64>>> = vec![];
        let mut biases: Vec<Vec<Vec<f64>>> = vec![];
        let mut layers: Vec<usize> = vec![];

        file.read_to_string(&mut buffer)
            .expect("Unable to read model file");

        let data: SaveData = from_str(&buffer).expect("Unable to parse model file");

        data.weights.iter().for_each(|weight| {
            weights.push(weight.clone());
        });
        data.biases.iter().for_each(|bias| {
            biases.push(bias.clone());
        });
        data.layers.iter().for_each(|layer| {
            layers.push(*layer);
        });
        let weights_matrix: Vec<Matrix> = data
            .weights
            .iter()
            .map(|weights| Matrix::from(weights.clone()))
            .collect();
        let biases_matrix: Vec<Matrix> = data
            .biases
            .iter()
            .map(|bias| Matrix::from(bias.clone()))
            .collect();

        let activation_vec: Vec<Activation> = data
            .layers_activation
            .iter()
            .map(|activation| match activation.as_str() {
                "ReLU" => Ok(RELU),
                "Sigmoid" => Ok(SIGMOID),
                "Leaky ReLU" => Ok(LEAKY_RELU),
                _ => return Err(format!("Invalid activation function: {}", activation)),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Network {
            layers: Layers {
                layers_vec: layers,
                activation_vec: activation_vec,
            },
            weights: weights_matrix,
            biases: biases_matrix,
            data: vec![],
            learning_rate: 0.01,
            acurracy: 0.0,
        })
    }
}
