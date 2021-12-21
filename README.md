# FiX meets Serde

This workspace is a set of Rust libraries for serialising to and deserialising from
the [FiX] format. It is built
upon [Serde], a high performance generic serialization framework.

[Serde]: https://github.com/serde-rs/serde
[FiX]: http://www.fixprotocol.org/

## Warning

This workspace makes a really heavy use of proc-macros, building is really CPU and RAM intensive and can take a lot of time, at point of being killed by kernel OOM Killer.

## Composition

This workspace is composed of:
* [code_generator](./code_generator): utility to generate code from protocol specs
* [enum_checker](./enum_checker): utility to find duplicate enums across code
* [fix40](./fix40): FiX 4.0 entities
* [fix41](./fix41): FiX 4.1 entities
* [fix42](./fix42): FiX 4.2 entities
* [fix43](./fix43): FiX 4.3 entities
* [fix44](./fix44): FiX 4.4 entities
* [fix50](./fix50): FiX 5.0 entities
* [fix50sp1](./fix50sp1): FiX 5.0SP1 entities
* [fix50sp2](./fix50sp2): FiX 5.0SP2 entities
* [fixt11](./fixt11): FiX Transport 1.1
* [fix_common](./fix_common): common FiX entities, data type and structures
* [fix_message](./fix_message): Modular multiversion message enum
* [fix_message_viewer](./fix_message_viewer): Example application
* [serde_fix](./serde_fix): Serialization/Deserialization methods

## Example usage

In your Cargo.toml simply insert `fix_message` for data structures and `serde_fix` for serialization/deserialization APIs:
```toml
[dependencies]
fix_message = "*"
serde_fix = "*"
```

If you need only certain FiX protocol versions, to speed up build times, you can use `fix_messages` features:
```toml
[dependencies]
fix_message = { version = "*", deafult-featues = false, features = ["fix_50", "fix_50sp1", "fix_50sp2"] }
serde_fix = "*"
```

To deserialize a FiX message you can simply do:
```rust
let message: fix_message::Message = serde_fix::from_bytes(&buf).unwrap();
```

`fix_message::Message` is an enum, every variant represents a different FiX version.
Inside every variant there is another enum, e.g. `fix_message::fix40::Message`, that represents every message type for that version.
For example, a FiX 4.0 Logon message would decode like `fix_message::Message::FIX40(fix_message::fix40::Message::Logon(Box<fix_message::fix40::messages::logon::Logon>))`.
