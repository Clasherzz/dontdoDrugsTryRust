use hyper::{Body, Client, Request, Uri};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use std::convert::TryInto;

use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let url = "https://api.apidash.dev".parse::<Uri>().unwrap();
    let req = Request::builder()
              .method("GET")
              .uri(url)        
              .header("Accept-Encoding", "sdsd");
        
    let form_data = vec![
        ("name", r"C:\Users\HP\Downloads\Visa General Rules and Setup for CodeSignal Assessments.pdf")
    ];
    let body = serde_urlencoded::to_string(&form_data)?;
    let req = req.body(Body::from(body))?;
    let res = client.request(req).await?;
    let status = res.status();
    let body_bytes = hyper::body::to_bytes(res).await?;
    let body = String::from_utf8(body_bytes.to_vec())?;

    println!("Response Status: {}", status);
    println!("Response: {:?}", body);

    Ok(())
}


/*use hyper::{Body, Client, Request, Uri};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use reqwest::multipart;
use std::error::Error;
use tokio;
use hyper::header::{CONTENT_TYPE, AUTHORIZATION};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // ----------- Using hyper for Header and JSON Request ----------- //
    let https = HttpsConnector::new();
    let hyper_client = Client::builder().build::<_, Body>(https);

    // Define a target URI for the hyper request (your JSON endpoint)
    let json_url = "http://localhost:3000/upload".parse::<Uri>().unwrap();

    // Create JSON payload
    let json_payload = r#"{
        "name": "John Doe",
        "age": 30
    }"#;

    // Create a POST request with JSON body using hyper
    let json_req = Request::builder()
        .method("POST")
        .uri(json_url)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, "Bearer my-secret-token")
        .body(Body::from(json_payload))?;

    // Send the request with hyper
    let hyper_res = hyper_client.request(json_req).await?;
    let hyper_status = hyper_res.status();
    let hyper_body_bytes = hyper::body::to_bytes(hyper_res.into_body()).await?;
    let hyper_body = String::from_utf8(hyper_body_bytes.to_vec())?;

    // Output the response from hyper
    println!("Hyper Response Status: {}", hyper_status);
    println!("Hyper Response Body: {:?}", hyper_body);

    // ----------- Using reqwest for Multipart Form Data Request ----------- //
    // Create a multipart form with a single text field using reqwest
    let form = multipart::Form::new()
        .text("field_name", "field_value");

    // Create the reqwest client
    let reqwest_client = reqwest::Client::new();

    // Send a POST request with the multipart form using reqwest
    let reqwest_response = reqwest_client
        .post("http://localhost:3000/upload")
        .multipart(form)
        .send()
        .await?;

    // Handle the reqwest response
    let reqwest_status = reqwest_response.status();
    let reqwest_body = reqwest_response.text().await?;

    // Output the response from reqwest
    println!("Reqwest Multipart Response Status: {}", reqwest_status);
    println!("Reqwest Multipart Response Body: {:?}", reqwest_body);

    Ok(())
}*/


/*
[package]
name = "hyper"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
# hyper = { version = "0.14", features = ["client", "http1"] }
hyper-tls = "0.5"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_urlencoded = "0.7" 
# Latest version of hyper
hyper = { version = "0.14", features = ["full", "tls"] }

# Latest version of reqwest with TLS and multipart support
reqwest = { version = "0.12.7", features = ["json", "multipart", "blocking", "tls"] }
*/


/*use hyper::{Body, Client, Request, Uri};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use multipart::client::lazy::Multipart;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::error::Error;
use hyper::header::{CONTENT_TYPE};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // HTTPS connector for hyper
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    // Define the target URI (endpoint) for the POST request
    let uri: Uri = "http://localhost:3000/upload".parse()?;

    // Create a new multipart form
    let mut form = Multipart::new();

    // Add a text field to the form
    form.add_text("username", "John Doe");

    // Read the file that you want to send
    let mut file = File::open("path/to/your/file.pdf").await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;

    // Add the file to the multipart form
    form.add_stream("file", &buffer[..], Some("file.pdf"), Some("application/pdf"));

    // Convert the form into a body that hyper can use
    let body = form.prepare().unwrap().into_body();

    // Create a POST request with multipart/form-data content type
    let req = Request::builder()
        .method("POST")
        .uri(uri)
        .header(CONTENT_TYPE, "multipart/form-data")
        .body(Body::from(body))?;

    // Send the request via hyper
    let res = client.request(req).await?;
    let status = res.status();
    let body_bytes = hyper::body::to_bytes(res.into_body()).await?;
    let body = String::from_utf8(body_bytes.to_vec())?;

    // Output the response from the server
    println!("Response Status: {}", status);
    println!("Response Body: {}", body);

    Ok(())
}*/


