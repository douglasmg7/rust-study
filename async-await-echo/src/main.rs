use {
    hyper::{
        // Handle a 'Request' and return a 'Response'.
        service::{make_service_fn, service_fn},
        Body,
        Client,
        Request,
        Response,
        Server,
        Uri,
    },
    std::net::SocketAddr,
};

async fn server_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Always return successfully with a response containing a body with
    // a friendly greeting ;)
    // Ok(Response::new(Body::from("hello, world!")))

    let url_str = "http://www.rust-lang.org/en-US/";
    let url = url_str.parse::<Uri>().expect("failed to parse URL");

    let res = Client::new().get(url).await?;
    Ok(res)
}

async fn run_server(addr: SocketAddr) {
    // Create a server bound on the provided address.
    println!("Listening on http://{}", addr);
    let server_future = Server::bind(&addr)
        // Serve requests using our `async serve_req` function.
        // `serve` takes a type which implements the `MakeService` trait.
        // `make_service_fn` converts a closure into a type which
        // implements the `MakeService` trait. That closure must return a
        // type that implements the `Service` trait, and `service_fn`
        // converts a request-response function into a type that implements
        // the `Service` trait.
        .serve(make_service_fn(|_| async {
            Ok::<_, hyper::Error>(service_fn(server_req))
        }));
    if let Err(e) = server_future.await {
        eprintln!("server error: {}", e);
    }
}

#[tokio::main]
async fn main() {
    // Set the address to run our socket on.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // Call our `run_server` function, which returns a future.
    // As with every `async fn`, for `run_server` to do anything,
    // the returned future needs to be run using `await`;
    run_server(addr).await;
}