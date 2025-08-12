use serde::Deserialize;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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

#[derive(Debug, Deserialize)]
struct YahooChart {
    chart: YahooChartInner,
}
#[derive(Debug, Deserialize)]
struct YahooChartInner {
    result: Vec<YahooResult>,
}
#[derive(Debug, Deserialize)]
struct YahooResult {
    meta: YahooMeta,
    #[serde(default)]
    indicators: Option<YahooIndicators>,
}
#[derive(Debug, Deserialize)]
struct YahooMeta {
    #[serde(rename = "regularMarketPrice")]
    regular_market_price: Option<f64>,
}
#[derive(Debug, Deserialize)]
struct YahooIndicators {
    quote: Vec<YahooQuote>,
}
#[derive(Debug, Deserialize)]
struct YahooQuote {
    close: Option<Vec<Option<f64>>>,
}

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

impl Pricing for SPy500 {
    fn fetch_price(&mut self) -> Result<f64, Box<dyn Error>> {
        let url = "https://query2.finance.yahoo.com/v8/finance/chart/%5EGSPC?range=1d&interval=1m";
        let resp: YahooChart = ureq::get(url)
            .set("User-Agent", "financial-data-fetcher/1.0")
            .call()?
            .into_json()?;

        let result = resp.chart.result.get(0).ok_or("No result")?;

        if let Some(p) = result.meta.regular_market_price {
            self.price = p;
            return Ok(self.price);
        }

        let last_close = result
            .indicators
            .as_ref()
            .and_then(|inds| inds.quote.get(0))
            .and_then(|q| q.close.as_ref())
            .and_then(|closes| closes.iter().rev().flatten().copied().next());

        if let Some(p) = last_close {
            self.price = p;
            Ok(self.price)
        } else {
            Err("No price found".into())
        }
    }
    fn current_price(&self) -> f64 {
        self.price
    }
    fn symbol(&self) -> &'static str {
        "SP500"
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting Financial Data Fetcher. Press Ctrl+C to stop.");

    let mut assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin::new()),
        Box::new(Ethereum::new()),
        Box::new(SPy500::new()),
    ];

    loop {
        for asset in assets.iter_mut() {
            match asset.fetch_price() {
                Ok(p) => {
                    let file = match asset.symbol() {
                        "BTC" => "btc.csv",
                        "ETH" => "eth.csv",
                        _ => "sp500.csv",
                    };
                    if let Err(e) = asset.save_to_file(file) {
                        eprintln!("Save error {}: {}", asset.symbol(), e);
                    } else {
                        println!("{}: ${:.2} -> {}", asset.symbol(), p, file);
                    }
                }
                Err(e) => eprintln!("Fetch error {}: {}", asset.symbol(), e),
            }
        }
        thread::sleep(Duration::from_secs(10));
    }
}
