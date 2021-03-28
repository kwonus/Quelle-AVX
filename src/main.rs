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
// end askama

use mime;
use mime::Mime;

extern crate serde;
extern crate serde_json;
use serde_json::json;
use serde_json::Result;

#[macro_use]
extern crate rmp_serde;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};
use gotham::start;
use gotham::hyper::body::Buf;
use gotham::hyper::HeaderMap as hmap;
use gotham::hyper::header::CONTENT_TYPE;
use std::any::Any;
use std::borrow::Borrow;

#[derive(Serialize, Deserialize, Debug)]
struct PostRequestTest {
    x: u32,
    y: i32,
    message: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct ZPostRequestTest {
    x: u32,
    y: i32,
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PostResponseTest {
    pub matches: HashMap<u8,HashMap<u8,HashMap<u8,HashMap<u8,u64>>>>,
    pub abstracts: HashMap<u32, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchRequestBrief {
    pub clauses: Vec<String>,
    pub controls: HashMap<String,String>,
    pub count: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct SearchResponse {
    pub matches: HashMap<u8,HashMap<u8,HashMap<u8,HashMap<u8,u64>>>>,
    pub abstracts: HashMap<u32, String>,   // AVX extension to Quelle
    pub message: String,
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
fn test_request_from_rmp(bytes: Vec<u8>) -> PostRequestTest {
    let mut req: PostRequestTest = rmp_serde::from_read_ref(&bytes).unwrap();
    return req;
}
fn test_request_from_json(bytes: Vec<u8>) -> PostRequestTest {
    let asstr = std::str::from_utf8(&bytes).unwrap();
    let mut req: PostRequestTest = serde_json::from_str(asstr).unwrap();
    return req;
}
fn test_request_from_bytes(req: Vec<u8>, msgpack: bool) -> PostRequestTest {
    if msgpack {
        return test_request_from_rmp(req);
    } else {
        return test_request_from_json(req);
    }
}
fn test_result_into_rmp(resp: &PostResponseTest) -> Vec<u8> {
    return rmp_serde::to_vec(&resp).unwrap();
}
fn test_result_into_json(resp: &PostResponseTest) -> Vec<u8> {
    serde_json::to_vec(&resp).unwrap()
}
fn test_result_into_bytes(resp: &PostResponseTest, msgpack: bool) -> Vec<u8> {
    if msgpack {
        test_result_into_rmp(&resp)
    } else {
        test_result_into_json(&resp)
    }
}
fn search_request_from_rmp(bytes: Vec<u8>) -> SearchRequestBrief {
    let mut req: SearchRequestBrief = rmp_serde::from_read_ref(&bytes).unwrap();
    return req;
}
fn search_request_from_json(bytes: Vec<u8>) -> SearchRequestBrief {
    let asstr = std::str::from_utf8(&bytes).unwrap();
    let mut req: SearchRequestBrief = serde_json::from_str(asstr).unwrap();
    return req;
}
fn search_request_from_bytes(req: Vec<u8>, msgpack: bool) -> SearchRequestBrief {
    if msgpack {
        return search_request_from_rmp(req);
    } else {
        return search_request_from_json(req);
    }
}
fn search_result_into_rmp(resp: &SearchResponse) -> Vec<u8> {
    return rmp_serde::to_vec(&resp).unwrap();
}
fn search_result_into_json(resp: &SearchResponse) -> Vec<u8> {
    serde_json::to_vec(&resp).unwrap()
}
fn search_result_into_bytes(resp: &SearchResponse, msgpack: bool) -> Vec<u8> {
    if msgpack {
        search_result_into_rmp(&resp)
    } else {
        search_result_into_json(&resp)
    }
}
fn get_mimetype_from_header(hmap: &HeaderMap) -> String {
    if hmap.contains_key(CONTENT_TYPE) {
        let cloned = hmap.get(CONTENT_TYPE).clone().unwrap();
        let rmp = cloned.eq("application/msgpack");
        if rmp {
            return String::from("application/msgpack");
        }
    }
    String::from("application/json")
}
fn simulate_search_into_abstracts() -> HashMap<u32, String> {
    let mut abstracts: HashMap<u32, String> = HashMap::new();

    let ac: u32 = 44 * 0x10000 + 17 * 0x100 + 29;
    let rm: u32 = 45 * 0x10000 +  1 * 0x100 + 20;
    let co: u32 = 51 * 0x10000 +  2 * 0x100 + 51;

    abstracts.insert(ac, String::from("Forasmuch then as we are the offspring of God, we ought not to think that the Godhead is like unto gold, or silver, or stone, graven by art and man's device."));
    abstracts.insert(rm, String::from("For the invisible things of him from the creation of the world are clearly seen, being understood by the things that are made, even his eternal power and Godhead; so that they are without excuse:"));
    abstracts.insert(co, String::from("For in him dwelleth all the fulness of the Godhead bodily."));

    return abstracts;
}
fn simulate_search_into_matches() -> HashMap<u8,HashMap<u8,HashMap<u8,HashMap<u8,u64>>>> {
    let mut matches: HashMap<u8,HashMap<u8,HashMap<u8,HashMap<u8,u64>>>> = HashMap::new();

    let mut act17: HashMap<u8,HashMap<u8,HashMap<u8,u64>>> = HashMap::new();    // Acts 17
    let mut rom01: HashMap<u8,HashMap<u8,HashMap<u8,u64>>> = HashMap::new();    // Romans 1
    let mut col02: HashMap<u8,HashMap<u8,HashMap<u8,u64>>> = HashMap::new();    // Colossians 2

    let mut act1729: HashMap<u8,HashMap<u8,u64>> = HashMap::new();  // Acts 17:29
    let mut rom0120: HashMap<u8,HashMap<u8,u64>> = HashMap::new();  // Romans 1:20
    let mut col0209: HashMap<u8,HashMap<u8,u64>> = HashMap::new();  // Colossians 2:9

    let mut v1729: HashMap<u8,u64> = HashMap::new();    // Acts 17:29
    let mut v0120: HashMap<u8,u64> = HashMap::new();    // Romans 1:20
    let mut v0209: HashMap<u8,u64> = HashMap::new();    // Colossians 2:9

    v1729.insert(17, 1);
    v0120.insert(29, 1);
    v0209.insert(10, 1);

    act1729.insert(29, v1729);
    rom0120.insert(20, v0120);
    col0209.insert( 9, v0209);

    act17.insert(17, act1729);
    rom01.insert( 1, rom0120);
    col02.insert( 2, col0209);

    matches.insert(44, act17); // Acts
    matches.insert(45, rom01); // Romans
    matches.insert(51, col02); // Colossians

    return matches;
}
/// Extracts the elements of the POST request and prints them
fn post_test_handler(mut state: State) -> Pin<Box<HandlerFuture>> {
    print_request_elements(&state);
    let f = body::to_bytes(Body::take_from(&mut state)).then(|full_body| {
        match full_body {
            Ok(valid_body) => {
                let headers = HeaderMap::borrow_from(&state);
                let mime = get_mimetype_from_header(headers);

                let msgpack = "application/msgpack".eq_ignore_ascii_case(&mime);
                let mut req: PostRequestTest = test_request_from_bytes(valid_body.bytes().to_vec(), msgpack);
                println!("x = {}; y = {}; message = {}", req.x, req.y, req.message);

                let matches = simulate_search_into_matches();
                let abstracts = simulate_search_into_abstracts();

                let resp = PostResponseTest {
                    matches,
                    abstracts,
                };
                let bytes = test_result_into_bytes(&resp, msgpack);

                let res = create_response(&state, StatusCode::OK, mime.parse().unwrap(), bytes);

                future::ok((state, res))
            }
            Err(e) => future::err((state, e.into())),
        }
    });

    f.boxed()
}

/// Extracts the elements of the POST request and prints them
fn post_search_handler(mut state: State) -> Pin<Box<HandlerFuture>> {
    print_request_elements(&state);
    let f = body::to_bytes(Body::take_from(&mut state)).then(|full_body| {
        match full_body {
            Ok(valid_body) => {
                let headers = HeaderMap::borrow_from(&state);
                let mime = get_mimetype_from_header(headers);

                let msgpack = "application/msgpack".eq_ignore_ascii_case(&mime);
                let mut req: SearchRequestBrief = search_request_from_bytes(valid_body.bytes().to_vec(), msgpack);
                //println!("x = {}; y = {}; message = {}", req.x, req.y, req.message);

                let matches = simulate_search_into_matches();
                let abstracts = simulate_search_into_abstracts();

                let resp = SearchResponse {
                    matches,
                    abstracts,
                    message: String::from("Hello from Rust!"),
                };
                let bytes = search_result_into_bytes(&resp, msgpack);

                let res = create_response(&state, StatusCode::OK, mime.parse().unwrap(), bytes);

                future::ok((state, res))
            }
            Err(e) => future::err((state, e.into())),
        }
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
        route.associate("/test", |assoc| {
            assoc.post().to(post_test_handler);
        });
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