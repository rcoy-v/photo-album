use std::default::Default;

use reqwest::blocking::get;
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Photo {
    #[serde(rename = "albumId")]
    pub album_id: usize,
    pub id: usize,
    pub title: String,
    pub url: String,
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: String,
}

pub struct PhotoCollection {
    url: String,
}

impl PhotoCollection {
    pub fn new() -> PhotoCollection {
        PhotoCollection {
            url: "https://jsonplaceholder.typicode.com".to_string(),
        }
    }

    pub fn get_photos_by_album(&self, album_id: usize) -> Result<Vec<Photo>, reqwest::Error> {
        let photos_url = Url::parse_with_params(
            format!("{}/photos", self.url).as_str(),
            &[("albumId", album_id.to_string())],
        ).unwrap();

        get(photos_url)?.json::<Vec<Photo>>()
    }
}

#[cfg(test)]
mod test {
    use mockito::mock;

    use super::*;

    #[test]
    fn should_correctly_return_matching_photos_by_album() {
        let album_id = 1;
        let expected_photos = &vec![
            Photo {
                album_id: album_id,
                id: 1,
                title: "".to_string(),
                url: "".to_string(),
                thumbnail_url: "".to_string(),
            },
            Photo {
                album_id: album_id,
                id: 2,
                title: "".to_string(),
                url: "".to_string(),
                thumbnail_url: "".to_string(),
            },
        ];
        let _mock = mock("GET", "/photos")
            .match_query(format!("albumId={}", album_id).as_str())
            .with_status(200)
            .with_body(serde_json::to_string(expected_photos).unwrap())
            .create();
        let photo_collection = PhotoCollection {
            url: mockito::server_url(),
        };

        let photos = photo_collection.get_photos_by_album(album_id).unwrap();

        assert_eq!(photos.len(), expected_photos.len());
        for n in 0..photos.len() {
            assert_eq!(photos[n], expected_photos[n]);
        }
    }

    #[test]
    fn should_return_empty_list_when_no_matching_photos() {
        let album_id = 1;
        let _mock = mock("GET", "/photos")
            .match_query(format!("albumId={}", album_id).as_str())
            .with_status(200)
            .with_body("[]")
            .create();
        let photo_collection = PhotoCollection {
            url: mockito::server_url(),
        };

        let photos = photo_collection.get_photos_by_album(album_id).unwrap();

        assert_eq!(photos.len(), 0);
    }
}
