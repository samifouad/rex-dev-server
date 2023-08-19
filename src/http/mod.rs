use hyper::{server::conn::AddrStream, Body, Request, Response};
use std::convert::Infallible;
use std::fs::File;
use std::io::{self, Read};
use std::net::SocketAddr;
use tokio::task;

use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};

#[derive(Debug, Clone)]
pub struct Route {
    path: String,
    file: String,
}

#[derive(Debug, Clone)]
pub struct Http {
    pub html_routes: Vec<Route>,
    pub api_routes: Vec<Route>,
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
            file: file.to_string(),
        };
        self.html_routes.push(route);
    }

    pub fn api(&mut self, path: &str, file: &str) {
        let route = Route {
            path: path.to_string(),
            file: file.to_string(),
        };
        self.api_routes.push(route);
    }
}

#[derive(Clone, Debug)]
struct AppContext {
    routes: Vec<Route>,
}

fn read_file_to_bytes(filename: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

async fn router(
    context: AppContext,
    addr: SocketAddr,
    req: Request<Body>,
) -> Result<Response<Body>, Infallible> {
    println!("\n\n New Request from: {:?}", addr);

    // for route in context.routes {
    //     if route.path == req.uri().to_string() {
    //         println!("Matching Route Found!");
    //         println!("Route: {:?}", route);
    //         println!("Will load resource: {:?}", route.file);
    //     } else {
    //         // do nothing
    //     }
    // }

    // Clone the routes
    let routes = context.routes.clone();

    // Extract the route's file path before getting into async operations
    let maybe_file_path = context
        .routes
        .iter()
        .find(|&route| route.path == req.uri().to_string())
        .map(|route| route.file.clone());

    if let Some(file_path) = maybe_file_path {
        // ... your code ...

        // Now, you can read the file using the extracted path
        let file_data = task::spawn_blocking(move || read_file_to_bytes(&file_path))
            .await
            .unwrap_or_else(|_| Err(io::Error::new(io::ErrorKind::Other, "404!!")));

        match file_data {
            Ok(bytes) => Ok(Response::new(Body::from(bytes))),
            Err(e) => {
                eprintln!("Error reading for: {}", req.uri());
                Ok(Response::new(Body::from("404!")))
            }
        }
    } else {
        println!("No Matching Route Found");
        Ok(Response::new(Body::from("404!")))
    }
}

pub async fn start(routes: Vec<Route>) {
    let app_context = AppContext {
        routes: routes.clone(),
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
        let service = service_fn(move |req| router(context.clone(), addr, req));

        // Return the service to hyper.
        async move { Ok::<_, Infallible>(service) }
    });

    // Then bind and serve...
    let server = Server::bind(&addr).serve(make_service);

    // And run forever...
    server.await.unwrap();
}
