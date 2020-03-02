use std::default::Default;

use reqwest::blocking::get;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Photo {
    #[serde(rename = "albumId")]
    pub album_id: usize,
    pub id: usize,
    pub title: String,
    pub url: String,
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: String,
}

pub struct PhotoCollection {}

impl PhotoCollection {
    pub fn get_photos_by_album(album_id: &str) -> Result<Vec<Photo>, reqwest::Error> {
        let photos_url = &format!(
            "https://jsonplaceholder.typicode.com/photos?albumId={}",
            album_id
        );

        get(photos_url)?.json::<Vec<Photo>>()
    }
}
