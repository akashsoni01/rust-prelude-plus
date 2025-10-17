//! Higher-order functions for keypath operations

use crate::error::{KeyPathResult, KeyPathError};
use key_paths_core::KeyPaths;
use std::collections::HashMap;

/// Transform values at a specific keypath
/// 
/// # Examples
/// 
/// ```rust
/// use rust_prelude_plus::prelude::*;
/// use key_paths_derive::Keypath;
/// 
/// #[derive(Keypath, Debug, Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
/// 
/// let person = Person { name: "Alice".to_string(), age: 30 };
/// let result = map_keypath(person, Person::name(), |name| name.to_uppercase()).unwrap();
/// assert_eq!(result, "ALICE");
/// ```
pub fn map_keypath<T, V, F, R>(
    data: T,
    keypath: KeyPaths<T, V>,
    f: F,
) -> KeyPathResult<R>
where
    F: FnOnce(&V) -> R,
{
    let value = keypath.get(&data).ok_or_else(|| KeyPathError::InvalidAccess { message: "KeyPath access failed".to_string() })?;
    Ok(f(value))
}

/// Transform values at a specific keypath for collections
pub fn map_keypath_collection<T, V, F, R>(
    collection: &[T],
    keypath: KeyPaths<T, V>,
    f: F,
) -> KeyPathResult<Vec<R>>
where
    F: Fn(&V) -> R,
{
    let mut result = Vec::new();
    for item in collection {
        let value = keypath.get(item).ok_or_else(|| KeyPathError::InvalidAccess { message: "KeyPath access failed".to_string() })?;
        result.push(f(value));
    }
    Ok(result)
}

/// Filter collections based on keypath values
/// 
/// # Examples
/// 
/// ```rust
/// use rust_prelude_plus::prelude::*;
/// use key_paths_derive::Keypath;
/// 
/// #[derive(Keypath, Debug, Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
/// 
/// let people = vec![
///     Person { name: "Alice".to_string(), age: 30 },
///     Person { name: "Bob".to_string(), age: 25 },
/// ];
/// let young_people = filter_by_keypath(people, Person::age(), |&age| age < 30).unwrap();
/// assert_eq!(young_people.len(), 1);
/// assert_eq!(young_people[0].name, "Bob");
/// ```
pub fn filter_by_keypath<T, V, F>(
    collection: Vec<T>,
    keypath: KeyPaths<T, V>,
    predicate: F,
) -> KeyPathResult<Vec<T>>
where
    F: Fn(&V) -> bool,
{
    let mut result = Vec::new();
    for item in collection {
        let value = keypath.get(&item).ok_or_else(|| KeyPathError::InvalidAccess { message: "KeyPath access failed".to_string() })?;
        if predicate(value) {
            result.push(item);
        }
    }
    Ok(result)
}

/// Accumulate values from keypaths
/// 
/// # Examples
/// 
/// ```rust
/// use rust_prelude_plus::prelude::*;
/// use key_paths_derive::Keypath;
/// 
/// #[derive(Keypath, Debug, Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
/// 
/// let people = vec![
///     Person { name: "Alice".to_string(), age: 30 },
///     Person { name: "Bob".to_string(), age: 25 },
/// ];
/// let total_age = fold_keypath(people, Person::age(), 0, |acc, &age| acc + age).unwrap();
/// assert_eq!(total_age, 55);
/// ```
pub fn fold_keypath<T, V, F, B>(
    collection: Vec<T>,
    keypath: KeyPaths<T, V>,
    init: B,
    f: F,
) -> KeyPathResult<B>
where
    F: Fn(B, &V) -> B,
{
    let mut acc = init;
    for item in collection {
        let value = keypath.get(&item).ok_or_else(|| KeyPathError::InvalidAccess { message: "KeyPath access failed".to_string() })?;
        acc = f(acc, value);
    }
    Ok(acc)
}

