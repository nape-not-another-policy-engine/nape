mod composition;
mod gateway_driver;
mod io_adapter;

use crate::io_adapter::clap::cli;

#[tokio::main]
async fn main() {
    let command_results = cli::run();
    std::process::exit(io_adapter::clap::dispatch::dispatch(&command_results).await);
}
