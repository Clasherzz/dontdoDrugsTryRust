/*use reqwest::multipart::{Form, Part};
use reqwest::Client;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new reqwest client
    let client = Client::builder()
        .danger_accept_invalid_certs(true) // Use this if you want to accept invalid certificates
        .build()?;

    // Create a multipart form
    let form = Form::new()
        .text("field1", "value1")
        .text("field2", "value2");
        

    // Send the form data
    let response = client
        .post("https://example.com/upload")
        .multipart(form)
        .send()
        .await?;

    // Print the response status and body
    println!("Status: {}", response.status());
    let body = response.text().await?;
    println!("Body: {}", body);

    Ok(())
}
*/


/*use hyper::{Body, Request};
use hyper::header::{CONTENT_TYPE, USER_AGENT};
use hyper::{Client as HyperClient};
use hyper_rustls::HttpsConnectorBuilder;
use reqwest::multipart::{Form, Part};
use reqwest::Client as ReqwestClient;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Using hyper to send a request with headers
    let https = HttpsConnectorBuilder::new()
        .https_only()
        .enable_native_tls()
        .build();
    let hyper_client = HyperClient::builder().build::<_, hyper::Body>(https);

    let hyper_request = Request::builder()
        .method("GET")
        .uri("https://example.com/headers")
        .header(USER_AGENT, "hyper-client")
        .header(CONTENT_TYPE, "application/json")
        .body(Body::empty())
        .expect("Failed to build request");

    let hyper_response = hyper_client.request(hyper_request).await?;
    println!("Hyper Status: {}", hyper_response.status());

    // 2. Using reqwest to send multipart form data with a file
    let reqwest_client = ReqwestClient::builder()
        .danger_accept_invalid_certs(true) // Optional: Accept invalid certificates
        .build()?;

    let form = Form::new()
        .text("field1", "value1")
        .text("field2", "value2")
        .part("file", Part::file("path/to/your/file.txt")?);

    let reqwest_response = reqwest_client
        .post("https://example.com/upload")
        .multipart(form)
        .send()
        .await?;

    println!("Reqwest Status: {}", reqwest_response.status());
    let body = reqwest_response.text().await?;
    println!("Reqwest Body: {}", body);

    Ok(())
}
*/






//properly working
/*use hyper::{Body, Client, Request, Uri};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use std::convert::TryInto;
//use hyper_tls::HttpsConnector;

use reqwest::multipart::{Form};
use reqwest::Client as ReqwestClient;

use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let url = "http://localhost:3000/api/endpoint".parse::<Uri>().unwrap();
    let req = Request::builder()
              .method("POST")
              .uri(url)        
              .header("Accept-Encoding", "sdsd");
        
    /*let form_data = vec![
        ("name", r"C:\Users\HP\Downloads\Visa General Rules and Setup for CodeSignal Assessments.pdf")
    ];
    let body = serde_urlencoded::to_string(&form_data)?;*/
    let req = req.body(Body::empty())?;
    let res = client.request(req).await?;
    let status = res.status();
    let body_bytes = hyper::body::to_bytes(res).await?;
    let body = String::from_utf8(body_bytes.to_vec())?;

    println!("Response Status: {}", status);
    println!("Response: {:?}", body);
    println!("1");


    let reqwest_client = ReqwestClient::builder()
        .danger_accept_invalid_certs(true) // Optional: Accept invalid certificates
        .build()?;
        println!("2");
    let form = Form::new()
        .text("field1", "value1")
        .text("field2", "value2");
        
        println!("3");
    let reqwest_response = reqwest_client
        .post("http://localhost:3000/api/endpoint")
        .multipart(form)
        .send()
        .await?;
        println!("4");
    println!("Reqwest Status: {}", reqwest_response.status());
    let body = reqwest_response.text().await?;
    println!("Reqwest Body: {}", body);





    Ok(())
}*/
/*
use hyper::{Body, Client, Request, Uri};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use std::convert::TryInto;
use reqwest::multipart::{Form,Part};

use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let url = "https://api.apidash.dev".parse::<Uri>().unwrap();
    let req = Request::builder()
              .method("POST")
              .uri(url)        
              .header("Access-Control-Max-Age", "fcffc")
        
              .header("Forwarded", "fcfffc")
        
              .header("t", "t")
        
              .body(Body::empty())?;

    
    let form = Form::new()
        .text("name", "cfcff")
        .part("file", Part::file(r"C:\Users\HP\Downloads\mod 2 notes.pdf")?);
    
      let res = client.request(req).await?;
    let status = res.status();
    let body_bytes = hyper::body::to_bytes(res).await?;
    let body = String::from_utf8(body_bytes.to_vec())?;

    println!("Response Status: {}", status);
    println!("Response: {:?}", body);

    let clientReqwest = reqwest::Client::new();
    let responseReqwest = clientReqwest
        .post("https://api.apidash.dev")
        .multipart(form)
        .send()
        .await?;

    println!("Reqwest Status: {}", responseReqwest.status());
    let bodyReqwest = responseReqwest.text().await?;
    println!("Reqwest Body: {}", bodyReqwest);
    
    Ok(())
}
*/




