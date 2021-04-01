# FiX Code generator

This is a really stupid code generator, it has been forged against [fixopaedia](https://btobits.com/fixopaedia/) so probably it won't work on other sites.

It only generates main struct and needed enums, mapping data types.<br/>
It still needs manual intervention to add uses.<br/>
Enums can be duplicated, needs another tool to find duplications.

## Usage
```bash
cargo run <url>
```
It will write Rust code directly in stdout, you can use output redirection to put it in a file.

E.g.:
```bash
cargo run https://btobits.com/fixopaedia/fixdic50-sp2-ep/message_ExecutionReport_8_.html > ../src/entities/fix50sp2/messages/execution_report.rs
```
