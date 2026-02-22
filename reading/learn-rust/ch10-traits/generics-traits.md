# Generics

Similar to C++, but compiletime.


## Traits

Very similar to Interface.

```rust
pub trait Summary {
	fn summarize(&self) -> String;
}

pub struct NewsArticle {
  ...
}
impl Summary for NewsArticle {
	fn summarize(&self) -> String {
		format!("{} {}", self.headline, self.author)
	}
}

pub struct SocialPost {
  ...
}
impl Summary for SocialPost {
	fn summarize(&self) -> String {
		format!("{} :: {}", self.username, self.content)
	}
}
```


In this sense, it's like, abstract class.
```rust
pub trait Summary {
  fn summary_author(&self) -> String;
  
  fn summarize(&self) -> String {
    println!("Default impl:  author is {}", self.summary_author())
  }  

}
```

When passing parameter type, unlike other language, Rust has to tell "impl" type.
```rust
pub fn notify(item: &impl Summary) {
  println("Breaking news! {}", item.summarize())
}

// But, look at this.  This generics work like impl.  In fact, it is!
//    - Trait Bound
//      - <T: Summary>,  instead of <T>
pub fn notify<T: Summary>(item: &T) {
  println("Breaking news! {}", item.summarize())
}
```



