FiX meets Serde
===================================

This crate is a Rust library for serialising to and deserialising from
the [FiX] format. It is built
upon [Serde], a high performance generic serialization framework.

[Serde]: https://github.com/serde-rs/serde
[FiX]: http://www.fixprotocol.org/

Installation
============

This crate works with Cargo and can be found on
[crates.io] with a `Cargo.toml` like:

```toml
[dependencies]
serde_fix = "0.1"
```

The documentation is available on [docs.rs].

[crates.io]: https://crates.io/crates/serde_fix
[docs.rs]: https://docs.rs/serde_fix/0.1.0/serde_fix/

## Getting help

Serde developers live in the #serde channel on
[`irc.mozilla.org`](https://wiki.mozilla.org/IRC).
The #rust channel is also a good resource with generally
faster response time but less specific knowledge about Serde or this
crate. If IRC is not your thing, we are happy to respond to [GitHub
issues](https://github.com/nappa85/serde_fix/issues/new) as well.

## License

serde_fix is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in serde_fix by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

## Influences

This crate is heavily influenced by [serde_urlencoded](https://github.com/nox/serde_urlencoded/).
