use lastbuy_sdk::pda::*;
use solana_pubkey::{Pubkey, pubkey};

#[test]
#[ignore]
fn test_static_raydium_cpmm_authority_derivation() -> Result<(), Box<dyn std::error::Error>> {
    let cpmm_program_id = pubkey!("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C");
    let seed = "vault_and_lp_mint_auth_seed"; 
    
    let expected = pubkey!("GpMZbSM2GgvTKHJirzeGfMFoaZ8UR2X7F4v8vHTvxFbL");
    
    let derived = Pubkey::find_program_address(
        &[seed.as_ref()], 
        &cpmm_program_id
    ).0;

    assert_eq!(expected, derived);

    Ok(())
}

#[test]
#[ignore]
fn test_static_raydium_lock_authority_derivation() -> Result<(), Box<dyn std::error::Error>> {
    let lock_cp_program_id = pubkey!("LockrWmn6K5twhz3y9w1dQERbmgSaRkfnTeTKbpofwE");
    let seed = "lock_cp_authority_seed"; 
    
    let expected = pubkey!("3f7GcQFG397GAaEnv51zR6tsTVihYRydnydDD1cXekxH");
    
    let derived = Pubkey::find_program_address(
        &[seed.as_ref()], 
        &lock_cp_program_id
    ).0;

    assert_eq!(expected, derived);

    Ok(())
}

#[test]
#[ignore]
fn test_raydium_cpmm_authority_derivation() -> Result<(), Box<dyn std::error::Error>> {
    let cpmm_program_id = pubkey!("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C");
    
    let expected = pubkey!("GpMZbSM2GgvTKHJirzeGfMFoaZ8UR2X7F4v8vHTvxFbL");
    
    let derived = derive_raydium_cpmm_authority(&cpmm_program_id).0; 

    assert_eq!(expected, derived);

    Ok(())
}

#[test]
#[ignore]
fn test_raydium_lock_authority_derivation() -> Result<(), Box<dyn std::error::Error>> {
    let lock_cp_program_id = pubkey!("LockrWmn6K5twhz3y9w1dQERbmgSaRkfnTeTKbpofwE");

    let expected = pubkey!("3f7GcQFG397GAaEnv51zR6tsTVihYRydnydDD1cXekxH");
    
    let derived = derive_raydium_lock_authority(&lock_cp_program_id).0;

    assert_eq!(expected, derived);

    Ok(())
}