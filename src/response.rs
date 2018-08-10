
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
}

fn _phrase_from_code(code: i32) -> String {
    String::from(match code {
        200 => "OK",
        404 => "NOT FOUND",
        _   => "",
    })
}

