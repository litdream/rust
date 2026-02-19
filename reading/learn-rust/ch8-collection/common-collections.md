# 8.1 Storing Lists of Values with Vectors


Creating a new vector
```rust
let v: Vec<i32> = Vec::new();
            ^
			|
     // Need type annotation.
	 //   (we want to store i32).
	 // Maybe we want `mut`, mostly list requires changes.


// OR, implicitly,
//
let v = vec![1, 2, 3];
```


## reference mix.

main.rs:  err_why() and err_fix()


## Using an Enum to Store Multiple Types

enum trick, to use multiple types in a list.



# 8.2 Storing UTF-8 Encoded Text with Strings


# 8.3 Storing Keys with Associated Values in Hash Maps

HashMap<K, V>

This is useful to get **inside** value, not borrowed value.
```rust
use std::collections::HashMap;
let mut scores = HashMap::new();

scores.insert(....)

let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
//                                 ^         ^
//                                 |         Option<V> is now V, or if value is None, it's **0**.
//                                 |
//                                 + Because get() returns Option<&V>, copied()
//                                   makes to Option<V>.
//
```

Be careful about ownership: 
  - Copy trait:  e.g. i32:   will be happily copied into HashMap
  - But, owned:  e.g. String: will be owned, and old variables no longer valid.
  

Updating:
Because map returns Option<&V>, direct update should be done by "*" op.
```rust
let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;    // count is "borrwed" (or pointer), due to Option<&V>
}
println!("{map:?}");
```

