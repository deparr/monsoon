//use std::fs::File;
//use std::future::Future;
//use std::io::prelude::*;
//use std::path::Path;
//use std::env;

use hyper::{ Client, Uri };
use hyper_tls::HttpsConnector;
use hyper::body::HttpBody as _;
use tokio::io::{stdout, AsyncWriteExt as _};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {


    // TODO: get key and lat/lon from config file
    // (maybe all api params?)

    //println!("{}", env::current_dir().unwrap().to_str().unwrap_or("none"));

   /* 
    let path = Path::new("./key");
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("unable to open file: {err}"),
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => println!("{s}"),
        Err(err) => panic!("unable to read file: {err}"),
    }*/

    let https = HttpsConnector::new();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);
    let uri = Uri::builder()
        .scheme("https")
        .authority("api.openweathermap.org")
        .build()
        .expect("failed to parse");
    let mut resp = client.get(uri).await?;
    println!("Response: {}", resp.status());


    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }



   Ok(()) 
}
