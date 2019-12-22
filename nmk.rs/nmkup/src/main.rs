#[macro_use]
extern crate log;

mod archive;
mod arg;
mod build;
mod client;
mod entrypoint;
mod logging;
mod gcloud;
mod nmkup;

type BoxError = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg = arg::parse();
    logging::setup(arg.debug);

    let nmk_dir = nmkup::find_nmkdir();
    archive::install_or_update(&arg, &nmk_dir).await?;
    entrypoint::install(&nmk_dir).await?;
    nmkup::self_setup(&nmk_dir);
    Ok(())
}