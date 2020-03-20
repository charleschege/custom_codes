#![deny(missing_docs)]
#![deny(unsafe_code)]
//! This are Custom Enum codes for memorable and uniform response codes.
//! Enums are `cheaper` to `compare` and `harder` to get `wrong` than strings thereby guaranteeing efficiency.
//!
//! ## Examples
//!
//! ### Create codes for File Operations
//! ```no_run
//! use custom_codes::FileOps;
//!
//! fn open_file(file_name: &str) -> FileOps {
//!     match std::fs::File::create(file_name) {
//!         Ok(_) => FileOps::CreateTrue,
//!         Err(_) => FileOps::CreateFalse,
//!     }
//! }
//! fn main() {
//!     open_file("foo.txt");
//! }
//! ```

/// re-exports
pub use crate::custom_codes::*;

mod custom_codes;
