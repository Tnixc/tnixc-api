use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Song {
    #[schema(example = "鷺巣詩郎")]
    pub name: String,
    #[schema(example = "Thanatos")]
    pub artist: String,
    #[schema(
        example = "https://lastfm.freetls.fastly.net/i/u/174s/c77d271d58305dcebadf5830fc5e0c7f.jpg"
    )]
    pub image_url: String,
    #[schema(example = true)]
    pub now_playing: bool,
}
