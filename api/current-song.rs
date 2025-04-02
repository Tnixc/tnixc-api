use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{env, process};
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[derive(Debug, Serialize, Deserialize)]
pub struct Song {
    pub name: String,
    pub artist: String,
    pub image_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let api_key = match env::var("LAST_FM_API_KEY") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Could not find env var 'LAST_FM_API_KEY': {e}");
            process::exit(1)
        }
    };

    let song = current_song("Tnixc", &api_key).await;
    let out = match song {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error in getting current-song: {e}");
            process::exit(2)
        }
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(json!(out).to_string().into())?)
}

pub async fn current_song(
    username: &str,
    api_key: &str,
) -> Result<Song, Box<dyn std::error::Error>> {
    let client = Client::new();

    let response = client
        .get("https://ws.audioscrobbler.com/2.0/")
        .query(&[
            ("method", "user.getrecenttracks"),
            ("user", username),
            ("api_key", api_key),
            ("format", "json"),
            ("limit", "1"),
        ])
        .send()
        .await?;

    let data: serde_json::Value = response.json().await?;

    let track = &data["recenttracks"]["track"][0];

    let song = Song {
        name: track["name"].as_str().unwrap_or_default().to_string(),
        artist: track["artist"]["#text"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        image_url: track["image"][2]["#text"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
    };

    Ok(song)
}
