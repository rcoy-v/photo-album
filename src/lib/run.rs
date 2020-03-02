use std::error::Error;
use std::io::Write;

use super::photo_collection::*;

fn print_photo_info(photos: Vec<Photo>) -> String {
    if photos.is_empty() {
        String::from("no photos found\n")
    } else {
        let mut info = String::new();

        for photo in photos {
            info = format!("{}[{}] {}\n", info, photo.id, photo.title);
        }

        info
    }
}

pub fn run<W: Write>(args: Vec<String>, writer: &mut W) -> Result<(), Box<dyn Error>> {
    let album_id = args.get(1).expect("album id not provided");
    let photos = PhotoCollection::get_photos_by_album(album_id)?;

    writer
        .write_all(print_photo_info(photos).as_bytes())
        .unwrap();

    Ok(())
}

#[cfg(test)]
mod print_photo_info_tests {
    use super::*;

    #[test]
    fn should_return_formatted_photo_id_and_title() {
        let photos = vec![
            Photo {
                id: 1,
                title: "foo".to_string(),
                ..Default::default()
            },
            Photo {
                id: 2,
                title: "bar".to_string(),
                ..Default::default()
            },
        ];

        let expected = String::from("[1] foo\n[2] bar\n");

        let message = print_photo_info(photos);

        assert_eq!(expected, message);
    }

    #[test]
    fn should_return_warning_message_when_no_photos() {
        let photos = vec![];

        let message = print_photo_info(photos);

        assert_eq!("no photos found\n", message);
    }
}
