use hyper::client::Request;
use hyper::method::Method;
use hyper::net::Streaming;

use multipart::Client::Multipart;

use std::io::Read;

fn main() {
    let url = "http://localhost:80".parse()
        .expect("Failed to parse URL");

    let request = Request::new(Method::Post, url)
        .expect("Failed to create request");

    let mut multipart = Multipart::from_request(request)
        .expect("Failed to create Multipart");

    write_body(&mut multipart)
        .expect("Failed to write multipart body");

    let mut response = multipart.send().expect("Failed to send multipart request");

    if !response.status.is_success() {
        let mut res = String::new();
        response.read_to_string(&mut res).expect("failed to read response");
        println!("response reported unsuccessful: {:?}\n {}", response, res);
    }

    // Optional: read out response
}

fn write_body(multi: &mut Multipart<Request<Streaming>>) -> hyper::Result<()> {
    let mut binary = "Hello world from binary!".as_bytes();

    multi.write_text("text", "Hello, world!")?;
    multi.write_file("file", "lorem_ipsum.txt")?;
    // &[u8] impl Read
    multi.write_stream("binary", &mut binary, None, None)
        .and(Ok(()))
}


/*
use hyper::{Body, Client, Method, Request};
use hyper::client::HttpConnector;
use hyper::Response;
use hyper::StatusCode;
use multipart::client::lazy::Multipart;
use tokio::runtime::Runtime;

use std::io::Read;
use std::fs::File;

fn main() {
    // Create a runtime to run async code synchronously
    let rt = Runtime::new().expect("Failed to create runtime");

    rt.block_on(async {
        let url = "http://localhost:80";
        let uri = url.parse().expect("Failed to parse URL");

        // Create a new Hyper client
        let client = Client::new();

        // Create a Multipart request
        let mut multipart = Multipart::new();
        write_body(&mut multipart).expect("Failed to write multipart body");

        // Build the request with multipart body
        let request = Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header("Content-Type", format!("multipart/form-data; boundary={}", multipart.boundary()))
            .body(Body::from(multipart.prepare().unwrap()))
            .expect("Failed to create request");

        // Send the request
        let response = client.request(request).await.expect("Failed to send multipart request");
        handle_response(response).await;
    });
}

fn write_body(multipart: &mut Multipart) -> std::io::Result<()> {
    multipart.add_text("text", "Hello, world!");
    multipart.add_file("file", "lorem_ipsum.txt")?;
    Ok(())
}

async fn handle_response(response: Response<Body>) {
    if response.status() != StatusCode::OK {
        let body_bytes = hyper::body::to_bytes(response.into_body()).await.expect("Failed to read response body");
        let body_str = String::from_utf8(body_bytes.to_vec()).expect("Response body is not valid UTF-8");
        println!("Response reported unsuccessful: {:?}", body_str);
    } else {
        println!("Response was successful.");
    }
}
*/