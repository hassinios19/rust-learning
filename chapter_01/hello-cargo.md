# Hello Cargo

Cargo is the Rust build system and package manager. Most Restaceans use this tool to manage their projects cause it handles a lot of tasks for you, like building your code, downloading libraries and building them.

`cargo --version` to check the version
`cargo new xyz` to add a cargo project.

Cargo.toml
```
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```
TOML--> Toms Obvious Minimal Language ( configuration format)

`[package]`: section heading that indicates that thee following statements are configuring a package.
Next lines--> configuration information for compiling. 