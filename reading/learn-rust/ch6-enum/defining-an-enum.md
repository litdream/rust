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

We can use `match` or `if let`.
```rust
let config_max = Some(3u8);
match config_max {
	Some(max) => println!("The maximum is configured to be {max}"),
	_ => (),
}
```

Same as:
```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
	println!("The maximum is configured to be {max}");
}
```

`if let ... else` is also possible.

