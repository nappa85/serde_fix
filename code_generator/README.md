# FiX Code generator

This is a really stupid code generator, it has been forged against [fixopaedia](https://btobits.com/fixopaedia/) so probably it won't work on other sites.

It only generates main struct and needed enums, mapping data types.<br/>
It still needs some manual intervention fix duplicated enum items and something else...<br/>
Enums can be duplicated, use `enum_checker` tool to find duplications.

## Basic Usage
```bash
cargo run --release -- [-o <file>] <url> 
```
Without the optional `-o <file>` argument it will write Rust code directly in stdout, you can use output redirection to put it in a file.

E.g.:
```bash
cargo run --release https://btobits.com/fixopaedia/fixdic50-sp2-ep/message_ExecutionReport_8_.html > ../src/entities/fix50sp2/messages/execution_report.rs
```

## Advanced Usage
```bash
cargo run --release -- -r -o <folder> [-c <n>] <url>
```
It will take as `<url>` a Table Of Contents page, outputting in `<folder>` an rs file per block type, and in `<folder>/messages` an rs file per message type, generating also both `mod.rs` files.<br/>
With the optional `-c <n>` argument you can control how many futures are run at time, defaulting to 10.

E.g.:
```bash
cargo run --release -- -r -o ../src/entities/fix50sp2/ -c 100 https://btobits.com/fixopaedia/fixdic50-sp2-ep/toc.html
```
