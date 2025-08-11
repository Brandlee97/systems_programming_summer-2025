use serde::Deserialize;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Default)]
struct Bitcoin { price: f64 }
#[derive(Debug, Default)]
struct Ethereum { price: f64 }
#[derive(Debug, Default)]
struct SP500 { price: f64 }

trait Pricing {
    fn fetch_price(&mut self) -> Result<f64, Box<dyn Error>>;
    fn current_price(&self) -> f64;
    fn symbol(&self) -> &'static str;

    fn save_to_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let ts = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let mut f = OpenOptions::new().create(true).append(true).open(path)?;
        writeln!(f, "{},{}", ts, self.current_price())?;
        Ok(())
    }
}

// ---- You implement these three ----
impl Pricing for Bitcoin {
    fn fetch_price(&mut self) -> Result<f64, Box<dyn Error>> {
        // TODO: call CoinGecko for "bitcoin" using ureq,
        // parse JSON with serde, set self.price, return it.
        unimplemented!()
    }
    fn current_price(&self) -> f64 { self.price }
    fn symbol(&self) -> &'static str { "BTC" }
}

impl Pricing for Ethereum {
    fn fetch_price(&mut self) -> Result<f64, Box<dyn Error>> {
        // TODO: same idea for "ethereum".
        unimplemented!()
    }
    fn current_price(&self) -> f64 { self.price }
    fn symbol(&self) -> &'static str { "ETH" }
}

impl Pricing for SP500 {
    fn fetch_price(&mut self) -> Result<f64, Box<dyn Error>> {
        // TODO: fetch SPY from Stooq CSV, parse the close price,
        // set self.price, return it. (No extra crates.)
        unimplemented!()
    }
    fn current_price(&self) -> f64 { self.price }
    fn symbol(&self) -> &'static str { "SP500" }
}

// Optional helper type for BTC/ETH JSON
#[derive(Deserialize)]
struct UsdPrice { usd: f64 }

// Optional helper function for CoinGecko
fn fetch_coingecko_usd(_id: &str) -> Result<f64, Box<dyn Error>> {
    // TODO: build URL, ureq::get(...).call()?.into_json::<HashMap<_, UsdPrice>>()?,
    // extract .usd
    unimplemented!()
}

fn main() -> Result<(), Box<dyn Error>> {
    // TODO: create a Vec<Box<dyn Pricing>>, loop:
    // 1) fetch_price, 2) save_to_file("<symbol>.csv"), 3) sleep 10s
    unimplemented!()
}