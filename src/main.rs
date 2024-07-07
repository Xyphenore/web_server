// https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.ht

use crate::routes::{index::get as get_index, slow_request::get as get_slow_request};
use crate::server::{Debug, Method, WebServer};

mod requests;
mod routes;
mod server;
mod threads;

#[doc(hidden)]
static DEBUG: bool = false;

/// Executable script to start the Web server.
///
/// Add [`routes::index::get()`] and [`routes::slow_request::get()`] to the server.
/// The server listens on `127.0.0.1:8000`.
///
/// # Panics
///
/// - If any method ([`WebServer::serve()`] or [`WebServer::add_listener()`]) panics.
fn main() {
    WebServer::new(2, Debug::from(DEBUG))
        .add_listener(Method::get("/").unwrap(), get_index)
        .add_listener(Method::get("/slow_request").unwrap(), get_slow_request)
        .serve();
}
