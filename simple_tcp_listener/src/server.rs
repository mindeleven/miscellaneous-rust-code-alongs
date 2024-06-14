// https://github.com/codemoonsxyz/toy-http-rs/blob/master/http-lib/src/server.rs
use crate::request::*;
use crate::response::Response;

use crate::middleware::middleware::Middleware;
use crate::middleware::logger::LoggerMiddleware;

use crate::http::*;

use std::sync::Mutex;
use std::sync::Arc;
use std::cell::RefCell;
use httparse;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fmt;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::{fs, io::prelude::*, thread, time::Duration};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{channel, Sender};

use std::rc::Rc;

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

impl Server {
    // run function to spin up server and run tcp listener
    pub async fn run(&self) -> std::io::Result<()> {
        let address = self.address;
        let listener = TcpListener::bind(address).await?;
        println!("Server listening on {}", address.to_string());

        
        loop {
            let (mut stream, _) = listener.accept().await?;
            let routes = self.routes.clone();
            let middleware = Arc::clone(&self.middleware);
           
            tokio::spawn(async move {
                let mut buffer = [0; 1024];
                let _ = stream.read(&mut buffer).await.unwrap();

                let request = parse_request(&buffer).unwrap();
              
                let future_response = handle_route(request, &routes, &middleware).await;

                match future_response.await {
                    Ok(response) => {

                        let response_string = format!(
                            "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
                            response.status_code,
                            response.status_text,
                            response.headers
                                .iter()
                                .map(|(key, value)| format!("{}: {}", key, value))
                                .collect::<Vec<_>>()
                                .join("\r\n"),
                            response.body.unwrap_or_default()
                        );

                        
                        stream.write(response_string.as_bytes()).await.unwrap();
                        stream.flush().await.unwrap();
                        // Do something with the response
                    }

                    Err(e) => {}
                }
            });
        }
    }
}

fn parse_request(buffer: &[u8]) -> Result<Request, Box<dyn std::error::Error>> {
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);

    let res = match req.parse(&buffer)? {
        httparse::Status::Complete(amt) => amt,
        httparse::Status::Partial => {
            return Err("Request is incomplete".into());
        }
    };

    let method = match req.method.ok_or("Method not found")? {
        "GET" => HttpMethod::GET,
        "POST" => HttpMethod::POST,
        "PUT" => HttpMethod::PUT,
        "DELETE" => HttpMethod::DELETE,
        _ => HttpMethod::OTHER(req.method.unwrap().to_string()),
    };
    let uri = req.path.ok_or("URI not found")?.to_string();
    let version = req.version.ok_or("Version not found")?.to_string();

    let mut headers_map = HashMap::new();
    for header in req.headers.iter() {
        let name = header.name.to_string();
        let value = std::str::from_utf8(header.value)?.to_string();
        headers_map.insert(name, value);
    }

    let body = if res < buffer.len() {
        Some(String::from_utf8(buffer[res..].to_vec())?)
    } else {
        None
    };
    

    Ok(Request {
        method,
        uri,
        version,
        headers: headers_map,
        body,
    })
}

async fn handle_route<'a>(request:Request, routes: &'a HashMap<Route, RouteHandler>, middleware: &'a Vec<Box<dyn Middleware>>) -> FutureResponse<'a>{
    // Find the route handler based on the request path and call it
    if let Some(handler) = routes.get(&Route { method: request.method.clone(), path: request.uri.clone() }) {
        
       

        for mw in middleware {
             
            mw.on_request(request.clone());
        
        }

        handler(request)
    } else {
        Box::pin(async move {
           
            Err(HttpError::InternalServerError(
                HttpStatusCode::InternalServerError,
                "Internal Server Error"
            ))
          
        })
       
    }
}


