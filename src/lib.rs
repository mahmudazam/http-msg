
pub mod request;
pub mod response;

#[cfg(test)]
mod tests {

    use request::Request;
    use response::Response;

    #[test]
    fn test_req_parse() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_res_parse() {
        assert_eq!(2 + 2, 4);
    }
}
