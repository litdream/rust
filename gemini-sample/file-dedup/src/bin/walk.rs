// Using Rust, how can I walk the directory?
//

// WalkDir document:  https://docs.rs/walkdir/latest/walkdir/#example
//   - new():         pub fn new<P: AsRef<Path>>(root: P) -> Self
//   - into_iter():
//     - impl IntoIterator for WalkDir
//       - fn into_iter(self) -> IntoIter
//   - map():         https://docs.rs/walkdir/latest/walkdir/struct.IntoIter.html#method.map
//     - fn map<B, F>(self, f: F) -> Map<Self, F>
//

use walkdir::WalkDir;
use std::env;
use std::path::Path;

fn main() {
    // 1. Collect arguments
    let args: Vec<String> = env::args().collect();
    
    // 2. Validate input
    if args.len() < 2 {
        eprintln!("Usage: cargo run --bin walk <directory_path>");
        std::process::exit(1);
    }

    let target_dir = &args[1];

    // 3. Walk the directory
    // We use filter_map to ignore entries we don't have permission to read
    for entry in WalkDir::new(target_dir)
        .into_iter()
        .filter_map(|e| e.ok()) 
    {
        // 4. Convert to absolute path
        let path = entry.path();
        
        match path.canonicalize() {
            Ok(abs_path) => println!("{}", abs_path.display()),
            Err(_) => {
                // If canonicalize fails (e.g., broken symlink), 
                // we still print the relative path provided
                println!("{}", path.display());
            }
        }
    }
}
