mod cmd;
mod constants;
mod context;

use anyhow::Result;
use clap::{Parser, Subcommand};
use context::Context;
use serde_json::Value;

#[derive(Parser)]
#[command(name = "lictl")]
#[command(about = "A CLI tool for scripting Lichess", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Login to Lichess (alias for 'auth oauth')
    Login,
    /// Logout from Lichess
    Logout,
    /// Show current user information
    Whoami,
    /// Authentication commands
    #[command(subcommand)]
    Auth(cmd::auth::Commands),
    /// Broadcast commands
    #[command(subcommand)]
    Broadcasts(cmd::broadcasts::Commands),
    /// Game commands
    #[command(subcommand)]
    Games(cmd::games::Commands),
    /// Study commands
    #[command(subcommand)]
    Studies(cmd::studies::Commands),
    /// Broadcast round commands
    #[command(subcommand)]
    BroadcastRounds(cmd::broadcast_rounds::Commands),
    /// Raw API request commands
    #[command(subcommand)]
    Req(cmd::req::Commands),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let context = Context::new()?;

    let output: Value = match cli.command {
        Commands::Login => cmd::login::run().await?,
        Commands::Logout => cmd::logout::run().await?,
        Commands::Whoami => cmd::whoami::run(&context).await?,
        Commands::Auth(cmd) => cmd::auth::run(cmd).await?,
        Commands::Broadcasts(cmd) => cmd::broadcasts::run(&context, cmd).await?,
        Commands::Games(cmd) => cmd::games::run(&context, cmd).await?,
        Commands::Studies(cmd) => cmd::studies::run(&context, cmd).await?,
        Commands::BroadcastRounds(cmd) => cmd::broadcast_rounds::run(&context, cmd).await?,
        Commands::Req(cmd) => cmd::req::run(&context, cmd).await?,
    };

    // Print the output depending on the type of the output
    match output {
        Value::Null => {}
        Value::String(s) => println!("{}", s),
        _ => println!("{}", serde_json::to_string_pretty(&output)?),
    }

    Ok(())
}
