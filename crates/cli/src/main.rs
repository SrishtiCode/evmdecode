use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(name = "evm-xray", about = "EVM transaction decoder & simulator")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Decode {
        #[arg(long)] tx: String,
        #[arg(long)] rpc: String,
        #[arg(long)] abi: Option<String>,
        #[arg(long, default_value = "pretty")]
        output: String,
    },
    Simulate {
        #[arg(long)] tx: String,
        #[arg(long)] rpc: String,
    },
    Mev {
        #[arg(long)] tx: String,
        #[arg(long)] rpc: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Decode { tx, rpc, abi, output } =>
            commands::decode::run(&tx, &rpc, abi.as_deref(), &output).await?,
        Commands::Simulate { tx, rpc } =>
            commands::simulate::run(&tx, &rpc).await?,
        Commands::Mev { tx, rpc } =>
            commands::mev::run(&tx, &rpc).await?,
    }
    Ok(())
}