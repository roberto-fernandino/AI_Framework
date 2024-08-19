mod utils;
use utils::activation::{RELU, SIGMOID};
use utils::network::{Layers, Network};

fn main() {
    let inputs = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];
    let targets = vec![vec![0.0], vec![1.0], vec![1.0], vec![0.0]];
    let layers = Layers::new(vec![2, 16, 16, 1], vec![RELU, RELU, RELU, SIGMOID]);
    let mut network = Network::new(layers, 0.01);
    network.representation();

    println!("{:?}", network.feed_forward(inputs[0].clone()));
    println!("{:?}", network.feed_forward(inputs[1].clone()));
    println!("{:?}", network.feed_forward(inputs[2].clone()));
    println!("{:?}", network.feed_forward(inputs[3].clone()));

    network.train(inputs.clone(), targets.clone(), 50000);

    println!("{:?}", network.feed_forward(inputs[0].clone()));
    println!("{:?}", network.feed_forward(inputs[1].clone()));
    println!("{:?}", network.feed_forward(inputs[2].clone()));
    println!("{:?}", network.feed_forward(inputs[3].clone()));
}
