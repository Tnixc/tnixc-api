use rust_vercel::Song; // Import from our library
use serde_json::{json, Value};
use utoipa::OpenApi;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[derive(OpenApi)]
#[openapi(
    paths(
        current_song_handler,
        index
    ),
    components(
        schemas(Song)
    ),
    tags(
        (name = "Last.fm API", description = "Last.fm song tracking endpoints for tnixc")
    )
)]
struct ApiDoc;

#[utoipa::path(
    get,
    path = "/doc-json",
    tag = "Documentation",
    responses(
        (status = 200, description = "Swagger UI documentation in json", content_type = "text/json")
    )
)]
#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    // Check if requesting OpenAPI JSON
    let mut value = json!(ApiDoc::openapi());
    value["openapi"] = Value::String("3.1.0".to_string());
    return Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(value.to_string().into())?);
}

////////////////////////////////////////
// Function stubs for doc
////////////////////////////////////////

#[utoipa::path(
    get,
    path = "/current-song",
    tag = "Last.fm API",
    responses(
        (status = 200, description = "Successfully retrieved current song", body = Song),
        (status = 500, description = "Server error")
    )
)]
#[allow(dead_code)]
fn current_song_handler() {}

#[utoipa::path(
    get,
    path = "/index",
    tag = "Documentation",
    responses(
        (status = 200, description = "Swagger UI documentation", content_type = "text/html")
    )
)]
#[allow(dead_code)]
fn index() {}
