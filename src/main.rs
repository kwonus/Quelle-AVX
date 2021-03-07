use futures::prelude::*;
use gotham::hyper::{body, HeaderMap, Method, Uri, Version};
use std::pin::Pin;

use gotham::handler::HandlerFuture;
use gotham::helpers::http::response::{create_empty_response, create_response};
use gotham::router::builder::{build_simple_router, DefineSingleRoute, DrawRoutes};
use gotham::router::Router;
use gotham::state::{FromState, State};

// for askama
use askama::Template; // bring trait in scope
pub use askama::*;

pub use gotham::handler::IntoResponse;
pub use gotham::hyper::{Body, Response, StatusCode};
// ens askama

extern crate serde;
#[macro_use]
extern crate rmp_serde as rmps;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};
use gotham::start;
use gotham::hyper::body::Buf;

#[derive(Serialize, Deserialize, Debug)]
struct Searches {
    x: u32,
    y: i32,
    message: String,
}

const QUELLE_DISPLAY: &str = "Hello Display! (from Quelle)";

//  askama_gotham
#[derive(Template)] // this will generate the code...
#[template(path = "index.html")] // using the template in this path, relative
// to the `templates` dir in the crate root
struct IndexTemplate<'a> { // the name of the struct can be anything
    title: &'a str, // the field name should match the variable name in your template
}

pub fn respond<T: Template>(t: &T, _: &str) -> Response<Body> {
    match t.render() {
        Ok(body) => Response::builder()
            .status(StatusCode::OK)
            .body(body.into())
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(vec![].into())
            .unwrap(),
    }
}
//  end askama_gotham

/// Show the GET request components by printing them.
fn get_index_handler(state: State) -> (State, Response<Body>) {
    print_request_elements(&state);

    let html = IndexTemplate { title: "Quelle Search Provider for Digital-AV" };
    let response = respond(&html,"html");
    (state, response)
}

/// Extract the main elements of the request except for the `Body` (Gotham)
fn print_request_elements(state: &State) {
    let method = Method::borrow_from(state);
    let uri = Uri::borrow_from(state);
    let http_version = Version::borrow_from(state);
    let headers = HeaderMap::borrow_from(state);
    println!("Method: {:?}", method);
    println!("URI: {:?}", uri);
    println!("HTTP Version: {:?}", http_version);
    println!("Headers: {:?}", headers);
}

/// Extracts the elements of the POST request and prints them
fn post_status_handler(mut state: State) -> Pin<Box<HandlerFuture>> {
    print_request_elements(&state);
    let f = body::to_bytes(Body::take_from(&mut state)).then(|full_body| match full_body {
        Ok(valid_body) => {
            let body_content = String::from_utf8(valid_body.to_vec()).unwrap();
            println!("Body: {}", body_content);
            let res = create_empty_response(&state, StatusCode::OK);
            future::ok((state, res))
        }
        Err(e) => future::err((state, e.into())),
    });

    f.boxed()
}
/// Extracts the elements of the POST request and prints them
fn post_search_handler(mut state: State) -> Pin<Box<HandlerFuture>> {
    print_request_elements(&state);
    let f = body::to_bytes(Body::take_from(&mut state)).then(|full_body| match full_body {
        Ok(valid_body) => {
            let deserializedPoint: Searches = rmp_serde::from_read_ref(valid_body.bytes()).unwrap();
            println!("x = {}; y = {}; message = {}", deserializedPoint.x, deserializedPoint.y, deserializedPoint.message);
            let res = create_empty_response(&state, StatusCode::OK);
            future::ok((state, res))
        }
        Err(e) => future::err((state, e.into())),
    });

    f.boxed()
}


/// Show the GET request components by printing them.
fn get_display_handler(state: State) -> (State, Response<Body>) {
    print_request_elements(&state);

    let res = create_response(
        &state,
        StatusCode::OK,
        mime::TEXT_PLAIN,
        String::from(QUELLE_DISPLAY),
    );
    (state, res)
}

/// Create a `Router`
fn router() -> Router {
    build_simple_router(|route| {
        route.associate("/status", |assoc| {
            assoc.post().to(post_status_handler);
        });
        route.associate("/search", |assoc| {
            assoc.post().to(post_search_handler);
        });
        route.associate("/display", |assoc| {
            assoc.get().to(get_display_handler);
        });
        route.associate("/", |assoc| {
            assoc.get().to(get_index_handler);
        });
    })
}

/// Start a server and use a `Router` to dispatch requests
pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    start(addr, router())
}

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::TestServer;

    #[test]
    fn get_request() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .get("http://localhost")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn post_request() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .post("http://localhost", "", mime::TEXT_PLAIN)
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}

/*
fn main() {
    let point = Searches { x: 1, y: 2 };

    let bytes= rmp_serde::to_vec(&point).unwrap();

    let deserializedPoint: Searches = rmp_serde::from_read_ref(&bytes).unwrap();
    println!("x = {}; y = {}", deserializedPoint.x, deserializedPoint.y);
}
 */