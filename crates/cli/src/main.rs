mod cmd;

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "lictl")]
#[command(about = "A CLI tool for scripting Lichess", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Login,
    Logout,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Login => {
            cmd::login::run().await
        }
        Commands::Logout => {
            cmd::logout::run().await
        }
    }
}