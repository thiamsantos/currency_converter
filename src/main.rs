extern crate reqwest;
#[macro_use]
extern crate serde_derive;

use reqwest::Response;
use std::collections::HashMap;
use std::env;

#[derive(Deserialize)]
struct CurrencyRatesResponse {
    rates: HashMap<String, f64>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        panic!("You must provide three arguments: base, symbol and value");
    }

    let base = &args[1];
    let symbol = &args[2];
    let value = args[3].parse::<f64>().unwrap();

    let client = reqwest::Client::new();
    let body = client
        .get("https://api.exchangeratesapi.io/latest")
        .query(&[("base", base), ("symbols", symbol)])
        .send();

    match body {
        Ok(response) => handle_response(response, base, symbol, value),
        Err(err) => println!("error = {:?}", err),
    }
}

fn handle_response(mut response: Response, base: &str, symbol: &str, value: f64) {
    let rates: CurrencyRatesResponse = response.json().unwrap();
    let rate = rates.rates[symbol];
    let total = rate * value;

    println!("{:.2} {} worths {:.2} {}", value, base, total, symbol)
}
