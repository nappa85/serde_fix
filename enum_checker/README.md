# FiX Enum Checker

This is a really stupid tool to check enum duplication in a project.

## Usage
```bash
cargo run <path>
```
It will write list of possible duplications in form of `file`::`enum` == `file`::`enum` followed by enum keys.const 

An enum key is taken considering the `#[serde(rename = "<value>")]` directive, if present.

E.g.:
```bash
cargo run ../src
Equal ../src/lib.rs::Foo == ../src/lib.rs::Bar (["0", "1", "2"])
```
