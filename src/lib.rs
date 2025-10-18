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
//! - **Memory efficient**: Uses `Rc` and `Arc` to avoid unnecessary cloning
//! - **Lazy evaluation**: Iterator-based operations for memory efficiency
//!
//! ## Quick Start
//!
//! ```rust
//! use rust_prelude_plus::prelude::*;
//! use key_paths_derive::Keypath;
//! use std::rc::Rc;
//!
//! #[derive(Keypath, Debug, Clone)]
//! struct Person {
//!     name: String,
//!     age: u32,
//! }
//!
//! let people = vec![
//!     Rc::new(Person { name: "Alice".to_string(), age: 30 }),
//!     Rc::new(Person { name: "Bob".to_string(), age: 25 }),
//! ];
//!
//! // Filter people by age and extract their names (lazy evaluation)
//! let young_people_names: Vec<String> = people
//!     .iter()
//!     .filter_by_keypath(Person::age(), |&age| age < 30)
//!     .map_keypath(Person::name(), |name| name.clone())
//!     .collect();
//!
//! println!("Young people: {:?}", young_people_names);
//! ```
//!
//! ## Advanced Usage
//!
//! ### Lazy Evaluation with Iterators
//!
//! ```rust
//! use rust_prelude_plus::prelude::*;
//! use key_paths_derive::Keypath;
//! use std::rc::Rc;
//!
//! #[derive(Keypath, Debug, Clone)]
//! struct Product {
//!     name: String,
//!     price: f64,
//!     category: String,
//! }
//!
//! let products = vec![
//!     Rc::new(Product { name: "Laptop".to_string(), price: 999.99, category: "Electronics".to_string() }),
//!     Rc::new(Product { name: "Book".to_string(), price: 19.99, category: "Books".to_string() }),
//! ];
//!
//! // Lazy evaluation - no intermediate collections created
//! let expensive_electronics: Vec<String> = products
//!     .iter()
//!     .filter_by_keypath(Product::category(), |cat| cat == "Electronics")
//!     .filter_by_keypath(Product::price(), |&price| price > 100.0)
//!     .map_keypath(Product::name(), |name| name.clone())
//!     .collect();
//! ```
//!
//! ### Memory Efficient Operations
//!
//! ```rust
//! use rust_prelude_plus::prelude::*;
//! use key_paths_derive::Keypath;
//! use std::rc::Rc;
//!
//! #[derive(Keypath, Debug, Clone)]
//! struct User {
//!     id: u32,
//!     name: String,
//!     email: String,
//! }
//!
//! let users = vec![
//!     Rc::new(User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() }),
//!     Rc::new(User { id: 2, name: "Bob".to_string(), email: "bob@example.com".to_string() }),
//! ];
//!
//! // Memory efficient - uses Rc to avoid cloning
//! let user_emails: Vec<String> = users
//!     .iter()
//!     .map_keypath(User::email(), |email| email.clone())
//!     .collect();
//! ```

pub mod error;
pub mod higher_order;
pub mod traits;
pub mod composable;
pub mod collections;
pub mod parallel;
pub mod async_ops;

/// Re-exports for convenient usage
pub mod prelude {
    pub use crate::error::*;
    pub use crate::higher_order::*;
    pub use crate::traits::*;
    pub use crate::composable::{pipe, chain_keypath_ops, when_keypath, unless_keypath, KeyPathsChain, ComposableIterator};
    pub use crate::collections::{KeyPathsCollectionExt, specialized};
    
    #[cfg(feature = "parallel")]
    pub use crate::parallel::*;
    
    #[cfg(feature = "async")]
    pub use crate::async_ops::*;
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");