use lastbuy_sdk::instructions::*;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_keypair::{EncodableKey, Keypair, Signer};

#[tokio::test]
#[ignore]
async fn test_initialize_global() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    
    let url = std::env::var("TEST_RPC_URL")?;
    let payer_keypair_path = std::env::var("TEST_PAYER_PATH")?; 

    let rpc_client = RpcClient::new(url);
    
    let signer = Keypair::read_from_file(payer_keypair_path)?;
    let signer_pubkey = signer.pubkey();

    // let initialize_global_ix = 

    Ok(())
}