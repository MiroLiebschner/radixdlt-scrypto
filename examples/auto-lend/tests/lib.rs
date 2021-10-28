use radix_engine::ledger::*;
use radix_engine::transaction::*;
use scrypto::prelude::*;

#[test]
fn deposit_test() {
    // Set up environment.
    let mut ledger = InMemoryLedger::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, 0, 0);
    let pub_key = executor.new_public_key();
    let account = executor.create_account(pub_key);
    let package = executor.publish_package(include_code!());

    // we can't use ResourceBuilder in tests, so this is a workaround to create resources
    let create_token_b_tx = TransactionBuilder::new(&executor)
        .call_function(package, "Token", "new", vec!["Token B".to_string(), "tokenB".to_string()], Some(account))
        .deposit_all(account)
        .build(vec![pub_key])
        .unwrap();
    let create_token_b_tx_receipt = executor.run(create_token_b_tx, false).unwrap();
    println!("{:?}\n", create_token_b_tx_receipt);
    let token_b_address = create_token_b_tx_receipt.resource_def(0).unwrap();

    let create_token_c_tx = TransactionBuilder::new(&executor)
        .call_function(package, "Token", "new", vec!["Token C".to_string(), "tokenC".to_string()], Some(account))
        .deposit_all(account)
        .build(vec![pub_key])
        .unwrap();
    let create_token_c_tx_receipt = executor.run(create_token_c_tx, false).unwrap();
    let token_c_address: Address = create_token_c_tx_receipt.resource_def(0).unwrap();

    let create_auto_lend_tx = TransactionBuilder::new(&executor)
        .call_function(
            package, 
            "AutoLend", 
            "new", 
            vec![token_b_address.to_string(), token_c_address.to_string()], None
        )
        .build(vec![pub_key])
        .unwrap();
    let create_auto_lend_tx_receipt = executor.run(create_auto_lend_tx, false).unwrap();
    println!("{:?}\n", create_auto_lend_tx_receipt);
    let auto_lend_address = create_auto_lend_tx_receipt.component(0).unwrap();

    let tx = TransactionBuilder::new(&executor)
        .call_method(auto_lend_address, "deposit", vec![format!("{},{}", Amount::from(100), token_b_address), ], Some(account))
        .deposit_all(account)
        .build(vec![pub_key])
        .unwrap();

    let receipt3 = executor.run(tx, false).unwrap();
    println!("{:?}\n", receipt3);

}