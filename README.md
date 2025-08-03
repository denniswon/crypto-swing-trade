# Crypto Trade Alert 🚀

A Rust CLI tool that analyzes cryptocurrency markets to detect potential swing trading opportunities using volatility-based signals.

## Features

- **Real-time Market Analysis**: Fetches live data from CoinMarketCap API
- **Volatility-based Filtering**: Identifies coins within specified volatility ranges
- **Trade Signal Generation**: Calculates entry, take-profit, and stop-loss levels
- **Flexible Symbol Filtering**: Filter by specific tokens or analyze all markets
- **JSON Output**: Structured output for easy integration with other tools

## Installation

### Prerequisites

- Rust 1.70+ installed
- CoinMarketCap API key (get one at [coinmarketcap.com/api](https://coinmarketcap.com/api/))

### Setup

1. Clone the repository:

   ```bash
   git clone https://github.com/denniswon/crypto-swing-trade.git
   cd crypto-swing-trade
   ```

2. Install dependencies:

   ```bash
   cargo build
   ```

3. Set up your environment:

   ```bash
   cp .env.example .env
   # Edit .env and add your CoinMarketCap API key
   ```

## Usage

### Basic Commands

**Analyze all tokens with default volatility range (10-20%)**:

```bash
cargo run
```

**Filter by specific symbols**:

```bash
cargo run -- --symbols BONK DOGE PEPE
```

**Set custom volatility range**:

```bash
cargo run -- --min-atr 5.0 --max-atr 15.0
```

**Exact symbol matching**:

```bash
cargo run -- --symbols BTC ETH --exact-match true
```

### Command Line Options

| Option | Description | Default |
|--------|-------------|---------|
| `-s, --symbols` | Token symbols to filter (space-separated) | All tokens |
| `--exact-match` | Use exact symbol matching | `false` |
| `--min-atr` | Minimum volatility percentage | `10.0` |
| `--max-atr` | Maximum volatility percentage | `20.0` |

### Example Output

```json
[
  {
    "symbol": "BONK",
    "price": 0.00001234,
    "entry": 0.00001234,
    "take_profit": 0.00001419,
    "stop_loss": 0.00001172,
    "volatility_pct": 12.5,
    "expected_move": "Upside Breakout"
  }
]
```

## Configuration

### Environment Variables

Create a `.env` file in the project root:

```env
CMC_API_KEY=your_coinmarketcap_api_key_here
```

### API Requirements

- **CoinMarketCap Pro API**: Required for real-time market data
- **Rate Limits**: Respects API rate limits (varies by plan)
- **Data Coverage**: Analyzes top 100 cryptocurrencies by 24h volume

## Trading Strategy

The tool implements a simple swing trading strategy:

1. **Volatility Filter**: Identifies coins with 24h price changes within specified range
2. **Entry Signal**: Current market price as entry point
3. **Take Profit**: 15% above entry price
4. **Stop Loss**: 5% below entry price
5. **Trend Analysis**: Uses 7-day performance to suggest trade direction

### Risk Disclaimer

⚠️ **This tool is for educational purposes only. Cryptocurrency trading involves substantial risk of loss. Always do your own research and never invest more than you can afford to lose.**

## Development

### Dependencies

- `reqwest` - HTTP client for API requests
- `serde` - JSON serialization/deserialization
- `tokio` - Async runtime
- `chrono` - Date/time handling
- `clap` - Command-line argument parsing
- `dotenv` - Environment variable loading

### Building from Source

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Check code formatting
cargo fmt --check

# Run clippy lints
cargo clippy
```

### Project Structure

```
src/
├── main.rs          # Main application logic
Cargo.toml           # Dependencies and metadata
.env.example         # Environment template
README.md           # This file
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [ ] Add support for multiple exchanges
- [ ] Implement technical indicators (RSI, MACD, etc.)
- [ ] Add backtesting functionality
- [ ] Create web dashboard
- [ ] Add email/Discord notifications
- [ ] Support for custom trading strategies

## Support

If you find this tool useful, consider:

- ⭐ Starring the repository
- 🐛 Reporting bugs via issues
- 💡 Suggesting new features
- 🤝 Contributing code improvements
