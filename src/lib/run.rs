use std::error::Error;
use std::io::Write;

use super::photo_collection::*;

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
    use std::default::Default;
    use std::io::Cursor;

    use super::*;

    #[test]
    fn should_print_message_when_no_photos() {
        let mut writer = Cursor::new(Vec::new());

        print_photo_info(&mut writer, vec![]);

        assert_eq!(
            String::from_utf8(writer.into_inner()).unwrap(),
            "no photos found\n"
        )
    }

    #[test]
    fn should_print_each_photos_id_and_title() {
        let mut writer = Cursor::new(Vec::new());
        let photos = vec![
            Photo { id: 1, title: String::from("foo"), ..Default::default()},
            Photo { id: 2, title: String::from("bar"), ..Default::default()},
        ];


        print_photo_info(&mut writer, photos);

        assert_eq!(
            String::from_utf8(writer.into_inner()).unwrap(),
            "[1] foo\n[2] bar\n"
        )
    }
}