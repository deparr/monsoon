use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use hyper::{ Client, Uri };
use hyper_tls::HttpsConnector;
use hyper::body::HttpBody as _;
use tokio::io::{stdout, AsyncWriteExt as _};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let uri = "/data/2.5/weather?lat=40.24&lon=-111.65&units=imperial&appid=__KEY__";
    // TODO: get key and lat/lon from config file
    // (maybe all api params?)
    
    let path = Path::new("./key");
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("unable to open file: {err}"),
    };

    let mut key = String::new();
    match file.read_to_string(&mut key) {
        Ok(_) => (),
        Err(err) => panic!("unable to read key file: {err}"),
    }

    let https = HttpsConnector::new();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);
    let uri = Uri::builder()
        .scheme("https")
        .authority("api.openweathermap.org")
        .path_and_query(uri.replace("__KEY__", key.trim_end()))
        .build()
        .expect("failed to parse");
    let mut resp = client.get(uri).await?;
    println!("Response: {}", resp.status());

    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }

   Ok(()) 
}
