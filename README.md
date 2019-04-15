# Healthy Service

A simple service that responds with an OK 200.

## Technologies

* [Rust](https://www.rust-lang.org)
* [Iron](http://ironframework.io)

## Building

Using `cargo`:

```
cargo build
```

To build a release-ready binary:

```
cargo build --release
```

## Running

After building, you can run the binary:

```
$ ./target/debug/healty-service-rust 
Server on http://localhost:8080
```

Or, if you built the release version:

```
$ ./target/release/healty-service-rust 
Server on http://localhost:8080
```

## Checking the service

To check that the service is running:

```
$ curl -i http://localhost:8080
HTTP/1.1 200 OK
Content-Type: application/json
Date: Mon, 15 Apr 2019 05:25:59 GMT
Transfer-Encoding: chunked

{"status":"ok"}
```
