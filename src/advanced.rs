use clap::Parser;
use reqwest::Client;
use serde::Deserialize;
use std::{collections::HashMap, env};

use crate::category::Category;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Roi {
    times: f64,
    currency: String,
    percentage: f64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CoinGeckoResponse {
    id: String,
    symbol: String,
    name: String,
    image: String,
    current_price: Option<f64>,
    market_cap: Option<f64>,
    market_cap_rank: Option<u64>,
    fully_diluted_valuation: Option<f64>,
    total_volume: Option<f64>,
    high_24h: Option<f64>,
    low_24h: Option<f64>,
    price_change_24h: Option<f64>,
    price_change_percentage_24h: Option<f64>,
    market_cap_change_24h: Option<f64>,
    market_cap_change_percentage_24h: Option<f64>,
    circulating_supply: Option<f64>,
    total_supply: Option<f64>,
    max_supply: Option<f64>,
    ath: Option<f64>,
    ath_change_percentage: Option<f64>,
    ath_date: Option<String>,
    atl: Option<f64>,
    atl_change_percentage: Option<f64>,
    atl_date: Option<String>,
    roi: Option<Roi>,
    last_updated: String,
    price_change_percentage_7d_in_currency: Option<f64>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CoinMarketCapData {
    id: u32,
    symbol: String,
    name: String,
    quote: HashMap<String, Quote>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Quote {
    price: f64,
    percent_change_7d: Option<f64>,
    market_cap: f64,
    volume_24h: f64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CoinMarketCapResponse {
    data: Vec<CoinMarketCapData>,
}

#[derive(Debug, Clone)]
pub struct TradingSignal {
    symbol: String,
    name: String,
    current_price: f64,
    price_change_7d: f64,
    entry_price: f64,
    profit_target: f64,
    stop_loss: f64,
    position_size_usd: f64,
    position_size_coins: f64,
    risk_reward_ratio: f64,
    moving_average_7d: f64,
    ma_deviation_pct: f64,
    atr_7d: f64,
    atr_percentage: f64,
    signal_strength: f64,
    expected_duration_days: u32,
    market_cap: f64,
    volume: f64,
}

impl TradingSignal {
    fn calculate_signal_strength(&self) -> f64 {
        // Weighted scoring system
        let momentum_score = if self.price_change_7d > 0.0 {
            (self.price_change_7d / 100.0).min(1.0)
        } else {
            0.0
        };

        let volume_score = (self.volume / self.market_cap).min(1.0) * 0.5;
        let rr_score = (self.risk_reward_ratio / 5.0).min(1.0) * 0.3;
        let atr_score = (self.atr_percentage / 15.0).min(1.0) * 0.2;

        (momentum_score * 0.4 + volume_score * 0.3 + rr_score * 0.2 + atr_score * 0.1) * 100.0
    }
}

#[allow(dead_code)]
pub struct TradingAnalyzer {
    client: Client,
    coingecko_api: String,
    coinmarketcap_api: String,
    coinmarketcap_key: Option<String>,
    coingecko_key: Option<String>,
}

#[allow(dead_code)]
impl TradingAnalyzer {
    fn new(coinmarketcap_key: Option<String>, coingecko_key: Option<String>) -> Self {
        Self {
            client: Client::new(),
            coingecko_api: "https://pro-api.coingecko.com/api/v3".to_string(),
            coinmarketcap_api: "https://pro-api.coinmarketcap.com/v1".to_string(),
            coinmarketcap_key,
            coingecko_key,
        }
    }

    async fn fetch_coingecko_data(
        &self,
        limit: usize,
        page: u64,
        category: Option<Category>,
    ) -> Result<Vec<CoinGeckoResponse>, Box<dyn std::error::Error>> {
        let url = match category {
            Some(category) => format!(
                "{}/coins/markets?vs_currency=usd&order=market_cap_desc&per_page={}&page={}&category={}&sparkline=false&price_change_percentage=7d",
                self.coingecko_api, limit, page, category
            ),
            None => format!(
                "{}/coins/markets?vs_currency=usd&order=market_cap_desc&per_page={}&page={}&sparkline=false&price_change_percentage=7d",
                self.coingecko_api, limit, page
            ),
        };

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static("crypto-trading-signals/1.0"),
        );

        // Add API key if provided (for CoinGecko Pro)
        if let Some(key) = &self.coingecko_key {
            headers.insert(
                "x-cg-pro-api-key",
                reqwest::header::HeaderValue::from_str(key)?,
            );
        }

        let response = self.client.get(&url).headers(headers).send().await?;

        // Debug the response
        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            eprintln!("❌ CoinGecko API Error ({status}): {error_text}");
            return Err(format!("CoinGecko API returned status {status}: {error_text}").into());
        }

        let json_response = response.json::<Vec<CoinGeckoResponse>>().await;
        Ok(json_response?)
    }

    async fn fetch_coinmarketcap_data(
        &self,
        limit: usize,
        page: u64,
    ) -> Result<Vec<CoinMarketCapData>, Box<dyn std::error::Error>> {
        if let Some(api_key) = &self.coinmarketcap_key {
            let url = format!(
                "{}/cryptocurrency/listings/latest?start={}&limit={}&convert=USD",
                self.coinmarketcap_api, page, limit
            );

            let response = self
                .client
                .get(&url)
                .header("X-CMC_PRO_API_KEY", api_key)
                .send()
                .await?
                .json::<CoinMarketCapResponse>()
                .await?;

            Ok(response.data)
        } else {
            Ok(vec![])
        }
    }

    fn calculate_moving_average(&self, current_price: f64, price_change_7d: f64) -> f64 {
        // Approximate 7-day moving average based on current price and 7d change
        let price_7d_ago = current_price / (1.0 + price_change_7d / 100.0);
        (current_price + price_7d_ago) / 2.0
    }

    fn calculate_atr_7d(
        &self,
        current_price: f64,
        high_24h: f64,
        low_24h: f64,
        price_change_7d: f64,
        price_change_24h: f64,
    ) -> (f64, f64) {
        // Approximate ATR calculation using available data
        // ATR = Average of True Range over period

        // Calculate estimated daily volatility from 7-day change
        let daily_volatility = (price_change_7d.abs() / 7.0) / 100.0;

        // Current day's true range (24h high-low)
        let current_tr = high_24h - low_24h;

        // Estimate previous day's close
        let prev_close = current_price / (1.0 + price_change_24h / 100.0);

        // True range components
        let tr1 = (high_24h - prev_close).abs();
        let tr2 = (low_24h - prev_close).abs();
        let true_range = current_tr.max(tr1).max(tr2);

        // Approximate 7-day ATR
        let estimated_atr = (true_range + current_price * daily_volatility * 6.0) / 7.0;
        let atr_percentage = (estimated_atr / current_price) * 100.0;

        (estimated_atr, atr_percentage)
    }

    fn calculate_position_size(
        &self,
        account_balance: f64,
        risk_percentage: f64,
        entry_price: f64,
        stop_loss: f64,
    ) -> (f64, f64) {
        let risk_amount = account_balance * (risk_percentage / 100.0);
        let price_diff = (entry_price - stop_loss).abs();
        let position_size_coins = risk_amount / price_diff;
        let position_size_usd = position_size_coins * entry_price;

        (position_size_usd, position_size_coins)
    }

    #[allow(clippy::too_many_arguments)]
    fn generate_trading_signal(
        &self,
        coin: &CoinGeckoResponse,
        account_balance: f64,
        risk_percentage: f64,
        min_atr: f64,
        max_atr: f64,
        min_rr: f64,
        max_rr: f64,
        min_ma_pct: f64,
        max_ma_pct: f64,
        min_volume_pct: f64,
    ) -> Option<TradingSignal> {
        if coin.current_price.is_none()
            || coin.high_24h.is_none()
            || coin.low_24h.is_none()
            || coin.price_change_percentage_7d_in_currency.is_none()
            || coin.price_change_percentage_24h.is_none()
            || coin.market_cap.is_none()
            || coin.total_volume.is_none()
        {
            return None;
        }

        let current_price = coin.current_price.unwrap();
        let high_24h = coin.high_24h.unwrap();
        let low_24h = coin.low_24h.unwrap();
        let price_change_7d = coin.price_change_percentage_7d_in_currency.unwrap();
        let price_change_24h = coin.price_change_percentage_24h.unwrap();
        let market_cap = coin.market_cap.unwrap();
        let volume = coin.total_volume.unwrap();

        // Calculate ATR for volatility filtering
        let (atr_7d, atr_percentage) = self.calculate_atr_7d(
            current_price,
            high_24h,
            low_24h,
            price_change_7d,
            price_change_24h,
        );

        // Filter by ATR range
        if atr_percentage < min_atr || atr_percentage > max_atr {
            return None;
        }

        let moving_average_7d = self.calculate_moving_average(current_price, price_change_7d);

        // Calculate price position relative to moving average
        let ma_deviation_pct = ((current_price - moving_average_7d) / moving_average_7d) * 100.0;

        // Filter by moving average position
        if ma_deviation_pct < min_ma_pct || ma_deviation_pct > max_ma_pct {
            return None;
        }

        // Only consider coins with positive momentum and reasonable volume
        if price_change_7d <= 0.0 || volume < market_cap * min_volume_pct {
            return None;
        }

        // Entry strategy: slight pullback from current price
        let entry_price = current_price * 0.98; // 2% below current price

        // Dynamic profit target based on momentum and ATR
        let atr_multiplier = if atr_percentage > 15.0 {
            2.0
        } else if atr_percentage > 10.0 {
            1.8
        } else if atr_percentage > 5.0 {
            1.5
        } else {
            1.3
        };

        let profit_target = entry_price + (atr_7d * atr_multiplier);

        // Stop loss: 1.5x ATR below entry price
        let stop_loss = entry_price - (atr_7d * 1.5);

        // Ensure stop loss doesn't go below moving average support
        let stop_loss = stop_loss.max(moving_average_7d * 0.92);

        // Risk-to-reward calculation
        let potential_profit = profit_target - entry_price;
        let potential_loss = entry_price - stop_loss;

        if potential_loss <= 0.0 {
            return None;
        }

        let risk_reward_ratio = potential_profit / potential_loss;

        // Filter by risk-reward ratio range
        if risk_reward_ratio < min_rr || risk_reward_ratio > max_rr {
            return None;
        }

        let (position_size_usd, position_size_coins) =
            self.calculate_position_size(account_balance, risk_percentage, entry_price, stop_loss);

        // Expected duration based on ATR and momentum
        let expected_duration_days = if atr_percentage > 15.0 {
            2
        } else if atr_percentage > 10.0 {
            3
        } else if atr_percentage > 5.0 {
            5
        } else {
            7
        };

        let mut signal = TradingSignal {
            symbol: coin.symbol.clone(),
            name: coin.name.clone(),
            current_price,
            price_change_7d,
            entry_price,
            profit_target,
            stop_loss,
            position_size_usd,
            position_size_coins,
            risk_reward_ratio,
            moving_average_7d,
            ma_deviation_pct,
            atr_7d,
            atr_percentage,
            signal_strength: 0.0,
            expected_duration_days,
            market_cap,
            volume,
        };

        signal.signal_strength = signal.calculate_signal_strength();

        Some(signal)
    }

    #[allow(clippy::too_many_arguments)]
    async fn analyze_markets(
        &self,
        account_balance: f64,
        risk_percentage: f64,
        limit: usize,
        min_atr: f64,
        max_atr: f64,
        min_rr: f64,
        max_rr: f64,
        min_ma_pct: f64,
        max_ma_pct: f64,
        min_volume_pct: f64,
        page: u64,
        category: Option<Category>,
    ) -> Result<Vec<TradingSignal>, Box<dyn std::error::Error>> {
        println!("Fetching data from CoinGecko...");
        println!("🎯 ATR Filter: {min_atr:.1}% - {max_atr:.1}%");
        println!("💎 R:R Filter: {min_rr:.1}:1 - {max_rr:.1}:1");
        println!("📈 MA Filter: {min_ma_pct:.1}% - {max_ma_pct:.1}% vs 7d MA");
        println!("📊 Volume Filter: {min_volume_pct:.1}%");

        // Add small delay to avoid rate limiting
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let coingecko_data = self.fetch_coingecko_data(limit, page, category).await?;

        let mut signals = Vec::new();
        let mut filtered_by_atr = 0;
        let mut filtered_by_rr = 0;
        let mut filtered_by_ma = 0;
        let mut filtered_by_momentum = 0;

        for coin in &coingecko_data {
            if let Some(signal) = self.generate_trading_signal(
                coin,
                account_balance,
                risk_percentage,
                min_atr,
                max_atr,
                min_rr,
                max_rr,
                min_ma_pct,
                max_ma_pct,
                min_volume_pct,
            ) {
                signals.push(signal);
            } else {
                if coin.current_price.is_none()
                    || coin.high_24h.is_none()
                    || coin.low_24h.is_none()
                    || coin.price_change_percentage_7d_in_currency.is_none()
                    || coin.price_change_percentage_24h.is_none()
                    || coin.market_cap.is_none()
                    || coin.total_volume.is_none()
                {
                    continue;
                }

                let current_price = coin.current_price.unwrap();
                let high_24h = coin.high_24h.unwrap();
                let low_24h = coin.low_24h.unwrap();
                let price_change_7d = coin.price_change_percentage_7d_in_currency.unwrap();
                let price_change_24h = coin.price_change_percentage_24h.unwrap();
                let market_cap = coin.market_cap.unwrap();
                let total_volume = coin.total_volume.unwrap();

                // Count filter reasons for debugging

                let (_, atr_percentage) = self.calculate_atr_7d(
                    current_price,
                    high_24h,
                    low_24h,
                    price_change_7d,
                    price_change_24h,
                );

                if atr_percentage < min_atr || atr_percentage > max_atr {
                    filtered_by_atr += 1;
                } else if price_change_7d <= 0.0 || total_volume < market_cap * min_volume_pct {
                    filtered_by_momentum += 1;
                } else {
                    let moving_average_7d =
                        self.calculate_moving_average(current_price, price_change_7d);
                    let ma_deviation_pct =
                        ((current_price - moving_average_7d) / moving_average_7d) * 100.0;

                    if ma_deviation_pct < min_ma_pct || ma_deviation_pct > max_ma_pct {
                        filtered_by_ma += 1;
                    } else {
                        filtered_by_rr += 1;
                    }
                }
            }
        }

        println!("📊 Analysis Results for page {page}:");
        println!("   Total analyzed: {}", coingecko_data.len());
        println!("   ✅ Passed all filters: {}", signals.len());
        println!("   ❌ Filtered by ATR: {filtered_by_atr}");
        println!("   ❌ Filtered by momentum/volume: {filtered_by_momentum}");
        println!("   ❌ Filtered by MA position: {filtered_by_ma}");
        println!("   ❌ Filtered by R:R ratio: {filtered_by_rr}");

        // Sort by risk-reward ratio and signal strength
        signals.sort_by(|a, b| {
            let score_a = a.risk_reward_ratio * a.signal_strength;
            let score_b = b.risk_reward_ratio * b.signal_strength;
            score_b
                .partial_cmp(&score_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(signals)
    }

    fn print_signals(&self, signals: &[TradingSignal], top_n: usize) {
        println!("\n🚀 TOP {top_n} TRADING SIGNALS (Highest Risk:Reward Ratio)\n");
        println!(
            "{:<8} {:<15} {:<10} {:<10} {:<8} {:<8} {:<10} {:<10} {:<10} {:<8} {:<8} {:<12} {:<8}",
            "Rank",
            "Symbol",
            "Price",
            "7d %",
            "ATR %",
            "MA %",
            "Entry",
            "Target",
            "Stop",
            "R:R",
            "Duration",
            "Pos Size",
            "Strength"
        );
        println!("{}", "=".repeat(140));

        for (i, signal) in signals.iter().take(top_n).enumerate() {
            println!("{:<8} {:<15} ${:<9.4} {:<9.1}% {:<7.1}% {:<7.1}% ${:<9.4} ${:<9.4} ${:<9.4} {:<7.1}:1 {:<8}d ${:<11.0} {:<7.1}",
                     i + 1,
                     signal.symbol,
                     signal.current_price,
                     signal.price_change_7d,
                     signal.atr_percentage,
                     signal.ma_deviation_pct,
                     signal.entry_price,
                     signal.profit_target,
                     signal.stop_loss,
                     signal.risk_reward_ratio,
                     signal.expected_duration_days,
                     signal.position_size_usd,
                     signal.signal_strength
            );
        }

        if !signals.is_empty() {
            println!("\n📊 DETAILED ANALYSIS FOR TOP SIGNAL:");
            let top_signal = &signals[0];

            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("🎯 {} ({})", top_signal.name, top_signal.symbol);
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("📈 Current Price:      ${:.4}", top_signal.current_price);
            println!("📊 7-Day Change:       {:.2}%", top_signal.price_change_7d);
            println!(
                "📉 7-Day MA:           ${:.4} ({:.1}% deviation)",
                top_signal.moving_average_7d, top_signal.ma_deviation_pct
            );
            println!(
                "⚡ ATR (7d):           ${:.4} ({:.2}%)",
                top_signal.atr_7d, top_signal.atr_percentage
            );
            println!(
                "🔥 Signal Strength:    {:.1}/100",
                top_signal.signal_strength
            );
            println!();
            println!("💰 TRADING PLAN:");
            println!("   Entry Price:        ${:.4}", top_signal.entry_price);
            println!(
                "   Profit Target:      ${:.4} (+{:.1}%)",
                top_signal.profit_target,
                ((top_signal.profit_target - top_signal.entry_price) / top_signal.entry_price)
                    * 100.0
            );
            println!(
                "   Stop Loss:          ${:.4} (-{:.1}%)",
                top_signal.stop_loss,
                ((top_signal.entry_price - top_signal.stop_loss) / top_signal.entry_price) * 100.0
            );
            println!(
                "   Risk:Reward Ratio:  {:.1}:1",
                top_signal.risk_reward_ratio
            );
            println!();
            println!("💵 POSITION SIZING:");
            println!(
                "   Position Size:      ${:.0} ({:.2} coins)",
                top_signal.position_size_usd, top_signal.position_size_coins
            );
            println!(
                "   Expected Duration:  {} days",
                top_signal.expected_duration_days
            );
            println!();
            println!("📋 MARKET DATA:");
            println!("   Market Cap:         ${:.0}", top_signal.market_cap);
            println!("   24h Volume:         ${:.0}", top_signal.volume);
            println!(
                "   Volume/MCap Ratio:  {:.3}",
                top_signal.volume / top_signal.market_cap
            );
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "AdvancedTradingSignal",
    version = "1.0",
    about = "Generates high risk:reward crypto trading signals with advanced filtering"
)]
pub struct AdvancedArgs {
    /// Account balance in USD
    #[arg(short = 'b', long = "balance", default_value = "10000")]
    pub balance: Option<f64>,

    /// Risk percentage per trade (0.5-5%)
    #[arg(short = 'k', long = "risk", default_value = "2")]
    pub risk: Option<f64>,

    /// Number of coins to analyze
    #[arg(short = 'l', long = "limit", default_value = "100")]
    pub limit: Option<usize>,

    /// Number of top signals to display
    #[arg(short = 't', long = "top", default_value = "10")]
    pub top: Option<usize>,

    /// Minimum ATR percentage (volatility filter)
    #[arg(short = 'a', long = "min-atr", default_value = "1")]
    pub min_atr: Option<f64>,

    /// Maximum ATR percentage (volatility filter)
    #[arg(short = 'A', long = "max-atr", default_value = "20")]
    pub max_atr: Option<f64>,

    /// Minimum Risk:Reward ratio
    #[arg(short = 'r', long = "min-rr", default_value = "2.0")]
    pub min_rr: Option<f64>,

    /// Maximum Risk:Reward ratio
    #[arg(short = 'R', long = "max-rr", default_value = "10.0")]
    pub max_rr: Option<f64>,

    /// Moving Average filter: min percentage above MA (-20 = 20% below MA, 5 = 5% above MA)
    #[arg(short = 'm', long = "min-ma-pct", default_value = "-10.0")]
    pub min_ma_pct: Option<f64>,

    /// Moving Average filter: max percentage above MA
    #[arg(short = 'M', long = "max-ma-pct", default_value = "15.0")]
    pub max_ma_pct: Option<f64>,

    /// Minimum volume (in USD)
    #[arg(short = 'v', long = "min-volume", default_value = "1.0")]
    pub min_volume_pct: Option<f64>,

    /// Category filter
    #[arg(short = 'c', long = "category")]
    pub category: Option<Category>,
}

pub async fn run(args: &AdvancedArgs, page: u64) -> Result<usize, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let coinmarketcap_key = env::var("CMC_API_KEY").ok();
    let coingecko_key = env::var("COINGECKO_API_KEY").ok();

    let account_balance: f64 = args.balance.unwrap_or(10000.0);
    let risk_percentage: f64 = args.risk.unwrap_or(2.0);
    let limit: usize = args.limit.unwrap_or(100);
    let top_n: usize = args.top.unwrap_or(10);
    let min_atr: f64 = args.min_atr.unwrap_or(2.0);
    let max_atr: f64 = args.max_atr.unwrap_or(20.0);
    let min_rr: f64 = args.min_rr.unwrap_or(2.0);
    let max_rr: f64 = args.max_rr.unwrap_or(10.0);
    let min_ma_pct: f64 = args.min_ma_pct.unwrap_or(-10.0);
    let max_ma_pct: f64 = args.max_ma_pct.unwrap_or(15.0);
    let min_volume_pct: f64 = args.min_volume_pct.unwrap_or(1.0);
    let category: Option<Category> = args.category;

    // Validation
    if !(0.5..=5.0).contains(&risk_percentage) {
        eprintln!(
            "⚠️  Warning: Risk percentage should be between 0.5% and 5% for proper risk management"
        );
    }

    if min_atr >= max_atr {
        eprintln!("❌ Error: min-atr must be less than max-atr");
        return Err("min-atr must be less than max-atr".into());
    }

    if min_rr >= max_rr {
        eprintln!("❌ Error: min-rr must be less than max-rr");
        return Err("min-rr must be less than max-rr".into());
    }

    if min_ma_pct >= max_ma_pct {
        eprintln!("❌ Error: min-ma-pct must be less than max-ma-pct");
        return Err("min-ma-pct must be less than max-ma-pct".into());
    }

    // Display configuration
    println!("🔍 Crypto Trading Signal Analyzer");
    println!("💰 Account Balance: ${account_balance:.2}");
    println!("⚠️  Risk per Trade: {risk_percentage:.1}%");
    println!("🎯 ATR Range: {min_atr:.1}% - {max_atr:.1}%");
    println!("💎 R:R Range: {min_rr:.1}:1 - {max_rr:.1}:1");
    println!("📈 MA Range: {min_ma_pct:.1}% - {max_ma_pct:.1}%");
    println!("📊 Min Volume Pct: {min_volume_pct:.1}%");
    println!("📊 Analyzing {limit} coins...\n");

    let analyzer = TradingAnalyzer::new(coinmarketcap_key, coingecko_key);

    match analyzer
        .analyze_markets(
            account_balance,
            risk_percentage,
            limit,
            min_atr,
            max_atr,
            min_rr,
            max_rr,
            min_ma_pct,
            max_ma_pct,
            min_volume_pct,
            page,
            category,
        )
        .await
    {
        Ok(signals) => {
            if signals.is_empty() {
                println!("❌ No viable trading signals found with current criteria.");
                println!("💡 Try adjusting filter parameters:");
                println!("   • Widen ATR range (e.g., --min-atr 1 --max-atr 25)");
                println!("   • Lower R:R requirements (e.g., --min-rr 1.5)");
                println!("   • Broaden MA range (e.g., --min-ma-pct -20 --max-ma-pct 20)");
                println!("   • Decrease Min Volume Pct (e.g., --min-volume-pct 0.5)");
            } else {
                analyzer.print_signals(&signals, top_n);

                println!("\n📝 USAGE NOTES:");
                println!("• Signals filtered by ATR (volatility), R:R ratio, and MA position");
                println!("• ATR filters help control volatility exposure");
                println!("• R:R filters ensure favorable risk-reward ratios");
                println!("• MA filters show price position relative to 7-day moving average");
                println!("• Min Volume Pct filters ensure coins have sufficient liquidity");
                println!("• Always verify with additional indicators before trading");
                println!("• Consider market sentiment and news events");
                println!(
                    "• Use proper risk management and never risk more than you can afford to lose"
                );
                println!("• Entry prices assume 2% pullback from current price");
                println!("• This is not financial advice - do your own research!");

                println!("\n💡 FILTER EXAMPLES:");
                println!("• --min-atr 5 --max-atr 15    # Medium volatility coins");
                println!("• --min-rr 3 --max-rr 8       # Conservative R:R ratios");
                println!("• --min-ma-pct 0 --max-ma-pct 10  # Coins near/above MA");
                println!("• --min-ma-pct -5 --max-ma-pct 5  # Coins close to MA");
                println!("• --min-volume-pct 0.5  # Coins with sufficient liquidity");
            }
            Ok(signals.len())
        }
        Err(e) => {
            eprintln!("❌ Error analyzing markets: {e}");
            eprintln!("💡 Check your internet connection and API availability");
            Err(e)
        }
    }
}
