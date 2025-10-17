//! Composable operations for keypath functions

use key_paths_core::KeyPathss;
use crate::error::{KeyPathsResult, KeyPathsError};

/// Function composition for keypath operations
/// 
/// # Examples
/// 
/// ```rust
/// use rust_prelude_plus::prelude::*;
/// use key_paths_derive::Keypaths;
/// 
/// #[derive(Keypaths, Debug, Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
/// 
/// let people = vec![
///     Person { name: "Alice".to_string(), age: 30 },
///     Person { name: "Bob".to_string(), age: 25 },
/// ];
/// 
/// // Compose multiple operations
/// let result: Vec<String> = people
///     .into_iter()
///     .pipe(|iter| iter.filter_by_keypath(Person::age_r(), |&age| age < 30))
///     .pipe(|iter| iter.map_keypath(Person::name_r(), |name| name.to_uppercase()))
///     .collect();
/// 
/// assert_eq!(result, vec!["BOB"]);
/// ```
pub fn pipe<T, F, R>(value: T, f: F) -> R
where
    F: FnOnce(T) -> R,
{
    f(value)
}

/// Chain multiple keypath transformations
/// 
/// # Examples
/// 
/// ```rust
/// use rust_prelude_plus::prelude::*;
/// use key_paths_derive::Keypaths;
/// 
/// #[derive(Keypaths, Debug, Clone)]
/// struct Person {
///     name: String,
///     age: u32,
///     address: Address,
/// }
/// 
/// #[derive(Keypaths, Debug, Clone)]
/// struct Address {
///     city: String,
///     country: String,
/// }
/// 
/// let people = vec![
///     Person {
///         name: "Alice".to_string(),
///         age: 30,
///         address: Address { city: "New York".to_string(), country: "USA".to_string() },
///     },
/// ];
/// 
/// // Chain multiple transformations
/// let cities: Vec<String> = people
///     .into_iter()
///     .chain_keypath_ops()
///     .filter_by_keypath(Person::age_r(), |&age| age >= 30)
///     .map_keypath(Person::address_r().then(Address::city_r()), |city| city.clone())
///     .collect();
/// 
/// assert_eq!(cities, vec!["New York"]);
/// ```
pub fn chain_keypath_ops<T>(collection: Vec<T>) -> KeyPathsChain<T> {
    KeyPathsChain::new(collection)
}

/// Conditional keypath operations
/// 
/// # Examples
/// 
/// ```rust
/// use rust_prelude_plus::prelude::*;
/// use key_paths_derive::Keypaths;
/// 
/// #[derive(Keypaths, Debug, Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
/// 
/// let people = vec![
///     Person { name: "Alice".to_string(), age: 30 },
///     Person { name: "Bob".to_string(), age: 25 },
/// ];
/// 
/// // Apply operation only when condition is met
/// let result: Vec<String> = people
///     .into_iter()
///     .when_keypath(Person::age_r(), |&age| age >= 30, |iter| {
///         iter.map_keypath(Person::name_r(), |name| name.to_uppercase())
///     })
///     .collect();
/// 
/// assert_eq!(result, vec!["ALICE", "Bob"]);
/// ```
pub fn when_keypath<T, V, F, G, R>(
    collection: Vec<T>,
    keypath: impl KeyPaths<T, V>,
    condition: F,
    operation: G,
) -> KeyPathsResult<Vec<R>>
where
    F: Fn(&V) -> bool,
    G: FnOnce(std::vec::IntoIter<T>) -> std::vec::IntoIter<R>,
{
    let mut result = Vec::new();
    let mut iter = collection.into_iter();
    
    while let Some(item) = iter.next() {
        let value = keypath.get(&item);
        if condition(value) {
            // Apply operation to remaining items
            let mut remaining = std::iter::once(item).chain(iter).collect::<Vec<_>>();
            let transformed = operation(remaining.into_iter());
            result.extend(transformed);
            break;
        } else {
            // Keep original item
            result.push(item.into());
        }
    }
    
    Ok(result)
}

/// Inverse conditional operations
/// 
/// # Examples
/// 
/// ```rust
/// use rust_prelude_plus::prelude::*;
/// use key_paths_derive::Keypaths;
/// 
/// #[derive(Keypaths, Debug, Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
/// 
/// let people = vec![
///     Person { name: "Alice".to_string(), age: 30 },
///     Person { name: "Bob".to_string(), age: 25 },
/// ];
/// 
/// // Apply operation only when condition is NOT met
/// let result: Vec<String> = people
///     .into_iter()
///     .unless_keypath(Person::age_r(), |&age| age >= 30, |iter| {
///         iter.map_keypath(Person::name_r(), |name| name.to_uppercase())
///     })
///     .collect();
/// 
/// assert_eq!(result, vec!["Alice", "BOB"]);
/// ```
pub fn unless_keypath<T, V, F, G, R>(
    collection: Vec<T>,
    keypath: impl KeyPaths<T, V>,
    condition: F,
    operation: G,
) -> KeyPathsResult<Vec<R>>
where
    F: Fn(&V) -> bool,
    G: FnOnce(std::vec::IntoIter<T>) -> std::vec::IntoIter<R>,
{
    when_keypath(collection, keypath, |v| !condition(v), operation)
}

/// KeyPaths chain for composable operations
pub struct KeyPathsChain<T> {
    collection: Vec<T>,
}

impl<T> KeyPathsChain<T> {
    fn new(collection: Vec<T>) -> Self {
        Self { collection }
    }
    
    /// Filter by keypath predicate
    pub fn filter_by_keypath<V, F>(self, keypath: impl KeyPaths<T, V>, predicate: F) -> Self
    where
        F: Fn(&V) -> bool,
    {
        let filtered: Vec<T> = self.collection
            .into_iter()
            .filter(|item| {
                let value = keypath.get(item);
                predicate(value)
            })
            .collect();
        Self::new(filtered)
    }
    