/// Find elements matching keypath conditions
/// 
/// # Examples
/// 
/// ```rust
/// use rust_prelude_plus::prelude::*;
/// use key_paths_derive::Keypath;
/// 
/// #[derive(Keypath, Debug, Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
/// 
/// let people = vec![
///     Person { name: "Alice".to_string(), age: 30 },
///     Person { name: "Bob".to_string(), age: 25 },
/// ];
/// let found = find_by_keypath(people, Person::age(), |&age| age == 30).unwrap();
/// assert!(found.is_some());
/// assert_eq!(found.unwrap().name, "Alice");
/// ```
pub fn find_by_keypath<T, V, F>(
    collection: Vec<T>,
    keypath: KeyPaths<T, V>,
    predicate: F,
) -> KeyPathResult<Option<T>>
where
    F: Fn(&V) -> bool,
{
    for item in collection {
        let value = keypath.get(&item).ok_or_else(|| KeyPathError::InvalidAccess { message: "KeyPath access failed".to_string() })?;
        if predicate(value) {
            return Ok(Some(item));
        }
    }
    Ok(None)
}

/// Group elements by keypath values
/// 
/// # Examples
/// 
/// ```rust
/// use rust_prelude_plus::prelude::*;
/// use key_paths_derive::Keypath;
/// use std::collections::HashMap;
/// 
/// #[derive(Keypath, Debug, Clone)]
/// struct Person {
///     name: String,
///     age: u32,
///     department: String,
/// }
/// 
/// let people = vec![
///     Person { name: "Alice".to_string(), age: 30, department: "Engineering".to_string() },
///     Person { name: "Bob".to_string(), age: 25, department: "Engineering".to_string() },
///     Person { name: "Charlie".to_string(), age: 35, department: "Marketing".to_string() },
/// ];
/// 
/// let grouped = group_by_keypath(&people, Person::department(), |dept| dept.clone()).unwrap();
/// assert_eq!(grouped.len(), 2);
/// assert_eq!(grouped["Engineering"].len(), 2);
/// assert_eq!(grouped["Marketing"].len(), 1);
/// ```
pub fn group_by_keypath<T, V, F, K>(
    collection: &[T],
    keypath: KeyPaths<T, V>,
    key_fn: F,
) -> KeyPathResult<HashMap<K, Vec<T>>>
where
    T: Clone,
    F: Fn(&V) -> K,
    K: std::hash::Hash + Eq,
{
    let mut groups: HashMap<K, Vec<T>> = HashMap::new();
    for item in collection {
        let value = keypath.get(item).ok_or_else(|| KeyPathError::InvalidAccess { message: "KeyPath access failed".to_string() })?;
        let key = key_fn(value);
        groups.entry(key).or_insert_with(Vec::new).push(item.clone());
    }
    Ok(groups)
}

/// Sort collections by keypath values
/// 
/// # Examples
/// 
/// ```rust
/// use rust_prelude_plus::prelude::*;
/// use key_paths_derive::Keypath;
/// 
/// #[derive(Keypath, Debug, Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
/// 
/// let mut people = vec![
///     Person { name: "Alice".to_string(), age: 30 },
///     Person { name: "Bob".to_string(), age: 25 },
///     Person { name: "Charlie".to_string(), age: 35 },
/// ];
/// 
/// sort_by_keypath(&mut people, Person::age(), |a, b| a.cmp(b)).unwrap();
/// assert_eq!(people[0].age, 25);
/// assert_eq!(people[1].age, 30);
/// assert_eq!(people[2].age, 35);
/// ```
pub fn sort_by_keypath<T, V, F>(
    collection: &mut [T],
    keypath: KeyPaths<T, V>,
    compare: F,
) -> KeyPathResult<()>
where
    F: Fn(&V, &V) -> std::cmp::Ordering,
{
    collection.sort_by(|a, b| {
        let val_a = keypath.get(a).ok_or_else(|| KeyPathError::InvalidAccess { message: "KeyPath access failed".to_string() }).unwrap();
        let val_b = keypath.get(b).ok_or_else(|| KeyPathError::InvalidAccess { message: "KeyPath access failed".to_string() }).unwrap();
        compare(val_a, val_b)
    });
    Ok(())
}

