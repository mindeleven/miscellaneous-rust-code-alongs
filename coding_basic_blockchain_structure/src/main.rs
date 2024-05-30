/// coding along with the blog post Coding a P2P blockchain in Rust (Part 1)
/// https://medium.com/@prabhueshwarla/coding-a-p2p-blockchain-in-rust-part-1-2ecc8f6005ea
/// by Prabhu Eshwarla that has been published on August 19th, 2019
/// 
/// the use case of the blockchain this tutorail indents to build: SHARED DATA
/// -> scenarios where data needs to be shared among a group of participants
/// -> (1) in a private setting (eg. within an organisation)
/// -> (2) in a permissioned setting (eg. a consortium chain) 
/// -> (3) in a public setting (eg. a public notary)
/// 
/// the reason why we're building a blockchain is that we want to store 
/// identical copies of shared data across different nodes on a peer-to-peer network,
/// managed by a common protocol, 
/// and ensure the data once stored, is immutable

pub struct Transaction {
    pub transaction_id: String,
    pub transaction_timestamp: i64,
    pub transaction_details: String,
}

pub struct Block {
    pub block_number: u64,
    block_timestamp: i64,
    pub block_nonce: u64,
    pub transaction_list: Vec<Transaction>,
    previous_block_hash: String,
}

fn main() {
    println!("Hello, world!");
}
