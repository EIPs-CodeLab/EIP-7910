use clap::Parser;
use eth_config_api::EthConfigResponse;

/// Simple validator CLI for comparing eth_config responses across nodes.
#[derive(Parser, Debug)]
#[command(name = "eth-config-cli")]
struct Cli {
    /// Endpoint URLs to query
    #[arg(required = true)]
    endpoints: Vec<String>,
}

fn main() {
    let _cli = Cli::parse();
    // TODO: implement node queries and diffing.
    println!("eth-config-cli placeholder");
}
