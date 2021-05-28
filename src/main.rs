use dotenv::dotenv;
use rust_ethereum_study::models::contract;
use rust_ethereum_study::utils::{read_file_to_bytes, read_file_to_string};
use rust_ethereum_study::{
    contract_name_to_path, establish_connection, generate_web3_transport, get_account,
    parse_address,
};
use std::error::Error;
use std::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let conn = establish_connection();
    let web3 = generate_web3_transport()?;

    let my_account = get_account();
    let name = "SimpleStorage";

    let contract = &contract::read_by_name(&conn, name.to_string())[0];
    let web3_contract = contract::fetch(&web3, contract.address.clone(), name.to_string()).await?;

    println!("{:?}", web3_contract.abi());

    Ok(())
}
