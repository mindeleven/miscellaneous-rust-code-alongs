use std::fs;

use super::server::Handler;
use super::http::{
    Method, Request, Response, StatusCode
};
pub struct WebsiteHandler {
    pub public_path: String
}

/// preparing for a traversal vulnerability attack with fs::canonicalize(path)
/// "A path traversal vulnerability allows an attacker to access files on your web server 
/// to which they should not have access. They do this by tricking either the web server 
/// or the web application running on it into returning files that exist outside of the web 
/// root folder."

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        WebsiteHandler {
            public_path
        }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                // checking if path starts with public path
                if path.starts_with(&self.public_path) {
                    // read_to_string returns result
                    // ok() returns an option from the Result
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory traversal attack was attempted: {}", file_path);
                    None
                }
            },
            Err(_) => None
        }

       
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
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::OK, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None)
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