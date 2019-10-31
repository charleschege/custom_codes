# custom_codes
This are Custom `Enums` for memorable and uniform response codes.

Enums are *cheaper* to *compare* and *harder* to get *wrong* than strings thereby guaranteeing efficiency.

### Examples

#### Create codes for File Operations

```rust
use custom_codes::FileOps; 

fn create_file(file_name: &str) -> FileOps {
    match std::fs::File::create(file_name) {
        Ok(_) => FileOps::CreateTrue,
        Err(_) => FileOps::CreateFalse,
	}
}

fn main() {
	open("foo.txt");
}
```
