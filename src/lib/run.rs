use std::error::Error;
use std::fmt;
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

#[derive(Debug)]
struct ParseError;

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse Error")
    }
}

fn parse_album_id(args: Vec<String>) -> Result<usize, ParseError> {
    match args.get(1) {
        Some(i) => match i.parse::<usize>() {
            Ok(album_id) => Ok(album_id),
            Err(_) => Err(ParseError {}),
        },
        None => Err(ParseError {}),
    }
}

pub fn run<W: Write>(args: Vec<String>, writer: &mut W) -> Result<(), Box<dyn Error>> {
    let run = || -> Result<(), Box<dyn Error>> {
        let album_id = parse_album_id(args)?;
        let photos = PhotoCollection::new().get_photos_by_album(album_id)?;

        print_photo_info(writer, photos);

        Ok(())
    };

    if let Err(e) = run() {
        writeln!(writer, "no photos found").unwrap();
        Err(e)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_use_album_id_if_given() {
        let args = vec!["test".to_string(), "1".to_string()];

        let album_id = parse_album_id(args).unwrap();

        assert_eq!(1, album_id);
    }

    #[test]
    fn should_err_if_no_album_id_given() {
        let args = vec!["test".to_string()];

        let result = parse_album_id(args);

        assert!(result.is_err());
    }

    #[test]
    fn should_err_if_invalid_album_id_given() {
        let args = vec!["test".to_string(), "foo".to_string()];

        let result = parse_album_id(args);

        assert!(result.is_err());
    }
}
