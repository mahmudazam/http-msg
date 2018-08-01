
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
}

impl ToString for Request {
    fn to_string(&self) -> String {
        let start_line = format!("{} {} HTTP/{}\r\n",
                            self.method, self.target, self.version);
        let headers = self.headers
            .iter()
            .fold(String::from(""),
                  |acc, next| format!("{}{}: {}\r\n", acc, next.0, next.1))
            + "\r\n";
        let body = String::from_utf8(self.body.clone());
        format!("{}{}{}\r\n\r\n",
            start_line, headers, body.unwrap_or(String::new()))
    }
}

