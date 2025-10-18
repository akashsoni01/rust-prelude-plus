//! Core traits for keypath operations
//!
//! This module provides extension traits that enable keypath operations on various types.
//! It includes iterator extensions, collection extensions, and memory-efficient operations
//! using `Rc` and `Arc`.
//!
//! ## Examples
//!
//! ### Basic KeyPath Operations
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
//! let person = Rc::new(Person { name: "Alice".to_string(), age: 30 });
//! let name = person.get_at_keypath(Person::name()).unwrap();
//! assert_eq!(name, "Alice");
//! ```
//!
//! ### Iterator Extensions
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
//! }
//!
//! let products = vec![
//!     Rc::new(Product { name: "Laptop".to_string(), price: 999.99 }),
//!     Rc::new(Product { name: "Book".to_string(), price: 19.99 }),
//! ];
//!
//! // Lazy evaluation with iterator extensions
//! let expensive_products: Vec<Rc<Product>> = products
//!     .iter()
//!     .filter_by_keypath(Product::price(), |&price| price > 100.0)
//!     .map(|item| item.clone())
//!     .collect();
//! ```

use key_paths_core::KeyPaths;
use crate::error::{KeyPathResult, KeyPathError};

/// Trait for types that can be operated on with keypaths
pub trait KeyPathsOperable: Sized {
    /// Get a value at a keypath
    fn get_at_keypath<'a, V>(&'a self, keypath: &'a KeyPaths<Self, V>) -> KeyPathResult<&'a V> {
        keypath.get(self).ok_or_else(|| KeyPathError::InvalidAccess { 
            message: "KeyPath access failed".to_string() 
        })
    }
    
    /// Set a value at a keypath (if the keypath supports mutation)
    fn set_at_keypath<V>(&mut self, _keypath: KeyPaths<Self, V>, _value: V) -> KeyPathResult<()> {
        // Note: This is a simplified implementation
        // In practice, you'd need to handle the specific keypath type
        Err(KeyPathError::InvalidAccess { 
            message: "KeyPath mutation not supported in this context".to_string() 
        })
    }
}

/// Trait for iterators that support keypath operations
pub trait KeyPathsIterator: Iterator {
    /// Map over a keypath in the iterator
    fn map_keypath<V, F, R>(self, keypath: KeyPaths<Self::Item, V>, f: F) -> Vec<R>
    where
        Self: Sized,
        Self::Item: KeyPathsOperable,
        F: Fn(&V) -> R,
    {
        self.map(|item| {
            let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                panic!("KeyPath access failed in map_keypath")
            });
            f(value)
        }).collect()
    }
    
    /// Filter by a keypath predicate
    fn filter_by_keypath<V, F>(self, keypath: KeyPaths<Self::Item, V>, predicate: F) -> Vec<Self::Item>
    where
        Self: Sized,
        Self::Item: KeyPathsOperable,
        F: Fn(&V) -> bool,
    {
        self.filter(|item| {
            let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                panic!("KeyPath access failed in filter_by_keypath")
            });
            predicate(value)
        }).collect()
    }
    
    /// Find an element by keypath predicate
    fn find_by_keypath<V, F>(self, keypath: KeyPaths<Self::Item, V>, predicate: F) -> KeyPathResult<Option<Self::Item>>
    where
        Self: Sized,
        Self::Item: KeyPathsOperable,
        F: Fn(&V) -> bool,
    {
        for item in self {
            if let Ok(value) = item.get_at_keypath(&keypath) {
                if predicate(value) {
                    return Ok(Some(item));
                }
            }
        }
        Ok(None)
    }
    
    /// Fold over a keypath
    fn fold_keypath<V, F, B>(self, keypath: KeyPaths<Self::Item, V>, init: B, mut f: F) -> KeyPathResult<B>
    where
        Self: Sized,
        Self::Item: KeyPathsOperable,
        F: FnMut(B, &V) -> B,
    {
        let mut acc = init;
        for item in self {
            if let Ok(value) = item.get_at_keypath(&keypath) {
                acc = f(acc, value);
            }
        }
        Ok(acc)
    }
    
    /// Collect values from a keypath
    fn collect_keypath<V>(self, keypath: KeyPaths<Self::Item, V>) -> KeyPathResult<Vec<V>>
    where
        Self: Sized,
        Self::Item: KeyPathsOperable,
        V: Clone,
    {
        let mut result = Vec::new();
        for item in self {
            if let Ok(value) = item.get_at_keypath(&keypath) {
                result.push(value.clone());
            }
        }
        Ok(result)
    }
}

/// Trait for collections that support keypath operations
pub trait KeyPathsCollection<T> {
    /// Group elements by keypath values
    fn group_by_keypath<V, F>(&self, keypath: KeyPaths<T, V>, f: F) -> KeyPathResult<std::collections::HashMap<V, Vec<T>>>
    where
        V: std::hash::Hash + Eq + Clone,
        T: Clone + KeyPathsOperable,
        F: Fn(&V) -> V;
    
    /// Partition elements by keypath predicate
    fn partition_by_keypath<V, F>(&self, keypath: KeyPaths<T, V>, predicate: F) -> KeyPathResult<(Vec<T>, Vec<T>)>
    where
        T: Clone + KeyPathsOperable,
        F: Fn(&V) -> bool;
    
    /// Sort elements by keypath values
    fn sort_by_keypath<V, F>(&mut self, keypath: KeyPaths<T, V>, compare: F) -> KeyPathResult<()>
    where
        T: KeyPathsOperable,
        F: Fn(&V, &V) -> std::cmp::Ordering;
}

// Implement KeyPathsOperable for all types
impl<T> KeyPathsOperable for T {}

// Implement KeyPathsIterator for all iterators
impl<I: Iterator> KeyPathsIterator for I {}

// Implement KeyPathsCollection for Vec
impl<T> KeyPathsCollection<T> for Vec<T> {
    fn group_by_keypath<V, F>(&self, keypath: KeyPaths<T, V>, f: F) -> KeyPathResult<std::collections::HashMap<V, Vec<T>>>
    where
        V: std::hash::Hash + Eq + Clone,
        T: Clone + KeyPathsOperable,
        F: Fn(&V) -> V,
    {
        let mut groups = std::collections::HashMap::new();
        for item in self {
            if let Ok(value) = item.get_at_keypath(&keypath) {
                let key = f(value);
                groups.entry(key).or_insert_with(Vec::new).push(item.clone());
            }
        }
        Ok(groups)
    }
    
    fn partition_by_keypath<V, F>(&self, keypath: KeyPaths<T, V>, predicate: F) -> KeyPathResult<(Vec<T>, Vec<T>)>
    where
        T: Clone + KeyPathsOperable,
        F: Fn(&V) -> bool,
    {
        let mut left = Vec::new();
        let mut right = Vec::new();
        
        for item in self {
            if let Ok(value) = item.get_at_keypath(&keypath) {
                if predicate(value) {
                    left.push(item.clone());
                } else {
                    right.push(item.clone());
                }
            }
        }
        
        Ok((left, right))
    }
    
    fn sort_by_keypath<V, F>(&mut self, keypath: KeyPaths<T, V>, compare: F) -> KeyPathResult<()>
    where
        T: KeyPathsOperable,
        F: Fn(&V, &V) -> std::cmp::Ordering,
    {
        self.sort_by(|a, b| {
            let a_val = keypath.get(a).unwrap_or_else(|| {
                panic!("KeyPath access failed in sort")
            });
            let b_val = keypath.get(b).unwrap_or_else(|| {
                panic!("KeyPath access failed in sort")
            });
            compare(a_val, b_val)
        });
        Ok(())
    }
}
