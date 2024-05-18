use clap::{builder::ValueParser, Parser, Subcommand};

use std::str::FromStr;

mod cmd;
mod question;

use cmd::scrape::Page;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Scrape {
        #[arg(value_parser = ValueParser::new(Page::from_str))]
        pages: Vec<Page>,
    },
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Command::Scrape { pages } => {
            cmd::scrape::scrape(pages).await?;
        }
    }

    Ok(())
}