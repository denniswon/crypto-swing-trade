use clap::Parser;
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

/// Command-line options for token filter and volatility
#[derive(Parser, Debug)]
#[command(
    name = "SwingTradeAlert",
    version = "1.0",
    about = "Detects crypto swing trade setups"
)]
pub struct SimpleArgs {
    /// Token symbols to filter against (e.g., BONK DOGE PEPE)
    #[arg(short, long)]
    symbols: Option<Vec<String>>,

    #[arg(long, default_value = "false")]
    exact_match: Option<bool>,

    /// Minimum volatility % (e.g., 10.0)
    #[arg(long, default_value = "10.0")]
    min_atr: Option<f64>,

    /// Maximum volatility % (e.g., 20.0)
    #[arg(long, default_value = "20.0")]
    max_atr: Option<f64>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Platform {
    id: u64,
    name: String,
    symbol: String,
    slug: String,
    token_address: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Coin {
    id: u64,
    name: String,
    symbol: String,
    slug: String,
    cmc_rank: Option<u64>,
    num_market_pairs: Option<u64>,
    circulating_supply: Option<f64>,
    total_supply: Option<f64>,
    max_supply: Option<f64>,
    infinite_supply: Option<bool>,
    last_updated: String,
    date_added: String,
    tags: Vec<String>,
    platform: Option<Platform>,
    self_reported_circulating_supply: Option<f64>,
    self_reported_market_cap: Option<f64>,
    quote: HashMap<String, Quote>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Quote {
    price: f64,
    volume_24h: f64,
    volume_change_24h: f64,
    percent_change_1h: f64,
    percent_change_24h: f64,
    percent_change_7d: f64,
    market_cap: f64,
    market_cap_dominance: f64,
    fully_diluted_market_cap: f64,
    last_updated: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TradeSignal {
    symbol: String,
    price: f64,
    entry: f64,
    take_profit: f64,
    stop_loss: f64,
    volatility_pct: f64,
    expected_move: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CmcResponseMetadata {
    timestamp: String,
    error_code: u64,
    error_message: Option<String>,
    elapsed: u64,
    credit_count: u64,
    notice: Option<String>,
    total_count: u64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CmcResponse {
    status: CmcResponseMetadata,
    data: Vec<Coin>,
}

pub async fn run(args: &SimpleArgs, page: u64) -> Result<usize, Box<dyn std::error::Error>> {
    let symbols = args.symbols.clone().unwrap_or_default();
    let exact_match = args.exact_match.unwrap_or_default();
    let min_atr = args.min_atr.unwrap_or_default();
    let max_atr = args.max_atr.unwrap_or_default();

    let exact_match_str = match exact_match {
        true => "true",
        false => "false",
    };
    let token_filter_str = match symbols.len() {
        0 => "for all tokens".to_string(),
        _ if symbols.len() < 5 => format!(
            "for {} (exact match: {})",
            symbols.join(", "),
            exact_match_str
        ),
        _ => format!(
            "for {} tokens (exact match: {})",
            symbols.len(),
            exact_match_str,
        ),
    };

    println!(
        "Analyzing daily swing trades {token_filter_str} with min_atr: {min_atr} and max_atr: {max_atr}"
    );

    let token_filters = symbols
        .iter()
        .map(|s| s.to_lowercase())
        .collect::<Vec<String>>();

    dotenv().ok();
    let api_key = env::var("CMC_API_KEY")?;

    let mut headers = HeaderMap::new();
    headers.insert("X-CMC_PRO_API_KEY", HeaderValue::from_str(&api_key)?);

    let client = reqwest::Client::new();
    let url = format!(
        "https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest?limit=100&sort=volume_24h&start={page}"
    );

    let data = match client.get(url).headers(headers).send().await {
        Ok(res) => res.json::<CmcResponse>().await?.data,
        Err(e) => panic!("API request failed: {e}"),
    };

    let mut signals: Vec<TradeSignal> = Vec::new();

    for coin in data.iter().filter(|c| {
        if symbols.is_empty() {
            return true;
        }
        if exact_match {
            let symbol = c.symbol.to_lowercase();
            token_filters.contains(&symbol)
        } else {
            let symbol = c.symbol.to_lowercase();
            token_filters.iter().any(|f| symbol.contains(f))
        }
    }) {
        if let Some(usd) = coin.quote.get("USD") {
            let volatility = usd.percent_change_24h.abs();

            if volatility > min_atr && volatility < max_atr {
                let entry = usd.price;
                let take_profit = entry * 1.15;
                let stop_loss = entry * 0.95;
                let expected_move = if usd.percent_change_7d > 0.0 {
                    "Upside Breakout"
                } else {
                    "Cautious Reversal"
                };

                signals.push(TradeSignal {
                    symbol: coin.symbol.clone(),
                    price: usd.price,
                    entry,
                    take_profit,
                    stop_loss,
                    volatility_pct: volatility,
                    expected_move: expected_move.to_string(),
                });
            }
        }
    }

    println!("{}", serde_json::to_string_pretty(&signals)?);
    Ok(signals.len())
}
