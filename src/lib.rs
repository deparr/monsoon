use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use std::time::Duration;
use std::{env, fs};

use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
struct Config {
    key: String,
    lat: String,
    lon: String,
    units: String,
}

const CONFIG_PATH: &str = "monsoon/config.json";
const LOG_PATH: &str = "monsoon-log";
const URI: &str = "/data/2.5/weather?lat=__LAT__&lon=__LON__&units=__UNITS__&appid=__KEY__";
const AUTHORITY: &str = "api.openweathermap.org";
const DEFAULT_RES: &str = r#"{
    "main": {
        "temp": 0.0
    },
    "weather": [
        {
            "description": "none"
        }
    ]
}"#;

const FETCH_INTERVAL: u64 = 600;

fn load_config() -> Result<Config, io::Error> {
    let config_path = match env::var_os("XDG_CONFIG_HOME") {
        Some(str) => Some(PathBuf::from(&str)),
        None => {
            if let Some(str) = env::var_os("HOME") {
                let mut path = PathBuf::from(&str);
                path.push(".config");
                Some(path)
            } else {
                None
            }
        }
    };

    let mut config_dir = config_path.expect("expected ");
    config_dir.push(CONFIG_PATH);
    let config: Config = serde_json::from_str(&fs::read_to_string(config_dir)?)?;

    Ok(config)
}

fn open_log() -> io::Result<fs::File> {
    let mut path = match env::var_os("TMPDIR") {
        Some(str) => PathBuf::from(&str),
        None => PathBuf::from("/tmp"),
    };
    path.push(LOG_PATH);

    return fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path);
}

pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = load_config()?;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let uri = Uri::builder()
        .scheme("https")
        .authority(AUTHORITY)
        // probably a better way to do this
        .path_and_query(
            URI.replace("__LAT__", &config.lat)
                .replace("__LON__", &config.lon)
                .replace("__UNITS__", &config.units)
                .replace("__KEY__", &config.key),
        )
        .build()
        .expect("failed to parse uri");

    let mut log = io::LineWriter::new(open_log()?);

    let forever = tokio::task::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(FETCH_INTERVAL));

        loop {
            interval.tick().await;
            let res = client.get(uri.clone()).await;

            let res = match res {
                Ok(res) => res,
                // api call failed, try again in 10 mins
                // todo do this better
                Err(_) => continue,
            };

            let res = String::from_utf8(
                hyper::body::to_bytes(res.into_body())
                    .await?
                    .into_iter()
                    .collect(),
            )
            .unwrap_or(String::from(DEFAULT_RES));

            let full_json: Value = serde_json::from_str(res.as_str())?;

            let main = full_json.get("main").unwrap();
            let weather = full_json.get("weather").unwrap();

            // This is terrible
            let temp = main.get("temp").unwrap().as_f64().unwrap().floor();
            let desc = weather
                .as_array()
                .unwrap()
                .get(0)
                .unwrap()
                .get("description")
                .unwrap()
                .as_str()
                .unwrap();

            writeln!(log, "{}° | {}", temp, desc)?;

            // probably unnecessary,
            // should make sure this isn't leaking via hashmap
            drop(full_json);
        }
    });

    forever.await?
}
