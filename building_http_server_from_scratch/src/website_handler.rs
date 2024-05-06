use super::server::Handler;
use super::http::{
    Request, Response, StatusCode
};
pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(
            StatusCode::OK,
            Some("<h1>It's alive!!!!!!</h1><h2>handle request via WebsiteHandler.</h2>".to_string())
        )
    }
}