//! Higher-order functions for keypath operations

use crate::error::KeyPathResult;

/// A simple keypath trait for demonstration purposes
pub trait KeyPath<T, V> {
    fn get<'a>(&self, data: &'a T) -> &'a V;
    fn get_mut<'a>(&self, data: &'a mut T) -> &'a mut V;
}

/// Transform values at a specific keypath
/// 
/// # Examples
/// 
/// ```rust
/// use rust_prelude_plus::prelude::*;
/// 
/// struct Person {
///     name: String,
///     age: u32,
/// }
/// 
/// struct NameKeyPath;
/// impl KeyPath<Person, String> for NameKeyPath {
///     fn get<'a>(&self, data: &'a Person) -> &'a String { &data.name }
///     fn get_mut<'a>(&self, data: &'a mut Person) -> &'a mut String { &mut data.name }
/// }
/// 
/// let person = Person { name: "Alice".to_string(), age: 30 };
/// let result = map_keypath(person, NameKeyPath, |name| name.to_uppercase()).unwrap();
/// assert_eq!(result, "ALICE");
/// ```
pub fn map_keypath<T, V, F, R>(
    data: T,
    keypath: impl KeyPath<T, V>,
    f: F,
) -> KeyPathResult<R>
where
    F: FnOnce(&V) -> R,
{
    let value = keypath.get(&data);
    Ok(f(value))
}

/// Transform values at a specific keypath for collections
pub fn map_keypath_collection<T, V, F, R>(
    collection: &[T],
    keypath: impl KeyPath<T, V>,
    f: F,
) -> KeyPathResult<Vec<R>>
where
    F: Fn(&V) -> R,
{
    let result: Vec<R> = collection
        .iter()
        .map(|item| {
            let value = keypath.get(item);
            f(value)
        })
        .collect();
    Ok(result)
}

/// Filter collections based on keypath values
/// 
/// # Examples
/// 
/// ```rust
/// use rust_prelude_plus::prelude::*;
/// 
/// struct Person {
///     name: String,
///     age: u32,
/// }
/// 
/// struct AgeKeyPath;
/// impl KeyPath<Person, u32> for AgeKeyPath {
///     fn get<'a>(&self, data: &'a Person) -> &'a u32 { &data.age }
///     fn get_mut<'a>(&self, data: &'a mut Person) -> &'a mut u32 { &mut data.age }
/// }
/// 
/// let people = vec![
///     Person { name: "Alice".to_string(), age: 30 },
///     Person { name: "Bob".to_string(), age: 25 },
///     Person { name: "Charlie".to_string(), age: 35 },
/// ];
/// 
/// let young_people = filter_by_keypath(people, AgeKeyPath, |&age| age < 30).unwrap();
/// assert_eq!(young_people.len(), 1);
/// assert_eq!(young_people[0].name, "Bob");
/// ```
pub fn filter_by_keypath<T, V, F>(
    collection: Vec<T>,
    keypath: impl KeyPath<T, V>,
    predicate: F,
) -> KeyPathResult<Vec<T>>
where
    F: Fn(&V) -> bool,
{
    let result: Vec<T> = collection
        .into_iter()
        .filter(|item| {
            let value = keypath.get(item);
            predicate(value)
        })
        .collect();
    Ok(result)
}

