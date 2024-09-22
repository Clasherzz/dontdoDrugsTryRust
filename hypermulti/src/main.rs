/*extern crate hyper_multipart_rfc7578 as hyper_multipart;

use hyper::{
    Client, Request,
};
use hyper_multipart::client::{self, multipart};

#[tokio::main]
async fn main() {
  let client = Client::new();
  let mut form = multipart::Form::default();

  form.add_text("times", "3");
  form.add_text("text", "API");
  form.add_text("sep", "|");
 // form.add_text("test", "Hello World");


  let mut req_builder = Request::post("https://api.apidash.dev/io/form");
  let req = form.set_body_convert::<hyper::Body, multipart::Body>(req_builder)
      .unwrap();

  if let Ok(_) = client.request(req).await {
    println!("done...");
  } else {
    eprintln!("an error occurred");
  }
}*/



/*
extern crate hyper_multipart_rfc7578 as hyper_multipart;
extern crate hyper_tls;

use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;
use hyper_multipart::client::multipart;
use hyper::body::to_bytes;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create an HTTPS connector
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    // Create a multipart form and add text fields
    let mut form = multipart::Form::default();
    form.add_text("times", "3");
    form.add_text("text", "API");
    form.add_text("sep", "|");

    // Build the request
    //let req_builder = Request::post("https://api.apidash.dev/io/form");
    
    let req_builder = Request::post("http://localhost:3000/api/hello");
    let req = form.set_body_convert::<Body, multipart::Body>(req_builder)
        .map_err(|e| format!("Failed to set body: {}", e))?;

    // Send the request and await the response
    match client.request(req).await {
        Ok(res) => {
            // Print the response status
            println!("Response Status: {}", res.status());

            // Convert the response body to bytes and print it as a string
            let body_bytes = to_bytes(res).await?;
            let body_string = String::from_utf8(body_bytes.to_vec())?;
            println!("Response Body: {}", body_string);
        }
        Err(e) => {
            eprintln!("Failed to send request: {}", e);
        }
    }

    Ok(())
}

*/

extern crate hyper_multipart_rfc7578 as hyper_multipart;
extern crate hyper_tls;

use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;
use hyper_multipart::client::multipart;
use hyper::header::{HeaderValue, CONTENT_TYPE};
use hyper::body::to_bytes;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create an HTTPS connector and client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    // Define a custom boundary
    let custom_boundary = "--e1b04b70-78ac-11ef-b538-9531f40655f7\r\n";

    // Create a multipart form with a custom boundary
    let mut form = multipart::Form::with_boundary(custom_boundary.to_string());

    // Add text fields to the form
    form.add_text("times", "3");
    form.add_text("text", "API");
    form.add_text("sep", "|");

    // Build the request
    let mut req_builder = Request::post("https://api.apidash.dev/io/form");

    // Set the Content-Type header with the custom boundary
    req_builder = req_builder.header(CONTENT_TYPE, HeaderValue::from_str(&format!("multipart/form-data; boundary={}", custom_boundary))?);

    // Set the multipart body with the custom boundary
    let req = form.set_body_convert::<Body, multipart::Body>(req_builder).unwrap();

    // Send the request and await the response
    match client.request(req).await {
        Ok(res) => {
            // Print the response status
            println!("Response Status: {}", res.status());

            // Convert the response body to bytes and print it as a string
            let body_bytes = to_bytes(res).await?;
            let body_string = String::from_utf8(body_bytes.to_vec())?;
            println!("Response Body: {}", body_string);
        }
        Err(e) => {
            eprintln!("Failed to send request: {}", e);
        }
    }

    Ok(())
}
