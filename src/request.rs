
use std::collections::HashMap;
use std::io::Error;
use std::io::ErrorKind;
use std::io::{BufReader, BufRead, Read};
use std::io::Result;

pub struct Request {
    pub method:  String,
    pub target:  String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body:    String,
}

impl Request {
    fn parse_start_line(istream: &mut Read) -> Result<(String, String, String)> {
        let mut reader = BufReader::new(istream);
        let mut buf = String::new();
        match reader.read_line(&mut buf) {
            Ok(_)  => {
                let mut split = buf.split_whitespace();
                let method = split.next().map_or(String::new(), |x| String::from(x));
                let target = split.next().map_or(String::new(), |x| String::from(x));
                let version = split.next().map_or(String::new(), |x| String::from(x));
                Ok((method, target, version))
            },
            _ => Err(Error::new(ErrorKind::Other, "Error parsing start line"))
        }
    }

    pub fn parse(istream: &mut Read) -> Result<Request> {
        match Request::parse_start_line(istream) {
            Ok((method, target, version)) => Ok(Request {
                method,
                target,
                version,
                headers: HashMap::new(),
                body:    String::new()
            }),
            Err(e) => Err(e),
        }
    }
}

