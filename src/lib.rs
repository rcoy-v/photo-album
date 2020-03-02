use std::error::Error;
use std::io::Write;

use reqwest::blocking::get;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Photo {
    #[serde(rename = "albumId")]
    album_id: usize,
    id: usize,
    title: String,
    url: String,
    #[serde(rename = "thumbnailUrl")]
    thumbnail_url: String,
}

struct PhotoCollection {}

impl PhotoCollection {
    pub fn get_photos_by_album(album_id: &str) -> Result<Vec<Photo>, reqwest::Error> {
        let photos_url = &format!(
            "https://jsonplaceholder.typicode.com/photos?albumId={}",
            album_id
        );

        get(photos_url)?.json::<Vec<Photo>>()
    }
}

fn print_photo_info<W: Write>(writer: &mut W, photos: Vec<Photo>) {
    if photos.is_empty() {
        writeln!(writer, "no photos found").unwrap();
    } else {
        for photo in photos {
            writeln!(writer, "[{}] {}", photo.id, photo.title).unwrap();
        }
    }
}

pub fn run<W: Write>(args: Vec<String>, writer: &mut W) -> Result<(), Box<dyn Error>> {
    let album_id = args.get(1).expect("album id not provided");
    let photos = PhotoCollection::get_photos_by_album(album_id)?;

    print_photo_info(writer, photos);

    Ok(())
}

#[cfg(test)]
mod print_photo_info_tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn when_empty() {
        let mut writer = Cursor::new(Vec::new());

        print_photo_info(&mut writer, vec![]);

        assert_eq!(
            String::from_utf8(writer.into_inner()).unwrap(),
            "no photos found\n"
        )
    }
}
