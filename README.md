# http-protocol

## Headers

```rust
extern crate http_protocol;

use http_protocol::header::Header;
use http_protocol::Headers;

fn main() {
    let mut headers = Headers::new();
    headers.insert(Header::Connection, &b"close"[..]);
    println!("{:?}", headers);
}
```

## Response

```rust
extern crate http_protocol;

use http_protocol::header::Header;
use http_protocol::{Body, Headers, HttpVersion, Response, StatusCode};

fn main() {
    let mut headers = Headers::new();
    headers.insert(Header::Connection, &b"close"[..]);

    let mut response_builder = Response::builder();

    response_builder
        .http_version(HttpVersion::Http11)
        .status_code(StatusCode::S200)
        .headers(headers)
        .body(Body::empty());

    let response = response_builder.build().unwrap();

    println!("{:?}", response);
}
```