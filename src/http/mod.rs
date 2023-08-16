use hyper::{Response, Request, Body, server::conn::AddrStream};
use std::net::SocketAddr;
use std::convert::Infallible;

use hyper::{service::{service_fn, make_service_fn}, Server};

#[derive(Debug, Clone)]
pub struct Route {
    path: String,
    file: String
}

#[derive(Debug, Clone)]
pub struct Http {
    pub html_routes: Vec<Route>,
    pub api_routes: Vec<Route>
}

impl Http {
    pub fn new() -> Self {
        Self { 
            html_routes: Vec::new(),
            api_routes: Vec::new(),
        }
    }

    pub fn html(&mut self, path: &str, file: &str) {
        let route = Route {
            path: path.to_string(),
            file: file.to_string()
        };
        self.html_routes.push(route);
    }

    pub fn api(&mut self, path: &str, file: &str) {
        let route = Route {
            path: path.to_string(),
            file: file.to_string()
        };
        self.api_routes.push(route);
    }
}

#[derive(Clone, Debug)]
struct AppContext {
    routes: Vec<Route>
}

async fn router(
    context: AppContext,
    addr: SocketAddr,
    req: Request<Body>
) -> Result<Response<Body>, Infallible> {
    println!("\n\n New Request from: {:?}", addr);
    
    for route in context.routes {
        if route.path == req.uri().to_string() {
            println!("Matching Route Found!");
            println!("Route: {:?}", route);
            println!("Will load resource: {:?}", route.file);
        } else {
            // do nothing
        }
    } 
    
    println!("Req Method: {:?}", req.method());
    println!("Req URI: {:?}", req.uri());
    Ok(Response::new(Body::from("Hello World")))
}

pub async fn start(http_routes: Vec<Route>) {

    let app_context = AppContext {
        routes: http_routes.clone()
    };

    // Construct our SocketAddr to listen on...
    let addr = SocketAddr::from(([127, 0, 0, 1], 9001));

    let context = app_context.clone();

    // A `MakeService` that produces a `Service` to handle each connection.
    let make_service = make_service_fn(move |conn: &AddrStream| {
        // We have to clone the context to share it with each invocation of
        // `make_service`. If your data doesn't implement `Clone` consider using
        // an `std::sync::Arc`.
        let context = context.clone();

        // You can grab the address of the incoming connection like so.
        let addr = conn.remote_addr();

        // Create a `Service` for responding to the request.
        let service = service_fn(move |req| {
            router(context.clone(), addr, req)
        });

        // Return the service to hyper.
        async move { Ok::<_, Infallible>(service) }
    });

    // Then bind and serve...
    let server = Server::bind(&addr).serve(make_service);

    // And run forever...
    server.await.unwrap();
} 