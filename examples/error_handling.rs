use hyper::{Body, Request, Response, Server, StatusCode};
use routerify::{Router, RouterService};
use std::net::SocketAddr;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Home Error")]
    HomeError,
    #[error("API Error")]
    ApiError(#[source] api::Error)
}

impl From<api::Error> for Error {
    fn from(e: api::Error) -> Self {
        Self::ApiError(e)
    }
}

// A handler for "/" page.
async fn home_handler(_: Request<Body>) -> Result<Response<Body>, Error> {
    Err(Error::HomeError)
}

// A handler for "/about" page.
async fn about_handler(_: Request<Body>) -> Result<Response<Body>, Error> {
    Ok(Response::new(Body::from("About page")))
}

mod api {
    use super::*;

    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error("Test Error")]
        Test,
    }

    // A handler for "/fail" page
    async fn fail_handler(_: Request<Body>) -> Result<Response<Body>, Error> {
        Err(Error::Test)
    }

    pub fn router() -> Router<Body, super::Error> {
        // Create a router for API and specify the the handlers.
        Router::builder()
            .get("/fail", fail_handler)
            .build()
            .unwrap()
    }
}

fn format_cause_chain(err: impl std::error::Error) -> String {
    let mut lines = vec![format!("error: {}", err)];
    let mut source = err.source();
    while let Some(src) = source {
        lines.push(format!("  caused by: {}", src));
        source = src.source();
    }
    lines.join("\n")
}

// Define an error handler function which will accept the `routerify::RouterError`
// and generates an appropriate response.
async fn error_handler(err: routerify::RouterError<Error>) -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(format_cause_chain(err)))
        .unwrap()
}

fn router() -> Router<Body, Error> {
    // Create a router and specify the the handlers.
    Router::builder()
        .get("/", home_handler)
        .get("/about", about_handler)
        // Mount the api router at `/api` path.
        // Now the app can handle: `/api/fail` path.
        .scope("/api", api::router())
        // Specify the error handler to handle any errors caused by
        // a route or any middleware.
        .err_handler(error_handler)
        .build()
        .unwrap()
}

#[tokio::main]
async fn main() {
    let router = router();

    // Create a Service from the router above to handle incoming requests.
    let service = RouterService::new(router).unwrap();

    // The address on which the server will be listening.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    // Create a server by passing the created service to `.serve` method.
    let server = Server::bind(&addr).serve(service);

    println!("App is running on: {}", addr);
    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
    }
}
