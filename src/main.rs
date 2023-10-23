use bitcoin_node_handshake::{run, Config};

#[tokio::main]
async fn main() {
    let cli_arguments = std::env::args();

    let config = match Config::from(cli_arguments) {
        Ok(config) => config,
        Err(e) => {
            println!("Problem parsing arguments: {:?}", e.to_string());
            return;
        }
    };

    if let Err(e) = run(&config).await {
        println!(
            "Error while running the {:?}: {:?}",
            config.command,
            e.to_string()
        )
    }
}
