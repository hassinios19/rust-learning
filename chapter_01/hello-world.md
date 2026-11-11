# 1.1 Hello World

**Rust Program Basics**
```rust
fn main(){
    println!("Hello, world!");
}
```
**Anatomy of a Rust program**

`main`: Always the first code that runs in an executable Rust program. In the Hello World statement there are no parameters and nothing gets return. Functions are wrapped in {}. Rust requires these around all function bodies.

`println!`: calls a Rust macro. If called a function--> without the !.