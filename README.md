# FiX meets Serde

This workspace is a set of Rust libraries for serialising to and deserialising from
the [FiX] format. It is built
upon [Serde], a high performance generic serialization framework.

[Serde]: https://github.com/serde-rs/serde
[FiX]: http://www.fixprotocol.org/

## Composition

This workspace is composed of:
* [code_generator](./code_generator): utility to generate code from protocol specs
* [enum_checker](./enum_checker): utility to find duplicate enums across code
* [fix40](./fix40): FiX 4.0 entities
* [fix41](./fix41): FiX 4.1 entities
* [fix42](./fix42): FiX 4.2 entities
* [fix43](./fix43): FiX 4.3 entities
* [fix44](./fix44): FiX 4.4 entities
* [fix50](./fix45): FiX 5.0 entities
* [fix50sp1](./fix50sp1): FiX 5.0SP1 entities
* [fix50sp2](./fix50sp2): FiX 5.0SP2 entities
* [fix_common](./fix_common): common FiX entities, data type and structures
* [fix_message](./fix_message): Modular multiversion message enum
* [fixt11](./fixt11): FiX Transport 1.1
* [serde_fix](./serde_fix): Serialization/Deserialization methods