    /// Map over keypath values
    pub fn map_keypath<V, F, R>(self, keypath: impl KeyPaths<T, V>, f: F) -> KeyPathsChain<R>
    where
        F: Fn(&V) -> R,
    {
        let mapped: Vec<R> = self.collection
            .into_iter()
            .map(|item| {
                let value = keypath.get(&item);
                f(value)
            })
            .collect();
        KeyPathsChain::new(mapped)
    }
    
    /// Fold over keypath values
    pub fn fold_keypath<V, F, B>(self, keypath: impl KeyPaths<T, V>, init: B, f: F) -> KeyPathsResult<B>
    where
        F: Fn(B, &V) -> B,
    {
        let mut acc = init;
        for item in self.collection {
            let value = keypath.get(&item);
            acc = f(acc, value);
        }
        Ok(acc)
    }
    
    /// Collect into a vector
    pub fn collect<B: FromIterator<T>>(self) -> B {
        self.collection.into_iter().collect()
    }
    
    /// Take first n elements
    pub fn take(self, n: usize) -> Self {
        let taken: Vec<T> = self.collection.into_iter().take(n).collect();
        Self::new(taken)
    }
    
    /// Skip first n elements
    pub fn skip(self, n: usize) -> Self {
        let skipped: Vec<T> = self.collection.into_iter().skip(n).collect();
        Self::new(skipped)
    }
    
    /// Reverse the collection
    pub fn rev(self) -> Self {
        let mut reversed = self.collection;
        reversed.reverse();
        Self::new(reversed)
    }
}

/// Extension trait for adding composable operations to iterators
pub trait ComposableIterator<T>: Iterator<Item = T> {
    /// Pipe the iterator through a function
    fn pipe<F, R>(self, f: F) -> R
    where
        Self: Sized,
        F: FnOnce(Self) -> R,
    {
        f(self)
    }
    
    /// Chain keypath operations
    fn chain_keypath_ops(self) -> KeyPathsChain<T>
    where
        Self: Sized,
    {
        KeyPathsChain::new(self.collect())
    }
    
    /// Apply operation when condition is met
    fn when_keypath<V, F, G, R>(
        self,
        keypath: impl KeyPaths<T, V>,
        condition: F,
        operation: G,
    ) -> KeyPathsResult<Vec<R>>
    where
        Self: Sized,
        F: Fn(&V) -> bool,
        G: FnOnce(std::vec::IntoIter<T>) -> std::vec::IntoIter<R>,
    {
        when_keypath(self.collect(), keypath, condition, operation)
    }
    
    /// Apply operation unless condition is met
    fn unless_keypath<V, F, G, R>(
        self,
        keypath: impl KeyPaths<T, V>,
        condition: F,
        operation: G,
    ) -> KeyPathsResult<Vec<R>>
    where
        Self: Sized,
        F: Fn(&V) -> bool,
        G: FnOnce(std::vec::IntoIter<T>) -> std::vec::IntoIter<R>,
    {
        unless_keypath(self.collect(), keypath, condition, operation)
    }
}

// Implement ComposableIterator for all iterators
impl<I: Iterator> ComposableIterator<I::Item> for I {}

/// Macro for creating keypath operation pipelines
#[macro_export]
macro_rules! keypath_pipeline {
    ($collection:expr => $($op:tt)*) => {
        {
            let mut result = $collection;
            $(
                result = keypath_pipeline_op!(result, $op);
            )*
            result
        }
    };
}

#[macro_export]
macro_rules! keypath_pipeline_op {
    ($collection:expr, filter_by_keypath($keypath:expr, $predicate:expr)) => {
        $collection.into_iter().filter_by_keypath($keypath, $predicate).collect()
    };
    ($collection:expr, map_keypath($keypath:expr, $transform:expr)) => {
        $collection.into_iter().map_keypath($keypath, $transform).collect()
    };
    ($collection:expr, take($n:expr)) => {
        $collection.into_iter().take($n).collect()
    };
    ($collection:expr, skip($n:expr)) => {
        $collection.into_iter().skip($n).collect()
    };
}

/// Utility functions for common keypath operations
pub mod utils {
    use super::*;
    
    /// Create a keypath operation that can be reused
    pub fn create_keypath_operation<T, V, F, R>(
        keypath: impl KeyPaths<T, V>,
        operation: F,
    ) -> impl Fn(T) -> KeyPathsResult<R>
    where
        F: Fn(&V) -> R,
    {
        move |item| {
            let value = keypath.get(&item);
            Ok(operation(value))
        }
    }
    
    /// Create a keypath predicate that can be reused
    pub fn create_keypath_predicate<T, V, F>(
        keypath: impl KeyPaths<T, V>,
        predicate: F,
    ) -> impl Fn(&T) -> bool
    where
        F: Fn(&V) -> bool,
    {
        move |item| {
            let value = keypath.get(item);
            predicate(value)
        }
    }
    
    /// Combine multiple keypath operations
    pub fn combine_keypath_operations<T, V1, V2, F1, F2, R1, R2>(
        keypath1: impl KeyPaths<T, V1>,
        operation1: F1,
        keypath2: impl KeyPaths<T, V2>,
        operation2: F2,
    ) -> impl Fn(T) -> KeyPathsResult<(R1, R2)>
    where
        F1: Fn(&V1) -> R1,
        F2: Fn(&V2) -> R2,
    {
        move |item| {
            let value1 = keypath1.get(&item);
            let value2 = keypath2.get(&item);
            Ok((operation1(value1), operation2(value2)))
        }
    }
}