/*use hyper::{Body, Client, Request, Uri};
use hyper_tls::HttpsConnector;
use reqwest::multipart;
use std::error::Error;
use tokio;
use hyper::header::{CONTENT_TYPE, AUTHORIZATION};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // ----------- Sending JSON using hyper ----------- //
    let https = HttpsConnector::new();
    let hyper_client = Client::builder().build::<_, Body>(https);

    // Define a target URI for JSON (your Express endpoint)
    let json_url: Uri = "http://localhost:3000/upload".parse()?;

    // Create a JSON payload
    let json_payload = r#"{
        "name": "John Doe",
        "age": 30
    }"#;

    // Create the POST request with JSON and authorization header
    let json_req = Request::builder()
        .method("POST")
        .uri(json_url.clone())
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, "Bearer my-secret-token")
        .body(Body::from(json_payload))?;

    // Send the request using hyper
    let hyper_res = hyper_client.request(json_req).await?;
    let hyper_status = hyper_res.status();
    let hyper_body_bytes = hyper::body::to_bytes(hyper_res.into_body()).await?;
    let hyper_body = String::from_utf8(hyper_body_bytes.to_vec())?;

    // Output the response from hyper
    println!("Hyper Response Status: {}", hyper_status);
    println!("Hyper Response Body: {:?}", hyper_body);

    // ----------- Sending Form Data using reqwest ----------- //
    // Create a multipart form with text fields
    let form = multipart::Form::new()
        .text("username", "John Doe")
        .text("email", "john.doe@example.com");

    // Create the reqwest client
    let reqwest_client = reqwest::Client::new();

    // Send a POST request with the multipart form using reqwest
    let reqwest_response = reqwest_client
        .post("http://localhost:3000/upload")
        .multipart(form)
        .send()
        .await?;

    // Handle the reqwest response
    let reqwest_status = reqwest_response.status();
    let reqwest_body = reqwest_response.text().await?;

    // Output the response from reqwest
    println!("Reqwest Multipart Response Status: {}", reqwest_status);
    println!("Reqwest Multipart Response Body: {:?}", reqwest_body);

    Ok(())
}
*/

/*
use hyper::{Body, Client, Request};
use hyper::header::{CONTENT_TYPE, HeaderValue};
use multipart::server::Multipart;
use serde_json::json;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // Create the form data
    let mut form = Form::new();
    form.add_text("key1", "value1");
    form.add_text("key2", "value2");

    let form_data = form.into_parts();

    // Build the request
    let req = Request::builder()
        .method("POST")
        .uri("http://localhost:3000/rust3")
        .header(CONTENT_TYPE, "multipart/form-data; boundary=boundary")
        .header("Custom-Header", "HeaderValue")
        .body(Body::from(form_data)) // Here you need to use the form data properly
        .expect("Failed to build request");

    let response = client.request(req).await?;
    println!("Response: {}", response.status());

    Ok(())
}
*/


/*
use hyper::{Body, Client, Request};
use hyper::header::{CONTENT_TYPE};
use std::io::Write;
use std::io::Cursor;
use bytes::Bytes;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let boundary = "boundary".to_string();

    // Create a multipart body manually
    let mut body = Cursor::new(Vec::new());
    write!(body, "--{}\r\n", boundary)?;
    write!(body, "Content-Disposition: form-data; name=\"key1\"\r\n\r\n")?;
    write!(body, "value1\r\n")?;
    write!(body, "--{}\r\n", boundary)?;
    write!(body, "Content-Disposition: form-data; name=\"key2\"\r\n\r\n")?;
    write!(body, "value2\r\n")?;
    write!(body, "--{}--\r\n", boundary)?;

    let body = Body::from(body.into_inner());

    // Build the request
    let req = Request::builder()
        .method("POST")
        .uri("http://localhost:3000/api/post3")
        .header(CONTENT_TYPE, format!("multipart/form-data; boundary={}", boundary))
        .header("Custom-Header", "HeaderValue")
        .body(body)
        .expect("Failed to build request");

    let response = client.request(req).await?;
    println!("Response: {}", response.status());

    Ok(())
}
*/