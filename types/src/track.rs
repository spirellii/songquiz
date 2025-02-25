use rspotify::model::FullTrack;
use serde::{Deserialize, Serialize};

/// The representation of a track for the client
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Track {
    pub name: String,
    pub artists: Vec<String>,
    pub image: String,
}

impl From<FullTrack> for Track {
    fn from(value: FullTrack) -> Self {
        Self {
            name: value.name,
            artists: value
                .artists
                .into_iter()
                .map(|artist| artist.name)
                .collect(),
            image: value
                .album
                .images
                .into_iter()
                .next()
                .map_or(String::new(), |image| image.url),
        }
    }
}
