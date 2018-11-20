extern crate reqwest;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::env;

struct Arguments<'a> {
    base: &'a str,
    symbol: &'a str,
    value: f64,
}

#[derive(Deserialize)]
struct CurrencyRatesResponse {
    rates: HashMap<String, f64>,
}

struct CurrencyRate<'a> {
    base: &'a str,
    symbol: &'a str,
    rate: f64,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let parsed_arguments = parse_arguments(&args);
    let currency_rate = fetch_rate(&parsed_arguments);
    let currency_value = calculate_currency_value(parsed_arguments.value, currency_rate.rate);
    show_report(&parsed_arguments, &currency_rate, currency_value);
}

fn parse_arguments(args: &[String]) -> Arguments {
    Arguments {
        base: &args[1],
        symbol: &args[2],
        value: args[3].parse::<f64>().unwrap(),
    }
}

fn fetch_rate<'a>(arguments: &'a Arguments) -> CurrencyRate<'a> {
    let client = reqwest::Client::new();
    let mut response = client
        .get("https://api.exchangeratesapi.io/latest")
        .query(&[("base", arguments.base), ("symbols", arguments.symbol)])
        .send()
        .expect("Error");

    let rates: CurrencyRatesResponse = response.json().unwrap();

    CurrencyRate {
        base: arguments.base,
        symbol: arguments.symbol,
        rate: rates.rates[arguments.symbol],
    }
}

fn calculate_currency_value(value: f64, rate: f64) -> f64 {
    value * rate
}

fn show_report(arguments: &Arguments, currency_rate: &CurrencyRate, total: f64) {
    println!(
        "{:.2} {} worths {:.2} {}",
        arguments.value, currency_rate.base, total, currency_rate.symbol
    )
}
