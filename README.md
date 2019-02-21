# bender_mq

bender_mq is a rust library, that implements additional methods for the
amqp crate via traits

It can be loaded in a rust library via the public git mirror by putting this in your Cargo.toml:
```
[dependencies]
bender_mq = { git = "ssh://git@code.hfbk.net:4242/bendercode/bender-mq.git" }
```
To update this run
```
cargo clean
cargo update
```

### Testing
The tests can be run with
```
cargo test
```

### Documentation
If you want to view the documentation run
```
cargo doc --no-deps --open
```

### Installation
This is a library and will not be directly used. No need to install anything here

### API Methods
Check out the examples at the [BenderMQ](trait.BenderMQ.html) trait definition

License: MIT