/// Accumulate values from keypaths
/// 
/// # Examples
/// 
/// ```rust
/// use rust_prelude_plus::prelude::*;
/// 
/// struct Person {
///     name: String,
///     age: u32,
/// }
/// 
/// struct AgeKeyPath;
/// impl KeyPath<Person, u32> for AgeKeyPath {
///     fn get<'a>(&self, data: &'a Person) -> &'a u32 { &data.age }
///     fn get_mut<'a>(&self, data: &'a mut Person) -> &'a mut u32 { &mut data.age }
/// }
/// 
/// let people = vec![
///     Person { name: "Alice".to_string(), age: 30 },
///     Person { name: "Bob".to_string(), age: 25 },
///     Person { name: "Charlie".to_string(), age: 35 },
/// ];
/// 
/// let total_age = fold_keypath(people, AgeKeyPath, 0, |acc, &age| acc + age).unwrap();
/// assert_eq!(total_age, 90);
/// ```
pub fn fold_keypath<T, V, F, B>(
    collection: Vec<T>,
    keypath: impl KeyPath<T, V>,
    init: B,
    f: F,
) -> KeyPathResult<B>
where
    F: Fn(B, &V) -> B,
{
    let mut acc = init;
    for item in collection {
        let value = keypath.get(&item);
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
/// 
/// struct Person {
///     name: String,
///     age: u32,
/// }
/// 
/// struct AgeKeyPath;
/// impl KeyPath<Person, u32> for AgeKeyPath {
///     fn get<'a>(&self, data: &'a Person) -> &'a u32 { &data.age }
///     fn get_mut<'a>(&self, data: &'a mut Person) -> &'a mut u32 { &mut data.age }
/// }
/// 
/// let people = vec![
///     Person { name: "Alice".to_string(), age: 30 },
///     Person { name: "Bob".to_string(), age: 25 },
///     Person { name: "Charlie".to_string(), age: 35 },
/// ];
/// 
/// let found = find_by_keypath(people, AgeKeyPath, |&age| age == 30).unwrap();
/// assert!(found.is_some());
/// assert_eq!(found.unwrap().name, "Alice");
/// ```
pub fn find_by_keypath<T, V, F>(
    collection: Vec<T>,
    keypath: impl KeyPath<T, V>,
    predicate: F,
) -> KeyPathResult<Option<T>>
where
    F: Fn(&V) -> bool,
{
    for item in collection {
        let value = keypath.get(&item);
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
/// use std::collections::HashMap;
/// 
/// #[derive(Clone)]
/// struct Person {
///     name: String,
///     age: u32,
///     department: String,
/// }
/// 
/// struct DepartmentKeyPath;
/// impl KeyPath<Person, String> for DepartmentKeyPath {
///     fn get<'a>(&self, data: &'a Person) -> &'a String { &data.department }
///     fn get_mut<'a>(&self, data: &'a mut Person) -> &'a mut String { &mut data.department }
/// }
/// 
/// let people = vec![
///     Person { name: "Alice".to_string(), age: 30, department: "Engineering".to_string() },
///     Person { name: "Bob".to_string(), age: 25, department: "Engineering".to_string() },
///     Person { name: "Charlie".to_string(), age: 35, department: "Marketing".to_string() },
/// ];
/// 
/// let grouped = group_by_keypath(&people, DepartmentKeyPath, |dept| dept.clone()).unwrap();
/// assert_eq!(grouped.len(), 2);
/// assert_eq!(grouped["Engineering"].len(), 2);
/// assert_eq!(grouped["Marketing"].len(), 1);
/// ```
pub fn group_by_keypath<T, V, F>(
    collection: &[T],
    keypath: impl KeyPath<T, V>,
    f: F,
) -> KeyPathResult<std::collections::HashMap<V, Vec<T>>>
where
    V: std::hash::Hash + Eq + Clone,
    T: Clone,
    F: Fn(&V) -> V,
{
    let mut groups = std::collections::HashMap::new();
    for item in collection {
        let value = keypath.get(item);
        let key = f(value);
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
/// use std::cmp::Ordering;
/// 
/// struct Person {
///     name: String,
///     age: u32,
/// }
/// 
/// struct AgeKeyPath;
/// impl KeyPath<Person, u32> for AgeKeyPath {
///     fn get<'a>(&self, data: &'a Person) -> &'a u32 { &data.age }
///     fn get_mut<'a>(&self, data: &'a mut Person) -> &'a mut u32 { &mut data.age }
/// }
/// 
/// let mut people = vec![
///     Person { name: "Alice".to_string(), age: 30 },
///     Person { name: "Bob".to_string(), age: 25 },
///     Person { name: "Charlie".to_string(), age: 35 },
/// ];
/// 
/// sort_by_keypath(&mut people, AgeKeyPath, |a, b| a.cmp(b)).unwrap();
/// 
/// assert_eq!(people[0].name, "Bob");
/// assert_eq!(people[1].name, "Alice");
/// assert_eq!(people[2].name, "Charlie");
/// ```
pub fn sort_by_keypath<T, V, F>(
    collection: &mut [T],
    keypath: impl KeyPath<T, V>,
    compare: F,
) -> KeyPathResult<()>
where
    F: Fn(&V, &V) -> std::cmp::Ordering,
{
    collection.sort_by(|a, b| {
        let a_val = keypath.get(a);
        let b_val = keypath.get(b);
        compare(a_val, b_val)
    });
    Ok(())
}

/// Extract values from keypaths into collections
/// 
/// # Examples
/// 
/// ```rust
/// use rust_prelude_plus::prelude::*;
/// 
/// struct Person {
///     name: String,
///     age: u32,
/// }
/// 
/// struct AgeKeyPath;
/// impl KeyPath<Person, u32> for AgeKeyPath {
///     fn get<'a>(&self, data: &'a Person) -> &'a u32 { &data.age }
///     fn get_mut<'a>(&self, data: &'a mut Person) -> &'a mut u32 { &mut data.age }
/// }
/// 
/// let people = vec![
///     Person { name: "Alice".to_string(), age: 30 },
///     Person { name: "Bob".to_string(), age: 25 },
///     Person { name: "Charlie".to_string(), age: 35 },
/// ];
/// 
/// let ages = collect_keypath(people, AgeKeyPath).unwrap();
/// assert_eq!(ages, vec![30, 25, 35]);
/// ```
pub fn collect_keypath<T, V>(
    collection: Vec<T>,
    keypath: impl KeyPath<T, V>,
) -> KeyPathResult<Vec<V>>
where
    V: Clone,
{
    let mut result = Vec::new();
    for item in collection {
        let value = keypath.get(&item);
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
/// 
/// struct Person {
///     name: String,
///     age: u32,
/// }
/// 
/// struct AgeKeyPath;
/// impl KeyPath<Person, u32> for AgeKeyPath {
///     fn get<'a>(&self, data: &'a Person) -> &'a u32 { &data.age }
///     fn get_mut<'a>(&self, data: &'a mut Person) -> &'a mut u32 { &mut data.age }
/// }
/// 
/// let people = vec![
///     Person { name: "Alice".to_string(), age: 30 },
///     Person { name: "Bob".to_string(), age: 25 },
///     Person { name: "Charlie".to_string(), age: 35 },
/// ];
/// 
/// let (young, old) = partition_by_keypath(people, AgeKeyPath, |&age| age < 30).unwrap();
/// assert_eq!(young.len(), 1);
/// assert_eq!(old.len(), 2);
/// assert_eq!(young[0].name, "Bob");
/// ```
pub fn partition_by_keypath<T, V, F>(
    collection: Vec<T>,
    keypath: impl KeyPath<T, V>,
    predicate: F,
) -> KeyPathResult<(Vec<T>, Vec<T>)>
where
    F: Fn(&V) -> bool,
{
    let mut left = Vec::new();
    let mut right = Vec::new();
    
    for item in collection {
        let value = keypath.get(&item);
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
/// 
/// struct Person {
///     name: String,
///     age: u32,
/// }
/// 
/// struct NameKeyPath;
/// impl KeyPath<Person, String> for NameKeyPath {
///     fn get<'a>(&self, data: &'a Person) -> &'a String { &data.name }
///     fn get_mut<'a>(&self, data: &'a mut Person) -> &'a mut String { &mut data.name }
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
/// let combined = zip_with_keypath(
///     &people1,
///     &people2,
///     NameKeyPath,
///     NameKeyPath,
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
    keypath1: impl KeyPath<T1, V1>,
    keypath2: impl KeyPath<T2, V2>,
    f: F,
) -> KeyPathResult<Vec<R>>
where
    F: Fn(&V1, &V2) -> R,
{
    let min_len = collection1.len().min(collection2.len());
    let mut result = Vec::with_capacity(min_len);
    
    for i in 0..min_len {
        let value1 = keypath1.get(&collection1[i]);
        let value2 = keypath2.get(&collection2[i]);
        result.push(f(value1, value2));
    }
    
    Ok(result)
}