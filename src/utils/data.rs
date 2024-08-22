use std::{io, io::Write};

struct SwingTradeData {
    ticker: String,
    date: f64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

pub fn load_csv() {
    print!("Enter the file name> ");
    io::stdout().flush().unwrap();
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let file_name = input.trim();
    let file_path = format!("data/{}.csv", file_name);

    let mut rdr = match csv::Reader::from_path(&file_path) {
        Ok(rdr) => rdr,
        Err(e) => {
            println!("Error reading file");
            println!("{}", e);
            return;
        }
    };
    let mut swing_trade_data: Vec<SwingTradeData> = vec![];
}
