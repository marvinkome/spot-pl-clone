use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct CollectionResponse<T> {
    pub href: String,
    pub items: Vec<T>,
    pub limit: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub offset: u32,
    pub total: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlbumCover {
    pub height: u32,
    pub url: String,
    pub width: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Album {
    pub album_type: String,
    pub available_markets: Vec<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<AlbumCover>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub total_tracks: u32,
    pub r#type: String,
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Artist {
    pub href: String,
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub uri: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PlaylistInfo {
    pub collaborative: bool,
    pub description: Option<String>,
    pub href: String,
    pub id: String,
    pub name: String,
    pub public: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PlaylistTrack {
    pub added_at: Option<String>,
    pub is_local: bool,
    pub track: Track,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Track {
    pub album: Option<Album>,
    pub artists: Vec<Artist>,
    pub available_markets: Vec<String>,
    pub disc_number: u32,
    pub duration_ms: u32,
    pub explicit: bool,
    pub href: String,
    pub id: String,
    pub name: String,
    pub popularity: Option<u32>,
    pub preview_url: Option<String>,
    pub track_number: u32,
    pub r#type: String,
    pub uri: String,
}

pub type PlaylistTracksResponse = CollectionResponse<PlaylistTrack>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaylistAddItemResponse {
    snapshot_id: String,
}
