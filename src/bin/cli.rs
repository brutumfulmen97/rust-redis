use clap::{Parser, Subcommand};
use rust_redis::client::Client;

#[derive(Parser, Debug)]
#[command(name = "rust-redis-cli", version, about = "Issue redis commands")]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    #[arg(long, default_value_t = 6379)]
    port: u16,
}

#[derive(Subcommand, Debug)]
enum Command {
    Get { key: String },
    Set { key: String, value: String },
    Del { keys: Vec<String> },
    Ping { msg: Option<String> },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> rust_redis::Result<()> {
    let cli = Cli::parse();
    let addr = format!("{}:{}", cli.host, cli.port);
    let mut client = Client::connect(&addr).await?;

    match cli.command {
        Command::Get { key } => {
            if let Some(value) = client.get(&key).await? {
                if let Ok(s) = std::str::from_utf8(&value) {
                    println!("\"{s}\"");
                } else {
                    println!("{value:?}");
                }
            } else {
                println!("(nil)");
            }
        }
        Command::Set { key, value } => {
            client.set(&key, value.into()).await?;
            println!("OK");
        }
        Command::Del { keys } => {
            let res = client.del(keys).await?;
            println!("{res}");
        }
        Command::Ping { msg } => {
            let res = client.ping(msg.map(|m| m.into())).await?;
            if let Ok(s) = std::str::from_utf8(&res) {
                println!("\"{s}\"");
            } else {
                println!("{res:?}");
            }
        }
    }

    Ok(())
}
