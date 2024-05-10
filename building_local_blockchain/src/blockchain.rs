#![allow(unused_imports)]
use serde_derive::Serialize;
use time;
use serde;
use serde_json;
use sha2::{self, Sha256, Digest};
use core::hash;
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
    pub fn generate_new_block(&mut self) -> bool {
        let header = Blockheader {
            timestamp: time::now().to_timespec().sec,
            nonce: 0,
            pre_hash: self.last_hash(),
            merkle: String::new(),
            difficulty: self.difficulty,
        };

        let reward_trans = Transaction {
            sender: String::from("Root"),
            receiver: self.miner_addr.clone(),
            amount: self.reward,
        };
        
        // creating a block
        let mut block = Block {
            header: header,
            count: 0,
            transactions: vec![],
        };
        
        // setting all the information for the block
        block.transactions.push(reward_trans);
        block.transactions.append(&mut self.curr_transaction);
        block.count = block.transactions.len() as u32;
        block.header.merkle = Chain::get_merkle(block.transactions.clone());
        Chain::proof_of_work(&mut block.header);

        // printing block with extended debugger
        println!("{:#?}", block);

        // adding the block to the chain
        self.chain.push(block);

        true
    }
    
    // functionality to generate a new transaction
    pub fn new_transaction(
        &mut self,
        sender: String,
        receiver: String,
        amount: f32,
    ) -> bool {
        self.curr_transaction.push(Transaction {
            sender,
            receiver,
            amount,
        });

        true
    }

    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => {
                return String::from_utf8(vec![48; 64]).unwrap();
            }
        };
        
        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        true
    }

    fn hash(blockheader: &Blockheader) -> String {
        unimplemented!()
    }

    fn proof_of_work(blockheader: &mut Blockheader) {}

    fn get_merkle(curr_transaction: Vec<Transaction>) -> String {
        let mut merkle = Vec::new();
        for t in curr_transaction {
            let hash =Chain::hash(t);
            merkle.push(hash);
        }

        if merkle.len() % 2 == 1 {
            let last = merkle.last().clone().unwrap();
            merkle.push(last);
        }

        while merkle.len() > 1 {
            let mut h1 = merkle.remove(0);
            let mut h2 = merkle.remove(0);
            h1.push_str(&mut h2);
            let nh = Chain::hash(&h1);
            merkle.push(nh);
        }

        merkle.pop().unwrap()
    }

}