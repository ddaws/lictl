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
    Login,
    Logout,
    Whoami,
    #[command(subcommand)]
    Broadcasts(cmd::broadcasts::Commands),
    #[command(subcommand)]
    Games(cmd::games::Commands),
    #[command(subcommand)]
    Studies(cmd::studies::Commands),
    #[command(subcommand)]
    BroadcastRounds(cmd::broadcast_rounds::Commands),
    #[command(subcommand)]
    Req(cmd::req::Commands),
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let context = Context::new()?;

    let output: Value = match cli.command {
        Commands::Login => cmd::login::run().await?,
        Commands::Logout => cmd::logout::run().await?,
        Commands::Whoami => cmd::whoami::run(&context).await?,
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
