# bender_mq

bender_mq is a rust library, that implements additional methods for the
amqp crate via traits

It can be loaded in a rust library via the public git mirror by putting this in your Cargo.toml:
```rust
[dependencies]
bender_mq = { git = "https://github.com/atoav/bender-mq.git" }
```
To update this run
```rust
cargo clean
cargo update
```

### Testing
The tests can be run with
```rust
cargo test
```

### Documentation
If you want to view the documentation run
```rust
cargo doc --no-deps --open
```

### Installation
To run cargo, make sure you have rust installed. Go to [rustup.rs](http://rustup.rs) and follow the instructions there

