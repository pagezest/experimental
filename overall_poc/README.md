# Overall PageZest POC

To start the webserver run the following
```
cargo run --release
```

Hit the following GET request in order to call WASM module and run addition function from it.

```
http://localhost:8080/?a=15&b=15
```

Try with different query parameters values.
