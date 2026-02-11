# Using Structs to Structure Related Data

Different ways to define:
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
```


https://doc.rust-lang.org/book/ch05-01-defining-structs.html#defining-unit-like-structs
No field??
```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```
  -> Later in chapter 10
  


https://doc.rust-lang.org/book/ch05-01-defining-structs.html#ownership-of-struct-data

This, not This:

This:
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

Not This:
```rust
struct User {
    active: bool,
    username: &str,           // this will require lifetime (chapter 10)
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}
```
Read the section(link above).

Read about 
- print!("{rect}");
- print!("{rect:?}");    // adding `#[derive(Debug)]` traits
- dbg! macro

When we have larger structs, it’s useful to have output that’s a bit easier to read; 
in those cases, we can use `{:#?}` instead of `{:?}` in the println! string. 


