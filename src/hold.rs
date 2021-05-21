// extern crate web3;

use web3::types::{TransactionRequest, U256};

#[tokio::main]
async fn main() -> web3::Result<()> {
    let transport = web3::transports::Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(transport);

    println!("Calling accounts.");
    let mut accounts = web3.eth().accounts().await?;

    accounts.push("00a329c0648769a73afac7f9381e08fb43dbea76".parse().unwrap());

    println!("Accounts: {:?}", accounts);

    // println!("Calling balance.");
    // for account in accounts.into_iter() {
    //     let balance = web3.eth().balance(account, None).await?;
    //     println!("Balance of {:?}: {}", account, balance);
    // }


    let balance_before = web3.eth().balance(accounts[0], None).await?;

    let tx = TransactionRequest {
        from: accounts[0],
        to: Some(accounts[1]),
        gas: None,
        gas_price: None,
        value: Some(U256::from(10000)),
        data: None,
        nonce: None,
        condition: None
    };

    let tx_hash = web3.personal().send_transaction(tx, "").await?;

    let balance_after = web3.eth().balance(accounts[0], None).await?;

    println!("TX Hash: {:?}", tx_hash);
    println!("Balance before: {}", balance_before);
    println!("Balance after: {}", balance_after);

    Ok(())
}
