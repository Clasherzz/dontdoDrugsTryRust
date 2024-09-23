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
    eprintln!("an error occurred {}");
  }
}

*/
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
    let req_builder = Request::post("https://api.apidash.dev/io/form");
    
  //  let req_builder = Request::post("http://localhost:3000/api/hello");
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
/*
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
}*/

/*
use hyper::{Body, Client, Method, Request};
use hyper::header::{HeaderValue, CONTENT_TYPE};
use hyper_multipart_rfc7578::client::multipart;
use tokio::io::{self, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create a new Hyper client
    let client = Client::new();

    // Construct the multipart form
    let mut form = multipart::Form::default();
    form.add_text("times", "3");
    form.add_text("text", "API");
    form.add_text("sep", "|");

    // Prepare the body for the request
    let mut body = Body::wrap_stream(form.set_body_convert());

    // Create the request
    let req = Request::builder()
        .method(Method::POST)
        .uri("https://api.apidash.dev/io/form")
        .header(CONTENT_TYPE, HeaderValue::from_static("multipart/form-data"))
        .body(body)?;

    // Send the request
    let res = client.request(req).await?;

    // Read and print the response
    /*let mut body = res.into_body();
    let mut buf = Vec::new();
    while let Some(chunk) = body.data().await {
        let chunk = chunk?;
        buf.extend_from_slice(&chunk);
    }
    io::stdout().write_all(&buf).await?;*/
    let status = res.status();
    let body_bytes = hyper::body::to_bytes(res).await?;
    let body = String::from_utf8(body_bytes.to_vec())?;

    println!("Response Status: {}", status);
    println!("Response: {:?}", body);


    Ok(())
}

*/

/*
extern crate hyper_multipart_rfc7578 as hyper_multipart;
extern crate hyper_tls;

use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;
use hyper_multipart::client::multipart;
use hyper::header::CONTENT_TYPE;
use hyper::body::to_bytes;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create an HTTPS connector and client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    // Create a multipart form (without custom boundary)
    let mut form = multipart::Form::default();

    // Add text fields to the form
    form.add_text("times", "3");
    form.add_text("text", "API");
    form.add_text("sep", "|");

    // Build the request
    let mut req_builder = Request::post("https://api.apidash.dev/io/form");

    // Set the Content-Type header to multipart/form-data
    req_builder = req_builder.header(CONTENT_TYPE, "multipart/form-data");

    // Set the multipart body
    let req = form.set_body_convert::<Body, multipart::Body>(req_builder).unwrap();

    // Send the request and await the response
    match client.request(req).await {
        Ok(res) => {
            // Print the response status
            println!("Response Status: {}", res.status());

            // Convert the response body to a string and print it
            let body_bytes = to_bytes(res.into_body()).await?;
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
    // Create an HTTPS connector and client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    // Create a multipart form
    let mut form = multipart::Form::default();

    // Add text fields to the form
    form.add_text("times", "3");
    form.add_text("text", "API");
    form.add_text("sep", "|");

    // Build the request
    let req_builder = Request::post("https://api.apidash.dev/io/form");

    // Set the multipart body without manually setting headers
    let req = form.set_body_convert::<Body, multipart::Body>(req_builder).unwrap();

    // Send the request and await the response
    match client.request(req).await {
        Ok(res) => {
            // Print the response status
            println!("Response Status: {}", res.status());

            // Convert the response body to a string and print it
            let body_bytes = to_bytes(res.into_body()).await?;
            let body_string = String::from_utf8(body_bytes.to_vec())?;
            println!("Response Body: {}", body_string);
        }
        Err(e) => {
            eprintln!("Failed to send request: {}", e);
        }
    }

    Ok(())
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
    // Create an HTTPS connector and client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    // Create a multipart form
    let mut form = multipart::Form::default();

    // Add text fields to the form
   /* form.add_text("times", "3");
    form.add_text("text", "API");
    form.add_text("sep", "|");*/
    form.add_text("token", "xyz");
    form.add_file("imfile","C:/Users/HP/Downloads/WhatsApp_Image_2024-08-28_at_16.21.41_8fe559f6-removebg-preview.png");

    // Build the request
    let req_builder = Request::post("https://api.apidash.dev/io/img")
                            .header("User-Agent","Test Agent")
                            .body(Body::empty())?;

    // Set the multipart body without manually setting headers
    let req = form.set_body_convert::<Body, multipart::Body>(req_builder).unwrap();

    // Send the request and await the response
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
use hyper::{Body, Client, Request, Uri};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use std::convert::TryInto;

use serde_json::json;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let url = "https://api.apidash.dev/case/lower".parse::<Uri>().unwrap();
    let reqBuilder = Request::builder()
              .method("POST")
              .uri(url)        
              .header("User-Agent", "Test Agent")
                
              .body(Body::from(json!({
"text": "I LOVE Flutter"
}
).to_string()))?;
    let res = client.request(reqBuilder).await?;
    let status = res.status();
    let body_bytes = hyper::body::to_bytes(res).await?;
    let body = String::from_utf8(body_bytes.to_vec())?;

    println!("Response Status: {}", status);
    println!("Response: {:?}", body);

    Ok(())
}

*/

/*
use hyper::{Body, Client, Request, Uri};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use std::convert::TryInto;


use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let url = "https://api.apidash.dev/io/form".parse::<Uri>().unwrap();
    let reqBuilder = Request::builder()
              .method("POST")
              .uri(url)
              .body(Body::empty())?;

    
    let mut form = multipart::Form::default();
    form.add_text("text", "API");
    form.add_text("sep", "|");
    form.add_text("times", "3");

    let req = form.set_body_convert::<Body, multipart::Body>(reqBuilder).unwrap();
    
      let res = client.request(req).await?;
    let status = res.status();
    let body_bytes = hyper::body::to_bytes(res).await?;
    let body = String::from_utf8(body_bytes.to_vec())?;

    println!("Response Status: {}", status);
    println!("Response: {:?}", body);
    

    Ok(())
}

*/

extern crate hyper_multipart_rfc7578 as hyper_multipart;
use hyper::{Body, Client, Request, Uri};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use std::convert::TryInto;
use hyper_multipart::client::multipart;

use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let url = "https://api.apidash.dev/io/img".parse::<Uri>().unwrap();
    let reqBuilder = Request::builder()
              .method("POST")
              .uri(url)        
              .header("User-Agent", "Test Agent");
            
    let mut form = multipart::Form::default();
    form.add_text("token", "ABC");
    form.add_file("imfile", "C:/Users/HP/Downloads/WhatsApp_Image_2024-08-28_at_16.21.41_8fe559f6-removebg-preview.png").unwrap();

    let req = form.set_body_convert::<Body, multipart::Body>(reqBuilder).unwrap();
    
      let res = client.request(req).await?;
    let status = res.status();
    let body_bytes = hyper::body::to_bytes(res).await?;
    let body = String::from_utf8(body_bytes.to_vec())?;

    println!("Response Status: {}", status);
    println!("Response: {:?}", body);
    

    Ok(())
}

