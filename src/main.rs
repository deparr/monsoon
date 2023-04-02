use std::fs::File;
use std::path::Path;

use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use serde_json::Value;


struct TempData {
    temp: f64,
    feels_like: f64,
}

struct WeatherData {
    description: String,
    main: String
}

#[derive(Deserialize, Debug)]
struct Config {
    key: String,
    lat: String,
    lon: String,
    units: String
}

const URI: &str = "/data/2.5/weather?lat=__LAT__&lon=__LON__&units=__UNITS__&appid=__KEY__";
const AUTHORITY: &str = "api.openweathermap.org";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _outstr = "__TEMP__°__TEMP_UNIT__ | __DESCRIPTION__";
    // TODO: get key and lat/lon from config file
    // (maybe all api params?)


    let path = Path::new("./config.json");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("unable to open file: {err}"),
    };

    let config: Config = serde_json::from_reader(file)?;
    println!("{:#?}", config);


    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let uri = Uri::builder()
        .scheme("https")
        .authority(AUTHORITY)
        .path_and_query(URI.replace("__LAT__", &config.lat)
                            .replace("__LON__", &config.lon)
                            .replace("__UNITS__", &config.units)
                            .replace("__KEY__", &config.key))
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

    let full_json: Value = serde_json::from_str(resp.as_str())?;

    //let basic = serde_json::de:: (json.get("main"));

    //let basic = json.get("main").unwrap_or(&serde_json::from_str(r#"{"temp": "none"}"#).unwrap());
    //let weather = json.get("weather");


    println!("\n\n{:#?}\n\n", full_json);
    Ok(())
}
