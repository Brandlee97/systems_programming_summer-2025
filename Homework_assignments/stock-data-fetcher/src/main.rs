use serde::Deserialize;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// Trait that defines how to fetch and save asset prices
trait Pricing {
    fn fetch_price(&mut self) -> Result<f64, Box<dyn Error>>; // fetch latest price
    fn current_price(&self) -> f64; // return last stored price
    fn symbol(&self) -> &'static str; // short asset symbol

    // Save timestamp and price to a file
    fn save_to_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let ts = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let mut f = OpenOptions::new().create(true).append(true).open(path)?;
        writeln!(f, "{},{}", ts, self.current_price())?;
        Ok(())
    }
}

// JSON structs for parsing API responses
#[derive(Debug, Deserialize)]
struct CoinPrice {
    usd: f64,
}

#[derive(Debug, Deserialize)]
struct BitcoinResp {
    bitcoin: CoinPrice,
}

#[derive(Debug, Deserialize)]
struct EthereumResp {
    ethereum: CoinPrice,
}

// Asset structs to hold last fetched price
#[derive(Debug)]
struct Bitcoin {
    price: f64,
}
#[derive(Debug)]
struct Ethereum {
    price: f64,
}
#[derive(Debug)]
struct SPy500 {
    price: f64,
}

// Constructors for each asset
impl Bitcoin {
    fn new() -> Self {
        Self { price: 0.0 }
    }
}
impl Ethereum {
    fn new() -> Self {
        Self { price: 0.0 }
    }
}
impl SPy500 {
    fn new() -> Self {
        Self { price: 0.0 }
    }
}

// Bitcoin API implementation
impl Pricing for Bitcoin {
    fn fetch_price(&mut self) -> Result<f64, Box<dyn Error>> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
        let resp: BitcoinResp = ureq::get(url).call()?.into_json()?;
        self.price = resp.bitcoin.usd;
        Ok(self.price)
    }

    fn current_price(&self) -> f64 {
        self.price
    }

    fn symbol(&self) -> &'static str {
        "BTC"
    }
}

// Ethereum API implementation
impl Pricing for Ethereum {
    fn fetch_price(&mut self) -> Result<f64, Box<dyn Error>> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
        let resp: EthereumResp = ureq::get(url).call()?.into_json()?;
        self.price = resp.ethereum.usd;
        Ok(self.price)
    }

    fn current_price(&self) -> f64 {
        self.price
    }

    fn symbol(&self) -> &'static str {
        "ETH"
    }
}

// S&P 500 API implementation using SPY CSV from Stooq
impl Pricing for SPy500 {
    fn fetch_price(&mut self) -> Result<f64, Box<dyn Error>> {
        let url = "https://stooq.com/q/l/?s=spy&i=d";
        let body = ureq::get(url).call()?.into_string()?;

        // Skip header line and find first data line
        let line = body
            .lines()
            .skip(1)
            .find(|l| !l.trim().is_empty())
            .ok_or("SPY CSV: no data line")?;

        // CSV format: symbol,date,time,open,high,low,close,volume
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() < 7 {
            return Err("SPY CSV: unexpected format".into());
        }

        let close: f64 = cols[6].parse()?;
        self.price = close;
        Ok(self.price)
    }

    fn current_price(&self) -> f64 {
        self.price
    }

    fn symbol(&self) -> &'static str {
        "SP500"
    }
}

// Main loop
fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting Financial Data Fetcher. Press Ctrl+C to stop.");

    // Vector of different assets implementing Pricing trait
    let mut assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin::new()),
        Box::new(Ethereum::new()),
        Box::new(SPy500::new()),
    ];

    loop {
        // Fetch and save prices for each asset
        for asset in assets.iter_mut() {
            match asset.fetch_price() {
                Ok(p) => {
                    let file = match asset.symbol() {
                        "BTC" => "btc.csv",
                        "ETH" => "eth.csv",
                        _ => "sp500.csv",
                    };
                    if let Err(e) = asset.save_to_file(file) {
                        eprintln!("Failed to save {} to {}: {}", asset.symbol(), file, e);
                    } else {
                        println!("{}: ${:.2}  -> appended to {}", asset.symbol(), p, file);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to fetch {}: {}", asset.symbol(), e);
                }
            }
        }

        // Wait 10 seconds before fetching again
        thread::sleep(Duration::from_secs(10));
    }
}