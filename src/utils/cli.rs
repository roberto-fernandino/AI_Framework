use crate::utils::network::{Layers, Network};
use rustyline::history::MemHistory;
use rustyline::{Config, Editor};
use std::{
    any::{type_name, Any},
    io::{self, Write},
};

use crate::utils::activation::{self, INPUT};

pub fn input_parser(input: String) {
    match input.as_str() {
        "create model" => create_model(),
        "exit" => {
            println!("Exiting...");
            std::process::exit(0);
        }
        "help" => help(),
        _ => println!("Invalid command"),
    }
}

pub fn logo() {
    println!("               __              __           ____                                ___                ");
    println!("   _________  / /_  ___  _____/ /_____     / __/__  _________  ____ _____  ____/ (_)___  ____      ");
    println!("  / ___/ __ \\/ __ \\/ _ \\/ ___/ __/ __ \\   / /_/ _ \\/ ___/ __ \\/ __ `/ __ \\/ __  / / __ \\/ __ \\     ");
    println!(" / /  / /_/ / /_/ /  __/ /  / /_/ /_/ /  / __/  __/ /  / / / / /_/ / / / / /_/ / / / / / /_/ /     ");
    println!("/_/   \\____/_.___/\\___/_/   \\__/\\____/  /_/   \\___/_/  /_/ /_/\\__,_/_/ /_/\\__,_/_/_/ /_/\\____/      ");
    println!("                                                                                                  ");
    println!("                         __        __                 _                                            ");
    println!("   ____ ___  ____ ______/ /_____  / /_   ____ _____ _(_)                                           ");
    println!("  / __ `__ \\/ __ `/ ___/ //_/ _ \\/ __/  / __ `/ __ `/ /                                            ");
    println!(" / / / / / / /_/ / /  / ,< /  __/ /_   / /_/ / /_/ / /                                             ");
    println!("/_/ /_/ /_/\\__,_/_/  /_/|_|\\___/\\__/   \\__,_/\\__, /_/                                              ");
    println!("                                            /____/                                                ");
    println!();
    println!();
    println!();
}

pub fn help() {
    println!("Commands:");
    println!("create model <name> <layers> <activation> - Creates a new model");
    println!("exit - Exits the program");
}

pub fn create_model() {
    print!("Enter the model name: ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    name.trim().to_string();

    print!("Enter the number of layers: ");
    io::stdout().flush().unwrap();
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let layers: usize = match input.trim().parse::<usize>() {
        Ok(n) => {
            if n < 3 {
                println!("Invalid number of layers must be bigger or eq  3");
                return;
            } else {
                n
            }
        }
        Err(_) => {
            println!("Invalid number of layers");
            return;
        }
    };
    let mut neurons_per_layer: Vec<usize> = vec![];
    let mut num: usize = 1;
    while num <= layers {
        print!("Enter the number of neurons in the {} layer: ", &num);
        io::stdout().flush().unwrap();
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<usize>() {
            Ok(n) => {
                neurons_per_layer.push(n);
                num += 1;
            }
            Err(_) => {
                println!("Invalid number of neurons");
                num -= 1;
            }
        }
    }
    let mut activations: Vec<activation::Activation> = vec![INPUT];
    let mut n: usize = 1;
    while n <= layers - 1 {
        print!("Enter the activation function for the {} layer: ", n + 1);
        io::stdout().flush().unwrap();
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "sigmoid" => {
                activations.push(activation::SIGMOID);
                n += 1;
            }
            "relu" => {
                activations.push(activation::RELU);
                n += 1;
            }
            "leaky-relu" => {
                activations.push(activation::LEAKY_RELU);
                n += 1;
            }
            _ => {
                println!("Invalid activation function");
                println!("Function not found\nAvailable activation functions:\nsigmoid, relu, leaky-relu");
            }
        }
    }
    let learning_rate: f64;
    loop {
        let mut input: String = String::new();
        print!("Enter the learning rate: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<f64>() {
            Ok(n) => {
                learning_rate = n;
                break;
            }
            Err(_) => {
                println!("Invalid learning rate");
                continue;
            }
        };
    }
    let layers = Layers::new(neurons_per_layer, activations);
    let network = Network::new(layers, learning_rate);
    println!("Model created");
    network.representation();
    network.ask_save(name);
}

pub fn mainloop() {
    logo();

    let mut rl = Editor::<(), MemHistory>::with_history(Config::default(), MemHistory::new())
        .expect("Error initializing readline");
    loop {
        let readline = rl.readline("command> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())
                    .expect("Failed to add history entry");
                input_parser(line);
            }
            Err(_) => {
                println!("Exiting...");
                break;
            }
        }
    }
}
