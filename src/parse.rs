use std::{fmt, vec};

use bytes::Bytes;

use crate::frame::Frame;

#[derive(Debug)]
pub(crate) struct Parse {
    parts: vec::IntoIter<Frame>,
}

#[derive(Debug)]
pub(crate) enum ParseError {
    EndOfStream,
    Other(crate::Error),
}

impl Parse {
    pub(crate) fn new(frame: Frame) -> Result<Parse, ParseError> {
        let array = match frame {
            Frame::Array(array) => array,
            frame => return Err(format!("protocol error; expected array, got {frame:?}").into()),
        };

        Ok(Parse {
            parts: array.into_iter(),
        })
    }

    fn next(&mut self) -> Result<Frame, ParseError> {
        self.parts.next().ok_or(ParseError::EndOfStream)
    }

    pub(crate) fn next_string(&mut self) -> Result<String, ParseError> {
        match self.next()? {
            Frame::Simple(s) => Ok(s),
            Frame::Bulk(data) => str::from_utf8(&data[..])
                .map(|s| s.to_string())
                .map_err(|_| "protocol error; invalid string".into()),
            frame => Err(format!(
                "protocol error: expected simple frame or bulk frame; got {frame:?}"
            )
            .into()),
        }
    }

    pub(crate) fn next_bytes(&mut self) -> Result<Bytes, ParseError> {
        match self.next()? {
            Frame::Simple(s) => Ok(Bytes::from(s.into_bytes())),
            Frame::Bulk(data) => Ok(data),
            frame => Err(format!(
                "protocol error; expected simple frame or bulk frame; got {frame:?}"
            )
            .into()),
        }
    }

    pub(crate) fn next_int(&mut self) -> Result<u64, ParseError> {
        use atoi::atoi;

        const MSG: &str = "protocol error; invalid number";

        match self.next()? {
            Frame::Integer(v) => Ok(v),
            Frame::Simple(s) => atoi::<u64>(s.as_bytes()).ok_or_else(|| MSG.into()),
            Frame::Bulk(data) => atoi::<u64>(&data).ok_or_else(|| MSG.into()),
            frame => Err(format!("protocol error; expected int frame but got {frame:?}").into()),
        }
    }

    pub(crate) fn finish(&mut self) -> Result<(), ParseError> {
        if self.parts.next().is_none() {
            Ok(())
        } else {
            Err("protocol error; expected end of frame, but there was more".into())
        }
    }
}

impl From<String> for ParseError {
    fn from(src: String) -> Self {
        ParseError::Other(src.into())
    }
}

impl From<&str> for ParseError {
    fn from(src: &str) -> Self {
        src.to_string().into()
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EndOfStream => "protocol error; unexpected end of stream".fmt(f),
            ParseError::Other(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::Frame;

    #[test]
    fn parse_next_string() {
        let frame = Frame::Array(vec![
            Frame::Bulk("hello".into()),
            Frame::Bulk("world".into()),
        ]);
        let mut parse = Parse::new(frame).unwrap();
        assert_eq!(parse.next_string().unwrap(), "hello");
        assert_eq!(parse.next_string().unwrap(), "world");
        assert!(parse.next_string().is_err());
    }

    #[test]
    fn parse_next_bytes() {
        let frame = Frame::Array(vec![
            Frame::Simple("hello".into()),
            Frame::Bulk("world".into()),
            Frame::Integer(42),
        ]);
        let mut parse = Parse::new(frame).unwrap();
        assert_eq!(parse.next_bytes().unwrap(), "hello");
        assert_eq!(parse.next_bytes().unwrap(), "world");
        assert!(parse.next_bytes().is_err());
    }

    #[test]
    fn parse_next_int() {
        let frame = Frame::Array(vec![
            Frame::Integer(42),
            Frame::Simple("43".into()),
            Frame::Bulk("44".into()),
            Frame::Bulk("not a number".into()),
        ]);
        let mut parse = Parse::new(frame).unwrap();
        assert_eq!(parse.next_int().unwrap(), 42);
        assert_eq!(parse.next_int().unwrap(), 43);
        assert_eq!(parse.next_int().unwrap(), 44);
        assert!(parse.next_int().is_err());
    }

    #[test]
    fn parse_finish_ok() {
        let frame = Frame::Array(vec![
            Frame::Bulk("hello".into()),
            Frame::Bulk("world".into()),
        ]);
        let mut parse = Parse::new(frame).unwrap();
        let _ = parse.next_string().unwrap();
        let _ = parse.next_string().unwrap();
        assert!(parse.finish().is_ok());
    }

    #[test]
    fn parse_finish_err() {
        let frame = Frame::Array(vec![
            Frame::Bulk("hello".into()),
            Frame::Bulk("world".into()),
        ]);
        let mut parse = Parse::new(frame).unwrap();
        let _ = parse.next_string().unwrap();
        // let _ = parse.next_string().unwrap();
        assert!(parse.finish().is_err());
    }

    #[test]
    fn parse_non_array_err() {
        let frame = Frame::Simple("not an array".into());
        assert!(Parse::new(frame).is_err());
    }
}
