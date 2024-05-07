use std::fs;

use super::server::Handler;
use super::http::{
    Method, Request, Response, StatusCode
};
pub struct WebsiteHandler {
    pub public_path: String
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        WebsiteHandler {
            public_path
        }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        // read_to_string returns result
        // ok() returns an option from the Result
        fs::read_to_string(path).ok()
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        /* 
        Response::new(
            StatusCode::OK,
            Some("<h1>It's alive!!!!!!</h1><h2>handle request via WebsiteHandler.</h2>".to_string())
        )
        */
        // mathing on the incoming method
        match request.method() {
            Method::GET => match request.path() {
                "/" => {
                    Response::new(
                        StatusCode::OK,
                        // Some("<h1>It's alive!!!!!!</h1><h2>handle request via match request.method().</h2>".to_string())
                        // returning the index.html file instead of a string
                        self.read_file("index.html")
                    )
                },
                "/hello" => {
                    Response::new(
                        StatusCode::OK,
                        // Some("<h1>A simple hello, nothing special</h2>".to_string())
                        // returning the hello.html file instead of a string
                        self.read_file("hello.html")
                    )
                },
                _ => {
                    Response::new(
                        StatusCode::NotFound,
                        None
                    )
                }
            },
            _ => {
                Response::new(
                    StatusCode::NotFound,
                    None
                )
            }
        }
    }
}

// implementing routing logic in the handler