//trying to change the working code
/*
use hyper::{Body, Client, Request, Uri};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use std::convert::TryInto;
use multipart::client::Multipart;
use serde_json::json;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let url = "http://localhost:3000/api/hello".parse::<Uri>().unwrap();
    let req = Request::builder()
              .method("POST")
              .uri(url)        
              .header("Accept-Encoding", "dfd")
        
              .header("Accept", "sdfdsf")
                
              .body(Body::from(json!({
    "name":"kgovin",
    "place":"place"
}).to_string()))?;
    let mut multipart = Multipart::from_request(request)
                        .write_text("text", "Hello, world!")?
                        .write_file("file", "lorem_ipsum.txt")?;
    let res = client.request(req).await?;
    let status = res.status();
    let body_bytes = hyper::body::to_bytes(res).await?;
    let body = String::from_utf8(body_bytes.to_vec())?;

    println!("Response Status: {}", status);
    println!("Response: {:?}", body);

    Ok(())
}


*/
/*
use hyper::client::Request;
use hyper::method::Method;
use hyper::net::Streaming;

use multipart::client::Multipart;

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
*/

/*use hyper::Client;
use hyper::Request;
use hyper::{Body, Method};
use multipart_async::Client::multipart;
use multipart_async::Client::Part;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = "http://127.0.0.1:3000"; // Change this if necessary

    // Create a multipart form
    let form = multipart::Form::new()
        .part("text_field", Part::text("Hello World"))
        .part("file_field", Part::file("example.txt").await?); // Ensure example.txt exists

    // Create the request
    let request = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header("Content-Type", form.content_type())
        .body(Body::from(form.into_inner()?)?)?;

    // Send the request
    let response = client.request(request).await?;
    println!("Response: {:?}", response);

    Ok(())
}
*/
/*

use hyper::{Body, Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;
use multipart::client::lazy::Multipart;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an HTTPS connector and client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    // Parse the URL
    let url = "http://localhost:3000/api/hello".parse::<Uri>().unwrap();

    // Create a Multipart object and add multiple text and file fields
    let mut multipart = Multipart::new();

    // Adding multiple text fields
    multipart.add_text("text1", "Hello, world!"); // First text field
    multipart.add_text("text2", "Another text field"); // Second text field
    multipart.add_text("name", "kgovin"); // Example field for name
    multipart.add_text("place", "place"); // Example field for place

    // Adding multiple file fields
    multipart.add_file("file1", "path/to/first_file.txt")?; // First file field
    multipart.add_file("file2", "path/to/second_file.txt")?; // Second file field

    // Prepare the body from the Multipart
    let body = Body::from(multipart.prepare().unwrap());

    // Build the request with the multipart body and additional headers
    let req = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header(
            "Content-Type",
            format!("multipart/form-data; boundary={}", multipart.boundary()),
        )
        .header("Accept-Encoding", "dfd")
        .header("Accept", "sdfdsf")
        .body(body)?;

    // Send the request and await the response
    let res = client.request(req).await?;
    let status = res.status();

    // Read the response body as bytes and convert it to a string
    let body_bytes = hyper::body::to_bytes(res).await?;
    let body = String::from_utf8(body_bytes.to_vec())?;

    // Print the response status and body
    println!("Response Status: {}", status);
    println!("Response: {:?}", body);

    Ok(())
}
*/
/*

use hyper::{Body, Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;
use multipart::client::lazy::Multipart;
use tokio;
use std::io::Cursor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an HTTPS connector and client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    // Parse the URL
    let url = "http://localhost:3000/api/hello".parse::<Uri>().unwrap();

    // Create a Multipart object and add multiple text and file fields
    let mut multipart = Multipart::new();

    // Adding multiple text fields
    multipart.add_text("text1", "Hello, world!"); // First text field
    multipart.add_text("text2", "Another text field"); // Second text field
    multipart.add_text("name", "kgovin"); // Example field for name
    multipart.add_text("place", "place"); // Example field for place

    // Adding multiple file fields using Cursor (no `?` needed, handle errors manually)
    if let Err(e) = multipart.add_file("file1", "path/to/first_file.txt") {
        eprintln!("Error adding file1: {}", e);
    }
    if let Err(e) = multipart.add_file("file2", "path/to/second_file.txt") {
        eprintln!("Error adding file2: {}", e);
    }

    // Prepare the multipart body and convert it to a Body
    let mut prepared_body = Vec::new();
    multipart.prepare().unwrap().read_to_end(&mut prepared_body)?;

    let body = Body::from(prepared_body);

    // Build the request with the multipart body and additional headers
    let req = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header(
            "Content-Type",
            format!("multipart/form-data; boundary={}", multipart.boundary()),
        )
        .header("Accept-Encoding", "dfd")
        .header("Accept", "sdfdsf")
        .body(body)?;

    // Send the request and await the response
    let res = client.request(req).await?;
    let status = res.status();

    // Read the response body as bytes and convert it to a string
    let body_bytes = hyper::body::to_bytes(res).await?;
    let body = String::from_utf8(body_bytes.to_vec())?;

    // Print the response status and body
    println!("Response Status: {}", status);
    println!("Response: {:?}", body);

    Ok(())
}
*/

