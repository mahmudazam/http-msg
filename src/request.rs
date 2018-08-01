
use std::collections::HashMap;

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
            version: String::from("HTTP/".to_owned() + version),
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
}

