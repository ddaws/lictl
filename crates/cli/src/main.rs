mod cmd;
mod context;
mod constants;

use clap::{Parser, Subcommand};
use anyhow::Result;
use context::Context;

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
    Whoami,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let context = Context::new()?;

    match cli.command {
        Commands::Login => {
            cmd::login::run().await
        }
        Commands::Logout => {
            cmd::logout::run().await
        }
        Commands::Whoami => {
            cmd::whoami::run(&context).await
        }
    }
}