/// Extract values from keypaths into collections
/// 
/// # Examples
/// 
/// ```rust
/// use rust_prelude_plus::prelude::*;
/// use key_paths_derive::Keypath;
/// 
/// #[derive(Keypath, Debug, Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
/// 
/// let people = vec![
///     Person { name: "Alice".to_string(), age: 30 },
///     Person { name: "Bob".to_string(), age: 25 },
/// ];
/// let ages = collect_keypath(people, Person::age()).unwrap();
/// assert_eq!(ages, vec![30, 25]);
/// ```
pub fn collect_keypath<T, V>(
    collection: Vec<T>,
    keypath: KeyPaths<T, V>,
) -> KeyPathResult<Vec<V>>
where
    V: Clone,
{
    let mut result = Vec::new();
    for item in collection {
        let value = keypath.get(&item).ok_or_else(|| KeyPathError::InvalidAccess { message: "KeyPath access failed".to_string() })?;
        result.push(value.clone());
    }
    Ok(result)
}

/// Split collections based on keypath predicates
/// 
/// # Examples
/// 
/// ```rust
/// use rust_prelude_plus::prelude::*;
/// use key_paths_derive::Keypath;
/// 
/// #[derive(Keypath, Debug, Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
/// 
/// let people = vec![
///     Person { name: "Alice".to_string(), age: 30 },
///     Person { name: "Bob".to_string(), age: 25 },
///     Person { name: "Charlie".to_string(), age: 35 },
/// ];
/// 
/// let (young, old) = partition_by_keypath(people, Person::age(), |&age| age < 30).unwrap();
/// assert_eq!(young.len(), 1);
/// assert_eq!(old.len(), 2);
/// assert_eq!(young[0].name, "Bob");
/// ```
pub fn partition_by_keypath<T, V, F>(
    collection: Vec<T>,
    keypath: KeyPaths<T, V>,
    predicate: F,
) -> KeyPathResult<(Vec<T>, Vec<T>)>
where
    F: Fn(&V) -> bool,
{
    let mut left = Vec::new();
    let mut right = Vec::new();
    
    for item in collection {
        let value = keypath.get(&item).ok_or_else(|| KeyPathError::InvalidAccess { message: "KeyPath access failed".to_string() })?;
        if predicate(value) {
            left.push(item);
        } else {
            right.push(item);
        }
    }
    
    Ok((left, right))
}

/// Combine collections using keypath values
/// 
/// # Examples
/// 
/// ```rust
/// use rust_prelude_plus::prelude::*;
/// use key_paths_derive::Keypath;
/// 
/// #[derive(Keypath, Debug, Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
/// 
/// let people1 = vec![
///     Person { name: "Alice".to_string(), age: 30 },
///     Person { name: "Bob".to_string(), age: 25 },
/// ];
/// 
/// let people2 = vec![
///     Person { name: "Charlie".to_string(), age: 35 },
///     Person { name: "David".to_string(), age: 28 },
/// ];
/// 
/// let combined: Vec<(String, String)> = zip_with_keypath(
///     &people1,
///     &people2,
///     Person::name(),
///     Person::name(),
///     |name1, name2| (name1.clone(), name2.clone())
/// ).unwrap();
/// 
/// assert_eq!(combined.len(), 2);
/// assert_eq!(combined[0], ("Alice".to_string(), "Charlie".to_string()));
/// assert_eq!(combined[1], ("Bob".to_string(), "David".to_string()));
/// ```
pub fn zip_with_keypath<T1, T2, V1, V2, F, R>(
    collection1: &[T1],
    collection2: &[T2],
    keypath1: KeyPaths<T1, V1>,
    keypath2: KeyPaths<T2, V2>,
    f: F,
) -> KeyPathResult<Vec<R>>
where
    F: Fn(&V1, &V2) -> R,
{
    let min_len = collection1.len().min(collection2.len());
    let mut result = Vec::new();
    
    for i in 0..min_len {
        let val1 = keypath1.get(&collection1[i]).ok_or_else(|| KeyPathError::InvalidAccess { message: "KeyPath access failed".to_string() })?;
        let val2 = keypath2.get(&collection2[i]).ok_or_else(|| KeyPathError::InvalidAccess { message: "KeyPath access failed".to_string() })?;
        result.push(f(val1, val2));
    }
    
    Ok(result)
}