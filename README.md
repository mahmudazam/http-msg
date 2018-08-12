# http-msg
### HTTP Request/Response Library for Rust:
This library is primarily intended to parse HTTP requests and responses from
byte streams. It has no dependencies other than the Rust standard library.

A full implementation of the HTTP specification has yet to be done.

### Current Capabilities:

* Parsing Requests from arbitrary streams
```rust
let server = TcpListener::bind("127.0.0.1:8080").unwrap();
let (mut client, addr) = server.accept().unwrap();
let req = Request::parse_header(&mut client).unwrap();
print!("Request received from {:?}:\n{}", addr, req.to_string());
```

* Separate parsing of Request headers and body
```rust
let mut req_str = "GET / HTTP/1.1\r\n".to_owned()
                + "Content-Type: text/plain\r\n\r\n";
let req = Request::parse_header(&mut req_str.as_bytes()).unwrap();
let mut req_body_str = "Lorem impsum dolor\r\n\r\n";
let req = req.parse_body(&mut req_body_str.as_bytes()).unwrap();
print!("{}", req.to_string());
```

* Building requests by specifying fields
```rust
let mut req = Request::new("GET", "/", "1.1");
req.set_header("Content-Type", "text/html");
req.set_header("Accept-Encoding", "utf-8");
print!("{}", req.to_string());
```

* Serialization of responses to byte vectors
```rust
let res = Response::new("1.1", 200)
    .set_header("Content-Type", "text/plain")
    .set_header("Encoding", "utf-8")
    .set_body("Lorem ipsum dolor\r\n\r\n".as_bytes());
client.write(res.serialize().as_slice());
```

### Build:
```cargo build```

### Run Tests:
```cargo test```
