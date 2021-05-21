use dotenv::dotenv;
use rust_ethereum_study::generate_web3_transport;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let web3 = generate_web3_transport();

    Ok(())
}
