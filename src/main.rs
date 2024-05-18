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
    Util {
        #[command(subcommand)]
        command: UtilCommand,
    },
}

#[derive(Subcommand)]
enum UtilCommand {
    So {
        #[command(subcommand)]
        command: SoUtilCommand,
    },
}

#[derive(Subcommand)]
enum SoUtilCommand {
    Filter,
    Auth,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Command::Scrape { pages } => {
            cmd::scrape::scrape(pages).await?;
        }
        Command::Util {
            command: UtilCommand::So {
                command: SoUtilCommand::Filter,
            },
        } => {
            cmd::utils::so::filter::create().await?;
        }
        Command::Util {
            command: UtilCommand::So {
                command: SoUtilCommand::Auth,
            },
        } => {
            cmd::utils::so::auth::auth().await?;
        }
    }

    Ok(())
}