# Rust Prelude Plus

A comprehensive library that implements higher-order functions similar to functional programming patterns (map, filter, fold, etc.) but built on top of the `key-paths-core` and `key-paths-derive` crates. The library provides type-safe, composable operations on nested data structures.

## Features

- **Type-safe keypath operations**: All operations maintain Rust's compile-time guarantees
- **Composable functions**: Chain operations together for complex transformations
- **Collection operations**: Extended methods for working with collections and keypaths
- **Error handling**: Proper error handling for invalid keypath access
- **Performance**: Minimal overhead with zero-cost abstractions where possible
- **Async support**: Optional async/await support for I/O operations
- **Parallel processing**: Optional parallel processing for large collections

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-prelude-plus = "0.1.0"
key-paths-core = "1.0.2"
key-paths-derive = "0.6.0"
```

### Basic Usage

```rust
use rust_prelude_plus::prelude::*;
use key_paths_derive::Keypaths;

#[derive(Keypaths, Debug, Clone)]
struct Person {
    name: String,
    age: u32,
    address: Address,
}

#[derive(Keypaths, Debug, Clone)]
struct Address {
    city: String,
    country: String,
}

let people = vec![
    Person {
        name: "Alice".to_string(),
        age: 30,
        address: Address { city: "New York".to_string(), country: "USA".to_string() },
    },
    Person {
        name: "Bob".to_string(),
        age: 25,
        address: Address { city: "London".to_string(), country: "UK".to_string() },
    },
];

// Filter people by age and extract their names
let young_people_names: Vec<String> = people
    .into_iter()
    .filter_by_keypath(Person::age_r(), |&age| age < 30)
    .map_keypath(Person::name_r(), |name| name.clone())
    .collect();

println!("Young people: {:?}", young_people_names);
```

## Core Concepts

### KeyPaths

KeyPaths provide type-safe access to nested data structures. They're similar to Swift's KeyPath system but designed for Rust's ownership model.

### Higher-Order Functions

The library provides functional programming primitives that work with keypaths:

- `map_keypath`: Transform values at a specific keypath
- `filter_by_keypath`: Filter collections based on keypath values
- `fold_keypath`: Accumulate values from keypaths
- `find_by_keypath`: Find elements matching keypath conditions
- `group_by_keypath`: Group elements by keypath values
- `sort_by_keypath`: Sort collections by keypath values

### Composable Operations

Functions can be chained together for complex transformations:

- `pipe`: Function composition for keypath operations
- `chain`: Chain multiple keypath transformations
- `when`: Conditional keypath operations
- `unless`: Inverse conditional operations

## Examples

### Basic Operations

```rust
use rust_prelude_plus::prelude::*;
use key_paths_derive::Keypaths;

#[derive(Keypaths, Debug, Clone)]
struct Person {
    name: String,
    age: u32,
    skills: Vec<String>,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 30, skills: vec!["Rust".to_string()] },
    Person { name: "Bob".to_string(), age: 25, skills: vec!["Python".to_string()] },
];

// Filter by age
let young_people: Vec<Person> = people
    .into_iter()
    .filter_by_keypath(Person::age_r(), |&age| age < 30)
    .collect();

// Map over names
let names: Vec<String> = people
    .into_iter()
    .map_keypath(Person::name_r(), |name| name.to_uppercase())
    .collect();

// Find by condition
let found = people
    .into_iter()
    .find_by_keypath(Person::age_r(), |&age| age == 30)
    .unwrap();
```

### Advanced Operations

```rust
// Group by age range
let grouped: HashMap<String, Vec<Person>> = people
    .group_by_keypath(Person::age_r(), |&age| {
        if age < 30 { "young".to_string() } else { "adult".to_string() }
    })
    .unwrap();

// Sort by age
let mut sorted_people = people.clone();
sorted_people.sort_by_keypath(Person::age_r(), |a, b| a.cmp(b)).unwrap();

// Partition by condition
let (young, old): (Vec<Person>, Vec<Person>) = people
    .partition_by_keypath(Person::age_r(), |&age| age < 30)
    .unwrap();
```

### Composable Operations

```rust
// Using pipe for function composition
let result: Vec<String> = people
    .into_iter()
    .pipe(|iter| iter.filter_by_keypath(Person::age_r(), |&age| age < 30))
    .pipe(|iter| iter.map_keypath(Person::name_r(), |name| name.to_uppercase()))
    .collect();

// Using chain for complex operations
let result: Vec<String> = people
    .into_iter()
    .chain_keypath_ops()
    .filter_by_keypath(Person::age_r(), |&age| age >= 30)
    .map_keypath(Person::name_r(), |name| name.clone())
    .collect();
```

## Features

### Async Support

Enable the `async` feature for async operations:

```toml
[dependencies]
rust-prelude-plus = { version = "0.1.0", features = ["async"] }
```

```rust
#[cfg(feature = "async")]
use rust_prelude_plus::async_ops::*;

// Async keypath operations
let result: Vec<String> = async_collections::map_keypath_async(
    people,
    Person::name_r(),
    |name| name.clone()
).await.unwrap();
```

### Parallel Processing

Enable the `parallel` feature for parallel operations:

```toml
[dependencies]
rust-prelude-plus = { version = "0.1.0", features = ["parallel"] }
```

```rust
#[cfg(feature = "parallel")]
use rust_prelude_plus::parallel::*;

// Parallel keypath operations
let result: Vec<String> = parallel_collections::par_map_keypath(
    &people,
    Person::name_r(),
    |name| name.clone()
).unwrap();
```

### Serde Integration

Enable the `serde` feature for serialization support:

```toml
[dependencies]
rust-prelude-plus = { version = "0.1.0", features = ["serde"] }
```

## Performance

The library is designed for performance with minimal overhead:

- Zero-cost abstractions where possible
- Compile-time optimizations
- Optional parallel processing for large collections
- Efficient memory usage patterns

## Error Handling

All operations include proper error handling:

- Type-safe keypath access
- Clear error messages
- Graceful failure modes
- Integration with Rust's error handling ecosystem

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

- Inspired by Swift's KeyPath system
- Built on top of the excellent `key-paths-core` and `key-paths-derive` crates
- Functional programming patterns from various languages
