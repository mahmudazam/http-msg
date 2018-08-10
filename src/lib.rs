
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
    fn test_req_parse() {
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
    fn test_req_parse_exact() {
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

    #[test]
    fn test_res_new() {
        let res = Response::new("1.1", 200);
        assert_eq!(res.version, String::from("1.1"));
        assert_eq!(res.response_code, 200);
        assert_eq!(res.response_phrase, String::from("OK"));
        assert_eq!(res.body.len(), 0);
    }

    #[test]
    fn test_res_headers() {
        let version = "1.1";
        let res = Response::new(&version, 200)
            .set_header("foo", "foo_val")
            .set_header("bar", "bar_val");
        assert_eq!(res.get_header("foo"), Some(&String::from("foo_val")));
        assert_eq!(res.get_header("bar"), Some(&String::from("bar_val")));
    }

    #[test]
    fn test_res_set_body() {
        let version = "1.1";
        let res = Response::new(&version, 200)
            .set_body("Hello, World!\r\n".as_bytes());
        assert_eq!(res.body, "Hello, World!\r\n".as_bytes());
    }

    #[test]
    fn test_res_serialize() {
        let version = "1.1";
        let res = Response::new(&version, 200)
            .set_header("foo", "foo_val")
            .set_header("bar", "bar_val")
            .set_body("Hello, World!\r\n\r\n".as_bytes());
        let bytes = res.serialize();

        let start_line = "HTTP/1.1 200 OK\r\n";
        let headers1 = "foo: foo_val\r\nbar: bar_val\r\n\r\n";
        let headers2 = "bar: bar_val\r\nfoo: foo_val\r\n\r\n";
        let body = "Hello, World!\r\n\r\n";
        let exp_bytes1 = format!("{}{}{}", start_line, headers1, body);
        let exp_bytes2 = format!("{}{}{}", start_line, headers2, body);

        if bytes != exp_bytes1.as_str().as_bytes() {
            assert_eq!(bytes, exp_bytes2.as_str().as_bytes());
        } else {
            assert_eq!(bytes, exp_bytes1.as_str().as_bytes());
        }
    }
}

