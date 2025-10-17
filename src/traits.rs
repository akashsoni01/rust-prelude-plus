//! Core traits for keypath operations

use key_paths_core::KeyPaths;
use crate::error::{KeyPathResult, KeyPathError};

/// Trait for types that can be operated on with keypaths
pub trait KeyPathsOperable {
    /// Get a value at a keypath
    fn get_at_keypath<V>(&self, keypath: impl KeyPaths<Self, V>) -> KeyPathResult<&V> {
        Ok(keypath.get(self))
    }
    
    /// Set a value at a keypath
    fn set_at_keypath<V>(&mut self, keypath: impl KeyPaths<Self, V>, value: V) -> KeyPathResult<()> {
        keypath.set(self, value);
        Ok(())
    }
}

/// Trait for iterators that support keypath operations
pub trait KeyPathsIterator: Iterator {
    /// Map over a keypath in the iterator
    fn map_keypath<V, F, R>(self, keypath: impl KeyPaths<Self::Item, V>, f: F) -> MapKeyPaths<Self, V, F>
    where
        Self: Sized,
        F: FnMut(&V) -> R,
    {
        MapKeyPaths::new(self, keypath, f)
    }
    
    /// Filter by a keypath predicate
    fn filter_by_keypath<V, F>(self, keypath: impl KeyPaths<Self::Item, V>, predicate: F) -> FilterKeyPaths<Self, V, F>
    where
        Self: Sized,
        F: FnMut(&V) -> bool,
    {
        FilterKeyPaths::new(self, keypath, predicate)
    }
    
    /// Find an element by keypath predicate
    fn find_by_keypath<V, F>(self, keypath: impl KeyPaths<Self::Item, V>, predicate: F) -> KeyPathResult<Option<Self::Item>>
    where
        Self: Sized,
        F: FnMut(&V) -> bool,
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
    fn fold_keypath<V, F, B>(self, keypath: impl KeyPaths<Self::Item, V>, init: B, f: F) -> KeyPathResult<B>
    where
        Self: Sized,
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
    fn collect_keypath<V>(self, keypath: impl KeyPaths<Self::Item, V>) -> KeyPathResult<Vec<V>>
    where
        Self: Sized,
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
    fn group_by_keypath<V, F>(&self, keypath: impl KeyPaths<T, V>, f: F) -> KeyPathResult<std::collections::HashMap<V, Vec<T>>>
    where
        V: std::hash::Hash + Eq + Clone,
        T: Clone,
        F: Fn(&V) -> V;
    
    /// Partition elements by keypath predicate
    fn partition_by_keypath<V, F>(&self, keypath: impl KeyPaths<T, V>, predicate: F) -> KeyPathResult<(Vec<T>, Vec<T>)>
    where
        T: Clone,
        F: Fn(&V) -> bool;
    
    /// Sort elements by keypath values
    fn sort_by_keypath<V, F>(&mut self, keypath: impl KeyPaths<T, V>, compare: F) -> KeyPathResult<()>
    where
        F: Fn(&V, &V) -> std::cmp::Ordering;
}

/// Iterator adapter for mapping over keypaths
pub struct MapKeyPaths<I, V, F> {
    iter: I,
    keypath: V,
    f: F,
}

impl<I, V, F> MapKeyPaths<I, V, F> {
    fn new(iter: I, keypath: V, f: F) -> Self {
        Self { iter, keypath, f }
    }
}

impl<I, V, F, R> Iterator for MapKeyPaths<I, V, F>
where
    I: Iterator,
    V: KeyPaths<I::Item, V::Target>,
    F: FnMut(&V::Target) -> R,
{
    type Item = R;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| {
            let value = self.keypath.get(&item);
            (self.f)(value)
        })
    }
}

/// Iterator adapter for filtering by keypaths
pub struct FilterKeyPaths<I, V, F> {
    iter: I,
    keypath: V,
    predicate: F,
}

impl<I, V, F> FilterKeyPaths<I, V, F> {
    fn new(iter: I, keypath: V, predicate: F) -> Self {
        Self { iter, keypath, predicate }
    }
}

impl<I, V, F> Iterator for FilterKeyPaths<I, V, F>
where
    I: Iterator,
    V: KeyPaths<I::Item, V::Target>,
    F: FnMut(&V::Target) -> bool,
{
    type Item = I::Item;
    
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            let value = self.keypath.get(&item);
            if (self.predicate)(value) {
                return Some(item);
            }
        }
        None
    }
}

// Implement KeyPathsOperable for all types
impl<T> KeyPathsOperable for T {}

// Implement KeyPathsIterator for all iterators
impl<I: Iterator> KeyPathsIterator for I {}

// Implement KeyPathsCollection for Vec
impl<T> KeyPathsCollection<T> for Vec<T> {
    fn group_by_keypath<V, F>(&self, keypath: impl KeyPaths<T, V>, f: F) -> KeyPathResult<std::collections::HashMap<V, Vec<T>>>
    where
        V: std::hash::Hash + Eq + Clone,
        T: Clone,
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
    
    fn partition_by_keypath<V, F>(&self, keypath: impl KeyPaths<T, V>, predicate: F) -> KeyPathResult<(Vec<T>, Vec<T>)>
    where
        T: Clone,
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
    
    fn sort_by_keypath<V, F>(&mut self, keypath: impl KeyPaths<T, V>, compare: F) -> KeyPathResult<()>
    where
        F: Fn(&V, &V) -> std::cmp::Ordering,
    {
        self.sort_by(|a, b| {
            let a_val = keypath.get(a);
            let b_val = keypath.get(b);
            compare(a_val, b_val)
        });
        Ok(())
    }
}
