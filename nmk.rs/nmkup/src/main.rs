#[macro_use]
extern crate log;

mod archive;
mod arg;
mod client;
mod logging;
mod gcloud;

type BoxError = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg = arg::parse();
    logging::setup(arg.debug);
    archive::install_or_update().await?;
    Ok(())
}