use dotenv::dotenv;
use std::env;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcProgramAccountsConfig, RpcAccountInfoConfig};
use solana_client::rpc_filter::{RpcFilterType, Memcmp};
use solana_account_decoder::UiAccountEncoding;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use tokio::time::{sleep, Duration};
use std::str::FromStr;

async fn listen_for_new_pools(rpc_url: String, raydium_lp_program: String) {
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let min_liquidity: u64 = 5_000_000_000; // 5 SOL
    let min_liquidity_bytes = min_liquidity.to_le_bytes(); 

    let filters: Option<Vec<RpcFilterType>> = Some(vec![
        RpcFilterType::DataSize(165), 
        RpcFilterType::Memcmp(Memcmp::new_base58_encoded(
            96, 
            &min_liquidity_bytes, 
        )),
    ]);

    loop {
        match client.get_program_accounts_with_config(
            &Pubkey::from_str(&raydium_lp_program).unwrap(),
            RpcProgramAccountsConfig {
                filters: filters.clone(),
                account_config: RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64),
                    ..RpcAccountInfoConfig::default()
                },
                with_context: None,
                sort_results: None,
            },
        ) {
            Ok(pools) => {
                println!("Found {} new liquidity pools!", pools.len());

                for (__index, (pubkey, _)) in pools.iter().enumerate() {
                    println!("Pool Address: {}", pubkey);
                }
            }
            Err(e) => println!("Error fetching LP accounts: {:?}", e),
        }
        // Poll every ..
        sleep(Duration::from_secs(5)).await; 
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok(); 

    let rpc_url = env::var("RPC_URL").expect("Missing RPC_URL in .env");
    let raydium_lp_program = env::var("RAYDIUM_LP_PROGRAM").expect("Missing RAYDIUM_LP_PROGRAM in .env");

    println!("Listening for new liquidity pools...");
    
    listen_for_new_pools(rpc_url, raydium_lp_program).await;
}
