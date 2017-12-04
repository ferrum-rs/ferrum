//! Ferrum's extra URI methods.

use std::str::Split;

use hyper::Uri;
use url::percent_encoding::percent_decode;

pub trait UriPathSegments {
    fn path_segments(&self) -> Split<char>;

    fn decoded_path_segments(&self) -> Vec<String>;
}

impl UriPathSegments for Uri {
    fn path_segments(&self) -> Split<char> {
        let path = self.path();
        if path.starts_with('/') {
            path[1..].split('/')
        } else {
            path.split('/')
        }
    }

    fn decoded_path_segments(&self) -> Vec<String> {
        self.path_segments().map(|segment| {
            match percent_decode(segment.as_bytes()).decode_utf8() {
                Ok(decoded) => decoded.to_string(),
                Err(_) => segment.to_string()
            }
        }).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_path_segments() {
        let uri = Uri::from_str("https://example.com/foo/bar").unwrap();
        let mut path_segments = uri.path_segments();
        assert_eq!(path_segments.next(), Some("foo"));
        assert_eq!(path_segments.next(), Some("bar"));
        assert_eq!(path_segments.next(), None);

        let uri = Uri::from_str("https://example.com").unwrap();
        let mut path_segments = uri.path_segments();
        assert_eq!(path_segments.next(), Some(""));
        assert_eq!(path_segments.next(), None);

        let uri = Uri::from_str("https://example.com/countries/vi%E1%BB%87t%20nam").unwrap();
        let mut path_segments = uri.path_segments();
        assert_eq!(path_segments.next(), Some("countries"));
        assert_eq!(path_segments.next(), Some("vi%E1%BB%87t%20nam"));
    }

    #[test]
    fn test_decoded_path_segments() {
        let uri = Uri::from_str("https://example.com/foo/bar").unwrap();
        let path_segments = uri.decoded_path_segments();
        let mut path_segments = path_segments.iter().map(|s| s.as_ref());
        assert_eq!(path_segments.next(), Some("foo"));
        assert_eq!(path_segments.next(), Some("bar"));
        assert_eq!(path_segments.next(), None);

        let uri = Uri::from_str("https://example.com").unwrap();
        let path_segments = uri.decoded_path_segments();
        let mut path_segments = path_segments.iter().map(|s| s.as_ref());
        assert_eq!(path_segments.next(), Some(""));
        assert_eq!(path_segments.next(), None);

        let uri = Uri::from_str("https://example.com/countries/vi%E1%BB%87t%20nam").unwrap();
        let path_segments = uri.decoded_path_segments();
        let mut path_segments = path_segments.iter().map(|s| s.as_ref());
        assert_eq!(path_segments.next(), Some("countries"));
        assert_eq!(path_segments.next(), Some("việt nam"));
    }
}
