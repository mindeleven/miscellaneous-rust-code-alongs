// https://github.com/codemoonsxyz/toy-http-rs/blob/master/http-lib/src/middleware/middleware.rs
use crate::response::Response;
use crate::request::Request;
use crate::server::FutureResponse;
use std::future::Future;
use std::pin::Pin;
use std::ops::Deref;
use std::clone::Clone;

pub type FutureRequest<'a> = Pin<Box<dyn Future<Output = Result<Request, Box<dyn std::error::Error + Send + 'a>>> + Send + 'a>>;

// optional middleware we can add that will intercept our requests and run some particlar logic
// or intercept our response and run some particlar logic
pub trait Middleware: Send + Sync{
    fn on_request<'a>(&self, request: Request) -> FutureRequest<'a>;
    fn on_response<'a>(&self, response:Response) -> FutureResponse<'a>; 

}