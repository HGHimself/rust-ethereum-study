#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;
pub mod utils;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::error::Error;
use web3;

pub fn parse_address(address: String) -> web3::types::Address {
    address.parse().unwrap()
}

pub fn contract_name_to_path(name: String) -> String {
    format!("src/res/{}", name)
}

pub fn get_account() -> String {
    let eth_node_url = env::var("ETH_NODE_ACCOUNT").expect("ETH_NODE_ACCOUNT must be set");
    eth_node_url
}

pub fn generate_web3_transport() -> Result<web3::Web3<web3::transports::Http>, Box<dyn Error>> {
    let eth_node_url = env::var("ETH_NODE_URL").expect("ETH_NODE_URL must be set");
    let transport = web3::transports::Http::new(&eth_node_url)?;
    Ok(web3::Web3::new(transport))
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn establish_connection_test() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL_TEST").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
