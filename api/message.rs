use serenity::all::Http;
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::model::id::ChannelId;
use std::collections::HashMap;
use std::env;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

const CHANNEL_ID: u64 = 1364716034967470110;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let token = match env::var("DISCORD_TOKEN") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Could not find env var 'DISCORD_TOKEN': {e}");
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Server configuration error".into())?);
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

    // Build response with CORS headers
    let mut response_builder = Response::builder().header("Content-Type", "application/json");

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
        return Ok(response_builder.status(StatusCode::OK).body(Body::Empty)?);
    }

    // Parse query parameters
    let query_params = req
        .uri()
        .query()
        .map(|q| {
            url::form_urlencoded::parse(q.as_bytes())
                .into_owned()
                .collect::<HashMap<String, String>>()
        })
        .unwrap_or_default();

    // Validate required parameters
    if !query_params.contains_key("content") {
        return Ok(response_builder
            .status(StatusCode::BAD_REQUEST)
            .body("Missing 'content' parameter".to_string().into())?);
    }

    // Send message to Discord
    let res = discord_say(
        query_params.get("content"),
        query_params.get("contact"),
        &token,
    )
    .await;

    match res {
        Ok(_) => Ok(response_builder
            .status(StatusCode::OK)
            .body(r#"{"status":"success","message":"Message sent to Discord"}"#.into())?),
        Err(e) => {
            eprintln!("Error in sending discord message: {e}");
            Ok(response_builder
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(
                    format!(r#"{{"status":"error","message":"Failed to send message: {e}"}}"#)
                        .into(),
                )?)
        }
    }
}

pub async fn discord_say(
    content: Option<&String>,
    contact: Option<&String>,
    token: &str,
) -> Result<serenity::all::Message, serenity::Error> {
    let channel_id = ChannelId::from(CHANNEL_ID);
    let now = std::time::SystemTime::now();
    // Convert SystemTime to a Unix timestamp (seconds since UNIX epoch)
    let timestamp = now
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let formatted_time = format!("<t:{}:f>", timestamp);
    let embed = CreateEmbed::new()
        .title(content.unwrap_or(&"None".to_string()))
        .description(format!(
            "{} {}",
            contact.unwrap_or(&"None".to_string()),
            formatted_time
        ))
        .color((36, 99, 235)); // rgb(36, 99, 235)
    let builder = CreateMessage::new().embed(embed);
    let http = Http::new(token);

    channel_id.send_message(&http, builder).await
}
