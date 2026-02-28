# firefly-toml

A Rust macro for statically parsing [firefly.toml](https://docs.fireflyzero.com/dev/config/).

## Usage

It can be used through [firefly-rust](https://github.com/firefly-zero/firefly-rust) by activating `firefly-toml` crate feature:

```bash
cargo add firefly-rust --features firefly-toml
```

Which will add `firefly_rust::toml` module with the helpers based on the `firefly.toml` config:

```rust
use firefly_rust::toml;

toml::some_badge(peer, 123);
```

Or can be used directly via macro adding everything into the current scope:

```rust
mod toml {
    firefly_toml::import!();
}

toml::some_badge(peer, 123);
```

## License

MIT License. You can do whatever you want with the code in here, modify it, embed into any apps and games. Have fun!

Originally developed by [oli-obk](https://github.com/oli-obk) and is currently officially maintained by the Firefly Zero core team.
