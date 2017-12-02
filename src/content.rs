//! Ferrum's HTTP response Content representation and associated methods.

use std::ops::Deref;
use std::borrow::Cow;

use hyper::Body;

#[derive(Debug)]
pub struct Content(pub Vec<u8>);

impl Deref for Content {
    type Target = Vec<u8>;

    #[inline]
    fn deref(&self) -> &Vec<u8> {
        &self.0
    }
}

impl From<Vec<u8>> for Content {
    #[inline]
    fn from(vec: Vec<u8>) -> Content {
        Content(vec)
    }
}

impl<'a> From<&'a [u8]> for Content {
    #[inline]
    fn from(slice: &'a [u8]) -> Content {
        Content(slice.to_vec())
    }
}

impl<'a> From<Cow<'a, [u8]>> for Content {
    #[inline]
    fn from (cow: Cow<'a, [u8]>) -> Content {
        match cow {
            Cow::Borrowed(b) => Content::from(b),
            Cow::Owned(o) => Content::from(o)
        }
    }
}

impl From<String> for Content {
    #[inline]
    fn from(string: String) -> Content {
        Content(string.as_bytes().to_vec())
    }
}

impl<'a> From<&'a str> for Content {
    #[inline]
    fn from(slice: &'a str) -> Content {
        Content(slice.as_bytes().to_vec())
    }
}

impl<'a> From<Cow<'a, str>> for Content {
    #[inline]
    fn from (cow: Cow<'a, str>) -> Content {
        match cow {
            Cow::Borrowed(b) => Content::from(b),
            Cow::Owned(o) => Content::from(o)
        }
    }
}

impl Into<Body> for Content {
    #[inline]
    fn into(self) -> Body {
        self.0.into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deref_content() {
        let content = Content(vec![1, 2]);

        assert_eq!(vec![1, 2], *content);
        assert_eq!(vec![1, 2].len(), content.len());
    }
}