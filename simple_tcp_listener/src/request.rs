use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::http::HttpMethod;

/// a request is a message send by a client to a server
#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct Request {
    pub method: HttpMethod,
    pub uri: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>
}
