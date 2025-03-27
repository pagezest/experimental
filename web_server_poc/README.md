# Web Server POC

We have benchmark multiple web-servers
- actix
- axum
- may_minihttp
- tiny-http

To Start the webserver using the following command

```
cargo run --release --bin actix
cargo run --release --bin may_minihttp
cargo run --release --bin axum
cargo run --release --bin tiny-http
```

Visit localhost at port 8080 in your browser or make API call.

```
http://localhost:8080/
```
