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