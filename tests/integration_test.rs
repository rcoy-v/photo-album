use std::fs;
use std::io::Cursor;

use mockito::{Matcher, Mock};

use photo_album::run::run;

struct Setup {
    writer: Cursor<Vec<u8>>,
    args: Vec<String>,
    mock: Mock,
}

impl Setup {
    fn new(args: Vec<String>, mock: Mock) -> Setup {
        Setup {
            writer: Cursor::new(Vec::new()),
            args,
            mock,
        }
    }
}

mod when_photos_found {
    use super::*;

    fn setup() -> Setup {
        let matching_album_id = "50".to_string();
        let response = fs::read_to_string("tests/data/photos_5000.json").unwrap();

        let mock = mockito::mock("GET", "/photos")
            .match_query(format!("albumId={}", matching_album_id).as_str())
            .with_status(200)
            .with_body(response)
            .create();

        Setup::new(vec!["test".to_string(), matching_album_id], mock)
    }

    #[test]
    fn should_return_ok() {
        let mut s = setup();

        let result = run(s.args, &mut s.writer);

        assert!(result.is_ok());
    }

    #[test]
    fn should_print_photo_information() {
        let mut s = setup();
        let expected = fs::read_to_string("tests/data/photos_5000.txt").unwrap();

        let _result = run(s.args, &mut s.writer);

        assert_eq!(expected, String::from_utf8(s.writer.into_inner()).unwrap());
    }
}

mod when_no_matching_photos {
    use super::*;

    fn setup() -> Setup {
        let empty_album = "10000";
        let args = vec!["test".to_string(), empty_album.to_string()];
        let mock = mockito::mock("GET", "/photos")
            .match_query(format!("albumId={}", empty_album).as_str())
            .with_status(200)
            .with_body("[]")
            .create();

        Setup::new(args, mock)
    }

    #[test]
    fn should_return_ok() {
        let mut s = setup();

        let result = run(s.args, &mut s.writer);

        assert!(result.is_ok());
    }

    #[test]
    fn should_print_no_photos_found() {
        let mut s = setup();

        let _result = run(s.args, &mut s.writer);

        assert_eq!(
            String::from_utf8(s.writer.into_inner()).unwrap(),
            "no photos found\n"
        );
    }
}

mod when_album_id_not_given {
    use super::*;

    fn setup() -> Setup {
        let args = vec!["test".to_string()];
        let mock = mockito::mock("GET", "/photos").expect_at_most(0).create();

        Setup::new(args, mock)
    }

    #[test]
    fn should_return_error() {
        let mut s = setup();

        let result = run(s.args, &mut s.writer);

        assert!(result.is_err());
    }

    #[test]
    fn should_print_error_finding_photos() {
        let mut s = setup();

        let _result = run(s.args, &mut s.writer);

        assert_eq!(
            "error finding photos\n".to_string(),
            String::from_utf8(s.writer.into_inner()).unwrap(),
        );
    }

    #[test]
    fn should_not_make_http_get() {
        let mut s = setup();

        let _result = run(s.args, &mut s.writer);

        s.mock.assert();
    }
}

mod when_album_id_not_parsable {
    use super::*;

    fn setup() -> Setup {
        let not_parsable_album = "foo123";
        let args = vec!["test".to_string(), not_parsable_album.to_string()];
        let mock = mockito::mock("GET", "/photos").expect_at_most(0).create();

        Setup::new(args, mock)
    }

    #[test]
    fn should_return_error() {
        let mut s = setup();

        let result = run(s.args, &mut s.writer);

        assert!(result.is_err());
    }

    #[test]
    fn should_print_error_finding_photos() {
        let mut s = setup();

        let _result = run(s.args, &mut s.writer);

        assert_eq!(
            "error finding photos\n".to_string(),
            String::from_utf8(s.writer.into_inner()).unwrap(),
        );
    }

    #[test]
    fn should_not_make_http_get() {
        let mut s = setup();

        let _result = run(s.args, &mut s.writer);

        s.mock.assert();
    }
}

mod when_http_get_fails {
    use super::*;

    fn setup() -> Setup {
        let any_album = "123";
        let args = vec!["test".to_string(), any_album.to_string()];
        let mock = mockito::mock("GET", Matcher::Any).with_status(500).create();

        Setup::new(args, mock)
    }

    #[test]
    fn should_return_error() {
        let mut s = setup();

        let result = run(s.args, &mut s.writer);

        assert!(result.is_err());
        s.mock.assert();
    }

    #[test]
    fn should_print_error_finding_photos() {
        let mut s = setup();

        let _result = run(s.args, &mut s.writer);

        assert_eq!(
            "error finding photos\n".to_string(),
            String::from_utf8(s.writer.into_inner()).unwrap(),
        );
    }
}
