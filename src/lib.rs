#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate juniper;

pub mod context;
pub mod models;
pub mod schema;
pub mod utils;

use diesel::prelude::*;
use dotenv::dotenv;
use ethcontract::prelude::*;
use juniper::FieldResult;
use std::env;
use std::error::Error;
use web3;

ethcontract::contract!(
    pub "truffle/build/Gheedorah.json",
);

pub struct QueryRoot;

#[graphql_object(context = "Context")]
impl QueryRoot {
    async fn balance_of(context: &Context, storeId: String, clientId: String) -> FieldResult<f64> {
        let balance = context
            .contract
            .balance_of(storeId, clientId)
            .call()
            .await?;

        Ok((balance.as_u64() as f64) / 100f64)
    }
}

pub fn parse_address(address: String) -> web3::types::Address {
    address.parse().unwrap()
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
