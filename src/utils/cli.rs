use crate::utils::activation::{self, INPUT};
use crate::utils::data::load_csv;
use crate::utils::network::{Layers, Network};
use csv::Writer;
use rustyline::error::ReadlineError;
use rustyline::history::MemHistory;
use rustyline::{Config, Editor};
use std::error::Error;
use std::fs;
use std::io::{self, Write};

pub struct LoadedModel {
    pub name: String,
    pub network: Network<'static>,
}

pub fn input_parser(input: String, network: &mut Option<LoadedModel>) {
    match input.as_str() {
        "create model" => create_network(),
        "exit" => {
            println!("Exiting...");
            std::process::exit(0);
        }
        "help" => help(),
        "clear" => clear(),
        "load model" => {
            if let Some(ref mut new_network) = network {
                *new_network = load_network().unwrap();
            } else {
                *network = load_network();
            }
        }
        "represent model" => {
            if let Some(ref network) = network {
                println!();
                network.network.representation();
            } else {
                println!("No model selected");
            }
        }
        "load csv" => {
            load_csv();
        }
        "list models" => list_models(),
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
    println!("help - show all commands");
}

pub fn help() {
    println!();
    println!("Commands:");
    println!("create model - Creates a new model");
    println!("load model - Loads a model");
    println!("clear - clears the screen");
    println!("exit - Exits the program");
    println!("help - shows this help");
    println!("represent model - Represents the model");
    println!("list models - Lists all models");
    println!();
}

pub fn create_network() {
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

pub fn load_network() -> Option<LoadedModel> {
    print!("Enter the model name: ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    name.trim().to_string();

    for entry in fs::read_dir("models").unwrap() {
        let entry = entry.unwrap();
        if entry
            .file_name()
            .to_str()
            .unwrap()
            .eq(format!("{}.json", name.trim()).as_str())
        {
            let network =
                Network::load_model(format!("{}", entry.path().to_str().unwrap().to_string()));
            let network = network.unwrap();
            println!("Model loaded");
            network.representation();
            let loaded_model = LoadedModel {
                name: name.trim().to_string(),
                network: network,
            };
            return Some(loaded_model);
        }
    }
    println!("Model not found");
    None
}

pub fn mainloop() {
    logo();
    let mut selected_network: Option<LoadedModel> = None;
    let mut rl = Editor::<(), MemHistory>::with_history(Config::default(), MemHistory::new())
        .expect("Error initializing readline");
    loop {
        let readline: Result<String, ReadlineError>;
        if selected_network.is_none() {
            readline = rl.readline("command> ");
        } else {
            let prompt = format!(
                "{} selected\ncommand> ",
                selected_network.as_ref().unwrap().name.as_str(),
            );
            readline = rl.readline(&prompt.as_str());
        }
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())
                    .expect("Failed to add history entry");
                input_parser(line, &mut selected_network);
            }
            Err(_) => {
                println!("Exiting...");
                break;
            }
        }
    }
}

pub fn clear() {
    print!("\x1b[2J\x1b[1;1H");
    io::stdout().flush().unwrap();
    println!();
    println!();
    print!("comamand>");
    io::stdout().flush().unwrap();
}

pub fn list_models() {
    for entry in fs::read_dir("models").unwrap() {
        let entry = entry.unwrap();
        if entry.file_name().to_str().unwrap().contains(".json") {
            println!(
                "{}",
                entry
                    .file_name()
                    .to_str()
                    .unwrap()
                    .split(".")
                    .collect::<Vec<&str>>()[0]
            );
        }
    }
    println!();
}
