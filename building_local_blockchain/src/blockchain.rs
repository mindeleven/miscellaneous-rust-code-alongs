#![allow(unused_imports)]
use serde_derive::Serialize;
use time;
use serde;
use serde_json;
use sha2::{self, Sha256, Digest};
use std::fmt::Write;

// structuring data for the transactions
#[derive(Debug, Clone, Serialize)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

#[derive(Debug, Serialize)]
pub struct Blockheader {
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32,
}

#[derive(Debug, Serialize)]
pub struct Block {
    header: Blockheader,
    count: u32,
    transactions: Vec<Transaction>,
}

pub struct Chain {
    chain: Vec<Block>,
    curr_transaction: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32,
}

// implementing chain functionality
impl Chain {
    // creating a new miner
    pub fn new(miner_addr: String, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            curr_transaction: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100.0,
        };
        
        chain.generate_new_block();

        chain
    }
    
    // returns bool that tells us wether a new block has been generated or not
    fn generate_new_block(&mut self) -> bool {
        
        
        true
    }
}