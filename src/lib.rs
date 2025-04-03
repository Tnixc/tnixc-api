use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "artist": "Grant",
    "image_url": "https://lastfm.freetls.fastly.net/i/u/174s/bf9ad4665ed88573c7ff2d4081877c69.jpg",
    "name": "Dead Man Walking",
    "now_playing": false
}))]
pub struct Song {
    #[schema(example = "Grant")]
    pub name: String,
    #[schema(example = "Dead Man Walking")]
    pub artist: String,
    #[schema(
        example = "https://lastfm.freetls.fastly.net/i/u/174s/bf9ad4665ed88573c7ff2d4081877c69.jpg"
    )]
    pub image_url: String,
    #[schema(example = false)]
    pub now_playing: bool,
}
