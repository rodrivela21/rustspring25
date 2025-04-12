use std::fs::OpenOptions;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use serde::Deserialize;

// Trait for fetching and saving price data
trait Pricing {
    fn fetch_price(&mut self);
    fn save_to_file(&self);
}

// Struct for Bitcoin
#[derive(Debug, Deserialize)]
struct Bitcoin {
    price: f64,
}

// Update error handling for Bitcoin
impl Pricing for Bitcoin {
    fn fetch_price(&mut self) {
        let response = match ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd").call() {
            Ok(resp) => match resp.into_string() {
                Ok(body) => match serde_json::from_str::<serde_json::Value>(&body) {
                    Ok(json) => json,
                    Err(_) => {
                        eprintln!("Failed to parse JSON response for Bitcoin.");
                        return;
                    }
                },
                Err(_) => {
                    eprintln!("Failed to read response body for Bitcoin.");
                    return;
                }
            },
            Err(_) => {
                eprintln!("Failed to fetch Bitcoin price data.");
                return;
            }
        };
        self.price = match response["bitcoin"]["usd"].as_f64() {
            Some(price) => price,
            None => {
                eprintln!("Price data not found in Bitcoin response.");
                return;
            }
        };
    }

    fn save_to_file(&self) {
        let mut file = match OpenOptions::new()
            .append(true)
            .create(true)
            .open("bitcoin_price.txt")
        {
            Ok(f) => f,
            Err(_) => {
                eprintln!("Failed to open or create bitcoin_price.txt.");
                return;
            }
        };
        if writeln!(file, "Bitcoin Price: ${}", self.price).is_err() {
            eprintln!("Failed to write Bitcoin price to file.");
        }
    }
}

// Struct for Ethereum
#[derive(Debug, Deserialize)]
struct Ethereum {
    price: f64,
}

// Update error handling for Ethereum
impl Pricing for Ethereum {
    fn fetch_price(&mut self) {
        let response = match ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd").call() {
            Ok(resp) => match resp.into_string() {
                Ok(body) => match serde_json::from_str::<serde_json::Value>(&body) {
                    Ok(json) => json,
                    Err(_) => {
                        eprintln!("Failed to parse JSON response for Ethereum.");
                        return;
                    }
                },
                Err(_) => {
                    eprintln!("Failed to read response body for Ethereum.");
                    return;
                }
            },
            Err(_) => {
                eprintln!("Failed to fetch Ethereum price data.");
                return;
            }
        };
        self.price = match response["ethereum"]["usd"].as_f64() {
            Some(price) => price,
            None => {
                eprintln!("Price data not found in Ethereum response.");
                return;
            }
        };
    }

    fn save_to_file(&self) {
        let mut file = match OpenOptions::new()
            .append(true)
            .create(true)
            .open("ethereum_price.txt")
        {
            Ok(f) => f,
            Err(_) => {
                eprintln!("Failed to open or create ethereum_price.txt.");
                return;
            }
        };
        if writeln!(file, "Ethereum Price: ${}", self.price).is_err() {
            eprintln!("Failed to write Ethereum price to file.");
        }
    }
}

// Struct for S&P 500
#[derive(Debug, Deserialize)]
struct SP500 {
    price: f64,
}

impl Pricing for SP500 {
    fn fetch_price(&mut self) {
        let api_key = "8DX4RBAOLD18QRMU"; // Replace with your Alpha Vantage API key
        let url = format!("https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=SPY&apikey={}", api_key);
        let response = match ureq::get(&url).call() {
            Ok(resp) => match resp.into_string() {
                Ok(body) => match serde_json::from_str::<serde_json::Value>(&body) {
                    Ok(json) => json,
                    Err(_) => {
                        eprintln!("Failed to parse JSON response for S&P 500.");
                        return;
                    }
                },
                Err(_) => {
                    eprintln!("Failed to read response body for S&P 500.");
                    return;
                }
            },
            Err(_) => {
                eprintln!("Failed to fetch S&P 500 price data.");
                return;
            }
        };
        self.price = match response["Global Quote"]["05. price"].as_str() {
            Some(price_str) => match price_str.parse() {
                Ok(price) => price,
                Err(_) => {
                    eprintln!("Failed to parse price string for S&P 500.");
                    return;
                }
            },
            None => {
                eprintln!("Price data not found in S&P 500 response.");
                return;
            }
        };
    }

    fn save_to_file(&self) {
        let mut file = match OpenOptions::new()
            .append(true)
            .create(true)
            .open("sp500_price.txt")
        {
            Ok(f) => f,
            Err(_) => {
                eprintln!("Failed to open or create sp500_price.txt.");
                return;
            }
        };
        if writeln!(file, "S&P 500 Price: ${}", self.price).is_err() {
            eprintln!("Failed to write S&P 500 price to file.");
        }
    }
}

fn main() {
    let mut bitcoin = Bitcoin { price: 0.0 };
    let mut ethereum = Ethereum { price: 0.0 };
    let mut sp500 = SP500 { price: 0.0 };

    loop {
        bitcoin.fetch_price();
        bitcoin.save_to_file();

        ethereum.fetch_price();
        ethereum.save_to_file();

        sp500.fetch_price();
        sp500.save_to_file();

        println!("Prices updated. Waiting for 10 seconds...");
        sleep(Duration::from_secs(10));
    }
}
