
pub mod request;
pub mod response;

#[cfg(test)]
mod tests {

    use request::Request;
    use response::Response;

    #[test]
    fn test_req_create() {
        let req = Request::new("GET", "/hello", "1.1");
        assert_eq!(req.method, String::from("GET"));
        assert_eq!(req.target, String::from("/hello"));
        assert_eq!(req.version, String::from("1.1"));
    }

    #[test]
    fn test_req_get_set_header() {
        let name = "Content-Type";
        let value = "text/html";
        let mut req = Request::new("GET", "/hello", "1.1");
        assert_eq!(req.get_header(name), None);
        assert_eq!(req.set_header(name, value)
                      .get_header(name),
                   Some(&String::from(value)));
    }

    #[test]
    fn test_req_to_string() {
        let name = "Content-Type";
        let value = "text/html";
        let body = "Hello, World!";
        let mut req = Request::new("GET", "/hello", "1.1");
        req.set_header(name, value);
        req.body.append(&mut body.as_bytes().to_vec());
        print!("To String:\n{}", req.to_string());
        assert_eq!(req.to_string(),
            format!("{}\r\n{}: {}\r\n\r\n{}\r\n\r\n",
                "GET /hello HTTP/1.1",
                name, value,
                body));
    }

    #[test]
    fn test_parse() {
        let mut msg = String::new();
        msg.push_str("GET /hello HTTP/1.1\r\n");
        msg.push_str("Content-Type: text/html\r\n");
        msg.push_str("Accept-Charset: utf-8\r\n\r\n");
        msg.push_str("Hello, World!\r\n\r\n");
        let parse_result = Request::parse(&mut msg.as_bytes());
        assert!(parse_result.is_ok());
        let parse = parse_result.unwrap();
        print!("To String After Parse:\n{}", parse.to_string());
        assert_eq!(parse.method, "GET");
        assert_eq!(parse.target, "/hello");
        assert_eq!(parse.version, "1.1");
        assert_eq!(parse.get_header("Content-Type"),
                   Some(&String::from("text/html")));
        assert_eq!(parse.get_header("Accept-Charset"),
                   Some(&String::from("utf-8")));
        assert_eq!(parse.body.len(), 15);
        let body = String::from_utf8(parse.body).unwrap_or(String::new());
        assert_eq!(body, String::from("Hello, World!\r\n"));
    }

    #[test]
    fn test_parse_exact() {
        let mut msg = String::new();
        msg.push_str("GET /hello HTTP/1.1\r\n");
        msg.push_str("Content-Length: 17\r\n\r\n");
        msg.push_str("Hello, World!\r\n\r\n");
        let parse_result = Request::parse(&mut msg.as_bytes());
        assert!(parse_result.is_ok());
        let parse = parse_result.unwrap();
        assert_eq!(
            usize::from_str_radix(
                parse.get_header("Content-Length").unwrap(), 10),
            Ok(17));
        assert_eq!(parse.body.len(), 17);
        let body = String::from_utf8(parse.body).unwrap_or(String::new());
        assert_eq!(body, String::from("Hello, World!\r\n\r\n"));
    }
}

