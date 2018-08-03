
use std::collections::HashMap;

use std::io::{
    Read,
    BufRead,
    BufReader,
};

pub struct Request {
    pub method:  String,
    pub target:  String,
    pub version: String,
    headers:     HashMap<String, String>,
    pub body:    Vec<u8>,
}

impl Request {
    pub fn new(method: &str, target: &str, version: &str) -> Request {
        Request {
            method:  String::from(method),
            target:  String::from(target),
            version: String::from(version),
            headers: HashMap::new(),
            body:    Vec::new()
        }
    }

    pub fn get_header(&self, name: &str) -> Option<&String> {
        self.headers.get(&String::from(name))
    }

    pub fn set_header(&mut self, name: &str, value: &str) -> &Self {
        self.headers.insert(String::from(name), String::from(value));
        self
    }

    fn _read_start_line(reader: &mut BufReader<&mut Read>)
            -> Result<Request, String> {
        let mut buf = String::new();
        reader.read_line(&mut buf)
            .map_err(|_| String::from("Could not read start line."))
            .map(|_| buf.as_str().split_whitespace())
            .and_then(|mut split| {
                let method = split.next().unwrap_or(&"");
                let target = split.next().unwrap_or(&"");
                let version = &split.next().unwrap_or(&"")
                                           .replace("HTTP/", "");
                if "" == method || "" == target || "" == version {
                    Err(String::from("Invalid start line"))
                } else {
                    Ok(Request::new(method, target, version))
                }
            })
    }

    fn _read_headers(reader: &mut BufReader<&mut Read>, mut req: Request)
            -> Result<Request, String> {
        let mut header_buf = String::new();
        while let Ok(size) = reader.read_line(&mut header_buf) {
            if size > 2  {
                let mut split = header_buf.as_str().trim().split(":");
                let key = split.next().unwrap_or("").trim();
                let val = split.next().unwrap_or("").trim();
                req.set_header(key, val);
            } else {
                break;
            }
            header_buf.clear();
        };
        Ok(req)
    }

    fn _read_body(reader: &mut BufReader<&mut Read>, req: Request)
            -> Result<Request, String> {
        match req.get_header("Content-Length") {
            Some(n) => { ; },
            None => { ; }
        }
        Ok(req)
    }

    pub fn parse(source: &mut Read) -> Result<Request, String> {
        let mut reader = BufReader::new(source);
        Self::_read_start_line(&mut reader)
            .and_then(|req| Request::_read_headers(&mut reader, req))
            .and_then(|req| Request::_read_body(&mut reader, req))
    }
}

impl ToString for Request {
    fn to_string(&self) -> String {
        let start_line = format!("{} {} HTTP/{}\r\n",
                            self.method, self.target, self.version);
        let headers = self.headers.iter().fold(String::from(""),
            |acc, next| format!("{}{}: {}\r\n", acc, next.0, next.1))
            + "\r\n";
        let body = String::from_utf8(self.body.clone())
                          .unwrap_or(String::from("[Body is not string]"));
        format!("{}{}{}\r\n", start_line, headers, if "" == body
            { String::from("") } else { format!("{}\r\n", body) })
    }
}