/*use hyper::{Body, Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;
use multipart::client::lazy::Multipart;
use multipart::client::lazy::Multipart;
use tokio;
use std::io::Read;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an HTTPS connector and client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    // Parse the URL
    let url = "http://localhost:3000/api/hello".parse::<Uri>()?;

    // Create a Multipart object and add multiple text and file fields
    let mut multipart = Multipart::new();

    // Adding multiple text fields
    multipart.add_text("text", "API"); // First text field
    multipart.add_text("sep", "|"); // Second text field
    multipart.add_text("times", "3"); // Example field for name
    //multipart.add_text("place", "place"); // Example field for place

    // Adding multiple file fields directly without using `if let`
    //multipart.add_file("file1", r"C:\Users\HP\Downloads\GCF Prepartion Deck - Visa -Codesignal.pdf");
    

    // Prepare the multipart body
    let mut prepared_body = Vec::new();
    let mut prepared_fields = multipart.prepare().unwrap();
    prepared_fields.read_to_end(&mut prepared_body)?;

    let body = Body::from(prepared_body);

    // Manually specify the boundary used in Multipart creation
    let boundary = multipart.boundary();


    // Build the request with the multipart body and additional headers
    let req = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header(
            "Content-Type",
            format!("multipart/form-data; boundary={}", boundary),
        )
        .body(body)?;

    // Send the request and await the response
    let res = client.request(req).await?;
    let status = res.status();

    // Read the response body as bytes and convert it to a string
    let body_bytes = hyper::body::to_bytes(res).await?;
    let body = String::from_utf8(body_bytes.to_vec())?;

    // Print the response status and body
    println!("Response Status: {}", status);
    println!("Response: {:?}", body);

    Ok(())
}
*/


use hyper::{Body, Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;
use multipart::client::lazy::Multipart;

use tokio;
use std::io::Read;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an HTTPS connector and client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    // Parse the URL
    let url = "http://localhost:3000/api/hello".parse::<Uri>()?;

    // Create a Multipart object and add multiple text and file fields
    let mut multipart = Multipart::new();

    // Adding multiple text fields
    multipart.add_text("text", "API"); // First text field
    multipart.add_text("sep", "|"); // Second text field
    multipart.add_text("times", "3"); // Example field for name
    //multipart.add_text("place", "place"); // Example field for place

    // Adding multiple file fields directly without using `if let`
    //multipart.add_file("file1", r"C:\Users\HP\Downloads\GCF Prepartion Deck - Visa -Codesignal.pdf");
    

    // Prepare the multipart body
    let mut prepared_body = Vec::new();
    let mut prepared_fields = multipart.prepare().unwrap();
    prepared_fields.read_to_end(&mut prepared_body)?;

    let body = Body::from(prepared_body);

    // Manually specify the boundary used in Multipart creation
    let boundary = "----Boundary";


    // Build the request with the multipart body and additional headers
    let req = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header(
            "Content-Type",
            format!("multipart/form-data; boundary={}", boundary),
        )
        .body(body)?;

    // Send the request and await the response
    let res = client.request(req).await?;
    let status = res.status();

    // Read the response body as bytes and convert it to a string
    let body_bytes = hyper::body::to_bytes(res).await?;
    let body = String::from_utf8(body_bytes.to_vec())?;

    // Print the response status and body
    println!("Response Status: {}", status);
    println!("Response: {:?}", body);

    Ok(())
}