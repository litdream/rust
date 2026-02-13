# 6.1.  Defining an Enum


Final summary:  Option Enum

```rust
enum Option<T> {
    None,
    Some(T),
}

let x: i8 = 5;
let y: Option<i8> = Some(5);

// err:
// let sum = x + y;

// maybe like this?
match y {
	Option::Some(y) => {
		let sum = x + y;
	}
	...<the-rest>...
}
```

# 6.2.

Chapter 19, will cover all about "pattern" and "match"



# 6.3 Concise Control Flow with if let and let...else

