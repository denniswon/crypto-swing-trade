use clap::Parser;
use clap::Subcommand;
use swing_trade_alert::advanced;
use swing_trade_alert::advanced::AdvancedArgs;
use swing_trade_alert::simple;
use swing_trade_alert::simple::SimpleArgs;

/// Command-line options for token filter and volatility
#[derive(Parser, Debug)]
#[command(
    name = "SwingTradeAlert",
    version = "1.0",
    about = "Detects crypto swing trade setups"
)]
pub struct Cli {
    /// Commands to be executed
    #[command(subcommand)]
    pub command: Commands,
}

/// Commands to be executed
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Simple command
    #[command(name = "simple")]
    Simple(SimpleArgs),
    /// Advanced command
    #[command(name = "advanced")]
    Advanced(AdvancedArgs),
}

const MAX_RETRIES: u64 = 5;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let command = cli.command;

    let mut retries = MAX_RETRIES;
    loop {
        let count = match command {
            Commands::Simple(ref args) => simple::run(args, MAX_RETRIES - retries + 1).await,
            Commands::Advanced(ref args) => advanced::run(args, MAX_RETRIES - retries + 1).await,
        };
        if let Err(e) = count {
            eprintln!("❌ Error: {e}");
            break;
        }

        let count = count.unwrap();
        if count > 0 {
            break;
        }

        retries -= 1;
        if retries == 0 {
            break;
        }

        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    Ok(())
}
