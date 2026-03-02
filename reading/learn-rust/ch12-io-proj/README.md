# Ch12.  IO project


## cargo new minigrep


some collection of documents

### args to Vector

[std::env](https://doc.rust-lang.org/std/env/index.html)
  - [env::args() -> Args](https://doc.rust-lang.org/std/env/fn.args.html)
    - struct Args {...}
  - [env::args().collect() -> <B>](https://doc.rust-lang.org/std/env/struct.Args.html#method.collect)
    - https://doc.rust-lang.org/src/core/iter/traits/iterator.rs.html#2015-2017


^This kind of analysis helps to understand Rust convention.

Finished reading Ch12.
Make [my own minigrep](minigrep)


### File handling

Reading file line-by-line isn't straightforward as other languages do.


1. File open:

std::fs::File::open call

[std::fs::File](https://doc.rust-lang.org/std/fs/struct.File.html)

open() call (std::fs::File::open)
```
impl File
pub fn open<P: AsRef<Path>>(path: P) -> Result<File>
```
- Path is own "type"???
  - std::path, Struct Path
    - `pub struct Path { /* private fields */ }`


There is a convenient open (buffered reader)
```
pub fn open_buffered<P: AsRef<Path>>(path: P) -> \
   Result<BufReader<File>>
```


2. BufReader:

std::io::BufReader

[std::io::BufReader](https://doc.rust-lang.org/std/io/struct.BufReader.html)

new() call
```
impl<R: Read> BufReader<R>

pub fn new(inner: R) -> BufReader<R>

```
- Question:   Why are we bounding <R: Read>??
  - [Read](https://doc.rust-lang.org/std/io/trait.Read.html)
    - std::io   Trait Read:  `pub trait Read {... 13 methods ...}`



lines() call
```
fn lines(self) -> Lines<Self>
```
- wait...  What is "Lines" type, then?
  - [Lines](https://doc.rust-lang.org/std/io/struct.Lines.html)
    - std::io   Struct Lines:  `pub struct Lines<B> { .. private fields .. }`
	- An iterator over the lines of an instance of BufRead.
	  - yields `io::Result<String>`
      - For this:  I can put "Lines" under "for .. in lines()" loop


Then, String.contains() call

[std::string::String](https://doc.rust-lang.org/std/string/struct.String.html)

contains() call
```
pub fn contains<P>(&self, pat: P) -> bool
where
    P: Pattern,
```
- [Pattern](https://doc.rust-lang.org/std/str/pattern/trait.Pattern.html)
  - std::str::pattern:   Trait Pattern:
