//based on examples/contract.rs

use std::error::Error;
use std::time;
use web3::{
    contract::{Contract, Options},
    types::{U256, TransactionRequest},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let transport = web3::transports::Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(transport);
    let mut accounts = web3.eth().accounts().await?;

    accounts.push("00a329c0648769a73afac7f9381e08fb43dbea76".parse().unwrap());

    println!("Accounts: {:?}", accounts);
    //
    // let balance_before = web3.eth().balance(accounts[0], None).await?;
    //
    // let tx = TransactionRequest {
    //     from: accounts[0],
    //     to: Some(accounts[1]),
    //     gas: None,
    //     gas_price: None,
    //     value: Some(U256::from(10000)),
    //     data: None,
    //     nonce: None,
    //     condition: None
    // };
    //
    // let tx2 =TransactionRequest {
    //     from: accounts[0],
    //     to: Some(accounts[1]),
    //     gas: None,
    //     gas_price: None,
    //     value: Some(U256::from(10000)),
    //     data: None,
    //     nonce: None,
    //     condition: None
    // };
    //
    //
    // let tx_hash = web3.eth().send_transaction(tx).await?;
    // // let tx_hash = web3.eth().send_transaction(tx2).await?;
    // let balance_after = web3.eth().balance(accounts[0], None).await?;
    //
    // println!("TX Hash: {:?}", tx_hash);
    // println!("Balance before: {}", balance_before);
    // println!("Balance after: {}", balance_after);


    // Get the contract bytecode for instance from Solidity compiler
    let bytecode = include_str!("./res/SimpleStorage.bin");
    web3.personal().unlock_account(accounts[0], "", None).await?;
    println!("Deploying contract!");
    // Deploying a contract
    // let contract = Contract::deploy(web3.eth(), include_bytes!("./res/SimpleStorage.abi"))?
    //     .confirmations(0)
    //     .poll_interval(time::Duration::from_secs(1))
    //     .options(Options::with(|opt| opt.gas = Some(3_000_000.into())))
    //     .sign_and_execute(bytecode, (), accounts[0], "")
    //     .await?;
    
    // get an existing contract
    let contract = Contract::from_json(web3.eth(), "0x093514489c4b42ff54f942f4f91de3f89c797aab".parse().unwrap(), include_bytes!("./res/SimpleStorage.abi"))?;

    println!("Deployed at: {:?}", contract.address());

    // interact with the contract
    let result = contract.query("get", (), None, Options::default(), None);
    let storage: U256 = result.await?;
    println!("Get Storage: {}", storage);

    // Change state of the contract
    let tx = contract.call("set", (storage * 2,), accounts[0], Options::default()).await?;
    println!("TxHash: {}", tx);

    // consider using `async_std::task::sleep` instead.
    //std::thread::sleep(std::time::Duration::from_secs(5));

    // View changes made
    let result = contract.query("get", (), None, Options::default(), None);
    let storage: U256 = result.await?;
    println!("Get again: {}", storage);

    Ok(())
}
