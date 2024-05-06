use super::server::Handler;
use super::http::{
    Method, Request, Response, StatusCode
};
pub struct WebsiteHandler;

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
                        Some("<h1>It's alive!!!!!!</h1><h2>handle request via match request.method().</h2>".to_string())
                    )
                },
                "/hello" => {
                    Response::new(
                        StatusCode::OK,
                        Some("<h1>A simple hello, nothing special</h2>".to_string())
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