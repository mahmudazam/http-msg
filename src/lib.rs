
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
        assert_eq!(req.version, String::from("HTTP/1.1"));
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

}

