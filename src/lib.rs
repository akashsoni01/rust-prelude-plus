//! # Rust Prelude Plus
//!
//! A library that implements higher-order functions similar to functional programming patterns
//! (map, filter, fold, etc.) but built on top of the `key-paths-core` and `key-paths-derive` crates.
//! The library provides type-safe, composable operations on nested data structures.
//!
//! ## Features
//!
//! - **Type-safe keypath operations**: All operations maintain Rust's compile-time guarantees
//! - **Composable functions**: Chain operations together for complex transformations
//! - **Collection operations**: Extended methods for working with collections and keypaths
//! - **Error handling**: Proper error handling for invalid keypath access
//! - **Performance**: Minimal overhead with zero-cost abstractions where possible
//! - **Async support**: Optional async/await support for I/O operations
//! - **Parallel processing**: Optional parallel processing for large collections
//!
//! ## Quick Start
//!
//! ```rust
//! use rust_prelude_plus::prelude::*;
//! use key_paths_derive::Keypaths;
//!
//! #[derive(Keypaths, Debug, Clone)]
//! struct Person {
//!     name: String,
//!     age: u32,
//! }
//!
//! let people = vec![
//!     Person { name: "Alice".to_string(), age: 30 },
//!     Person { name: "Bob".to_string(), age: 25 },
//! ];
//!
//! // Filter people by age and extract their names
//! let young_people = filter_by_keypath(people, Person::age_r(), |&age| age < 30).unwrap();
//! let young_people_names: Vec<String> = map_keypath_collection(&young_people, Person::name_r(), |name| name.clone()).unwrap();
//!
//! println!("Young people: {:?}", young_people_names);
//! ```

pub mod error;
pub mod higher_order;

/// Re-exports for convenient usage
pub mod prelude {
    pub use crate::error::*;
    pub use crate::higher_order::*;
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");