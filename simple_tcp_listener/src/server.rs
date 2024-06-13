use crate::request::*;
use crate::response::Response;

use crate::middleware::middleware::Middleware;
use crate::middleware::logger::LoggerMiddleware;

use crate::http::*;

use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;
use std::cell::RefCell;

pub type FutureResponse<'a> = Pin<Box<dyn Future<Output = Result<Response, HttpError>> + Send + 'a>>;

pub type RouteHandler = fn(Request) -> FutureResponse<'static>;

#[derive(Clone)]
pub struct Server {
    // address we bind our open port to
    address: SocketAddr,
    // hashmap of routes, binding a perticular route to a route handler
    routes: HashMap<Route, RouteHandler>,
    middleware: Arc<Vec<Box<dyn Middleware>>>,
}

#[derive(Eq, Hash, PartialEq,Clone)]
struct Route {
    method: HttpMethod,
    path: String,
}
