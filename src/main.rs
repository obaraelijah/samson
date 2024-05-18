mod cmd;
mod question;

fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();
    Ok(())
}
