use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// a request is a message send by a client to a server
#[derive(Debug)]
pub struct Request {
    pub method: HttpMethod,
    pub uri: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default,PartialEq,Eq, Hash)]
pub enum HttpMethod {
    #[default]
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
    PATCH,
    OTHER(String)
}