use anchor_lang::prelude::Pubkey;

pub const POOL_SEED: &str = "pool";

pub const MAX_BUYER_COUNT: usize = 100;
pub const MAX_TOTAL_TICKET: u32 = 1000000000;
pub const MAX_RAFFLE_ID_LEN: usize = 50;
pub const DECIMAL: u64 = 1000000000;
pub const SLOTHASH_PUBKEY: Pubkey = anchor_lang::solana_program::pubkey!("SysvarS1otHashes111111111111111111111111111"); 

// ======================================== mainnet ===================================================================//
pub const ADMIN_KEY: Pubkey = anchor_lang::solana_program::pubkey!("C8HXcXRqA6UjWAf1NTQXY7i4DMvMY9x3zbUhj9dyw2Yi"); 
pub const COMMUNITY_KEY: Pubkey = anchor_lang::solana_program::pubkey!("3XN3bRqf6Nnf8ZM9jjwf8sP1fT7oQ6m7XgpPfEUmJiob"); 
pub const COLLECTION_KEY: Pubkey = anchor_lang::solana_program::pubkey!("CaYsVNkgS5yBMK1BVTmbpapumjbyXNsBFZ2W1zbbk374"); //mainnet
pub const PAY_TOKEN: Pubkey = anchor_lang::solana_program::pubkey!("9aeip1QTVXNUVbcQ14UMDssmxNv4ve7sg8cVyfHoeNmT");  //mainnet

pub const WITHDRAW_KEY: Pubkey = anchor_lang::solana_program::pubkey!("C8HXcXRqA6UjWAf1NTQXY7i4DMvMY9x3zbUhj9dyw2Yi");

pub const SOL_KEY: Pubkey = anchor_lang::solana_program::pubkey!("So11111111111111111111111111111111111111112");

// ======================================== devnet ===================================================================//
// pub const ADMIN_KEY: Pubkey = anchor_lang::solana_program::pubkey!("C8HXcXRqA6UjWAf1NTQXY7i4DMvMY9x3zbUhj9dyw2Yi");
// pub const COMMUNITY_KEY: Pubkey = anchor_lang::solana_program::pubkey!("2XtHzHeZMAqGgdUztQTjbMGAyYo8SZSmveosuKRN25MQ"); 
// pub const COLLECTION_KEY: Pubkey = anchor_lang::solana_program::pubkey!("91jNpHwSpuUFY8tyEa2zrAjHnufQd8QtDaoptbxqiXVT");
// pub const PAY_TOKEN: Pubkey = anchor_lang::solana_program::pubkey!("55u5jMiJtwsvyo834R2mmcrxMGu7x2KvbrguJNbHFnEJ");  



