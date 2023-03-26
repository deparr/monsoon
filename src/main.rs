use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use serde_json::Value;

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
    let client = Client::builder().build::<_, hyper::Body>(https);
    let uri = Uri::builder()
        .scheme("https")
        .authority("api.openweathermap.org")
        .path_and_query(uri.replace("__KEY__", key.trim_end()))
        .build()
        .expect("failed to parse");

    /*
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }*/

    let resp = client.get(uri).await?;
    println!("Response: {}", resp.status());
    let resp = String::from_utf8(
        hyper::body::to_bytes(resp.into_body())
            .await?
            .into_iter()
            .collect(),
    )
    .unwrap();

    //println!("Body: {}", resp);

    let json: Value = match serde_json::from_str(resp.as_str()) {
        Ok(v) => v,
        Err(err) => panic!("unable to deserialize: {err}"),
    };

    let json = match json.as_object() {
        Some(obj) => obj,
        None => panic!("unable to read map of res"),
    };

    let weather = json.get("weather");
    let main = json.get("main");

    println!("\n\n{:#?}\n\n{:#?}", main, weather);
    Ok(())
}
