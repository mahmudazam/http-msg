
use std::collections::HashMap;

pub struct Response {
    pub version:            String,
    pub response_phrase:    String,
    pub response_code:      i32,
    headers:                HashMap<String, String>,
    pub body:               Vec<u8>,
}

impl Response {
    pub fn new(version: &str, response_code: i32) -> Response {
        Response {
            version:            String::from(version),
            response_code:      response_code,
            response_phrase:    _phrase_from_code(response_code),
            headers:            HashMap::new(),
            body:               Vec::new(),
        }
    }

    pub fn get_header(&self, name: &str) -> Option<&String> {
        self.headers.get(&String::from(name))
    }

    pub fn set_header(mut self, name: &str, value: &str) -> Self {
        self.headers.insert(String::from(name), String::from(value));
        self
    }

    pub fn set_body(mut self, body: &[u8]) -> Self {
        self.body.clear();
        self.body.extend_from_slice(&body);
        self
    }

    pub fn serialize(&self) -> Vec<u8> {
        let str_rep = _to_str_without_body(self);
        let mut ret = Vec::new();
        ret.extend_from_slice(str_rep.as_str().as_bytes());
        ret.extend_from_slice(self.body.as_slice());
        ret
    }
}

fn _to_str_without_body(res: &Response) -> String {
    let start_line = format!("HTTP/{} {} {}",
        res.version, res.response_code, res.response_phrase);
    let headers = res.headers.iter().fold(String::from(""),
            |acc, next| format!("{}{}: {}\r\n", acc, next.0, next.1))
            + "\r\n";
    format!("{}\r\n{}", start_line, headers)
}

fn _phrase_from_code(code: i32) -> String {
    String::from(match code {
        200 => "OK",
        404 => "NOT FOUND",
        _   => "",
    })
}

