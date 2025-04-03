use reqwest::Client;
use serde_json::json;
use std::{env, process};
use tnixc_api::Song;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let api_key = match env::var("LAST_FM_API_KEY") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Could not find env var 'LAST_FM_API_KEY': {e}");
            process::exit(1)
        }
    };

    // Get origin from request headers
    let origin = req
        .headers()
        .get("origin")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    // Check if origin is allowed
    let allowed_origin =
        if origin.ends_with("tnixc.space") || origin.starts_with("http://localhost") {
            origin
        } else {
            // Default to deny if not from allowed origins
            ""
        };

    let song = current_song("Tnixc", &api_key).await;
    let out = match song {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error in getting current-song: {e}");
            process::exit(2)
        }
    };

    // Build response with CORS headers
    let mut response_builder = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json");

    // Only add CORS headers if origin is allowed
    if !allowed_origin.is_empty() {
        response_builder = response_builder
            .header("Access-Control-Allow-Origin", allowed_origin)
            .header("Access-Control-Allow-Methods", "GET, OPTIONS")
            .header("Access-Control-Allow-Headers", "Content-Type")
            .header("Access-Control-Max-Age", "86400"); // 24 hours
    }

    // Handle preflight OPTIONS request
    if req.method() == "OPTIONS" {
        return Ok(response_builder.body(Body::Empty)?);
    }

    Ok(response_builder.body(json!(out).to_string().into())?)
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
        now_playing: track["@attr"]["nowplaying"].as_str().unwrap_or_default() == "true",
    };

    Ok(song)
}
