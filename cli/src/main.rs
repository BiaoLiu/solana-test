use std::str::FromStr;
use solana_program::{
    pubkey, instruction,
};
use solana_sdk::{
    signature::Signer,
    signer::keypair,
    transaction,
};
use solana_rpc_client::rpc_client;


// const RPC_ADDR: &str = "https://api.devnet.solana.com";
const RPC_ADDR: &str = "http://localhost:8899";

fn main() {
    let helloworld = pubkey::Pubkey::from_str("FWC1k1uwMagMR7GRv9edqHjoc92LEa3V3V8VJ89BwgcY").unwrap();
    let key: &[u8] = &[65, 137, 130, 147, 240, 218, 54, 189, 99, 103, 145, 144, 56, 252, 71, 247, 38, 19, 249, 202, 162, 57, 57, 211, 247, 96, 91, 67, 113, 201, 238, 114, 86, 128, 42, 15, 189, 117, 129, 133, 217, 252, 117, 72, 202, 158, 118, 106, 66, 213, 47, 227, 36, 239, 119, 184, 42, 224, 243, 118, 252, 93, 139, 186];
    let me = keypair::Keypair::from_bytes(key).unwrap();
    println!("me:{}", me.pubkey());

    let client = rpc_client::RpcClient::new(RPC_ADDR);

    let account_metas = vec![
        instruction::AccountMeta::new(me.pubkey(), true),
    ];
    let instruction = instruction::Instruction::new_with_bytes(
        helloworld,
        "hello".as_bytes(),
        account_metas,
    );
    let ixs = vec![instruction];

    let latest_blockhash = client.get_latest_blockhash().unwrap();
    let sig = client.send_and_confirm_transaction(&transaction::Transaction::new_signed_with_payer(
        &ixs,
        Some(&me.pubkey()),
        &[&me],
        latest_blockhash,
    )).unwrap();
    println!("tx:{}", sig);
}
