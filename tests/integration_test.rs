use std::fs;
use std::io::Cursor;

use photo_album::run::run;

#[test]
fn should_print_message_when_no_photos() {
    let mut writer = create_writer();
    let args = vec!["test".to_string(), "10000".to_string()];

    let result = run(args, &mut writer);

    assert!(result.is_ok());
    assert_eq!(
        String::from_utf8(writer.into_inner()).unwrap(),
        "no photos found\n"
    );
}

#[test]
fn should_print_info_for_found_photos() {
    let mut writer = create_writer();
    let args = vec!["test".to_string(), "50".to_string()];
    let expected = fs::read_to_string("tests/data/photos.txt").unwrap();

    let result = run(args, &mut writer);

    assert!(result.is_ok());
    assert_eq!(expected, String::from_utf8(writer.into_inner()).unwrap(),);
}

#[test]
fn should_print_error_message_when_missing_argument() {
    let mut writer = create_writer();
    let args = vec!["test".to_string()];

    let result = run(args, &mut writer);

    assert!(result.is_err());
    assert_eq!(
        "no photos found\n".to_string(),
        String::from_utf8(writer.into_inner()).unwrap(),
    );
}

#[test]
fn should_print_error_message_when_album_id_not_parsable() {
    let mut writer = create_writer();
    let args = vec!["test".to_string(), "foo".to_string()];

    let result = run(args, &mut writer);

    assert!(result.is_err());
    assert_eq!(
        "no photos found\n".to_string(),
        String::from_utf8(writer.into_inner()).unwrap(),
    );
}

fn create_writer() -> Cursor<Vec<u8>> {
    Cursor::new(Vec::new())
}
