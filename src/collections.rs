//! Collection operations for keypath-based functional programming

use key_paths_core::KeyPaths;
use crate::error::{KeyPathResult, KeyPathError};
use crate::traits::KeyPathsOperable;
use std::collections::{HashMap, HashSet, BTreeMap};

/// Extension trait for collections with keypath operations
pub trait KeyPathsCollectionExt<T> {
    /// Extract values from keypaths into collections
    fn collect_keypath<V>(&self, keypath: KeyPaths<T, V>) -> KeyPathResult<Vec<V>>
    where
        V: Clone;
    
    /// Partition elements by keypath predicate
    fn partition_by_keypath<V, F>(&self, keypath: KeyPaths<T, V>, predicate: F) -> KeyPathResult<(Vec<T>, Vec<T>)>
    where
        T: Clone,
        F: Fn(&V) -> bool;
    
    /// Group elements by keypath values
    fn group_by_keypath<V, F>(&self, keypath: KeyPaths<T, V>, f: F) -> KeyPathResult<HashMap<V, Vec<T>>>
    where
        V: std::hash::Hash + Eq + Clone,
        T: Clone,
        F: Fn(&V) -> V;
    
    /// Sort elements by keypath values
    fn sort_by_keypath<V, F>(&mut self, keypath: KeyPaths<T, V>, compare: F) -> KeyPathResult<()>
    where
        F: Fn(&V, &V) -> std::cmp::Ordering;
    
    /// Find elements matching keypath conditions
    fn find_by_keypath<V, F>(&self, keypath: KeyPaths<T, V>, predicate: F) -> KeyPathResult<Option<&T>>
    where
        F: Fn(&V) -> bool;
    
    /// Check if any element matches keypath condition
    fn any_by_keypath<V, F>(&self, keypath: KeyPaths<T, V>, predicate: F) -> KeyPathResult<bool>
    where
        F: Fn(&V) -> bool;
    
    /// Check if all elements match keypath condition
    fn all_by_keypath<V, F>(&self, keypath: KeyPaths<T, V>, predicate: F) -> KeyPathResult<bool>
    where
        F: Fn(&V) -> bool;
    
    /// Count elements matching keypath condition
    fn count_by_keypath<V, F>(&self, keypath: KeyPaths<T, V>, predicate: F) -> KeyPathResult<usize>
    where
        F: Fn(&V) -> bool;
    
    /// Get unique values from keypath
    fn unique_by_keypath<V>(&self, keypath: KeyPaths<T, V>) -> KeyPathResult<HashSet<V>>
    where
        V: std::hash::Hash + Eq + Clone;
    
    /// Get distinct values from keypath with counts
    fn distinct_by_keypath<V>(&self, keypath: KeyPaths<T, V>) -> KeyPathResult<HashMap<V, usize>>
    where
        V: std::hash::Hash + Eq + Clone;
    
    /// Zip with another collection using keypath values
    fn zip_with_keypath<U, V1, V2, F, R>(
        &self,
        other: &[U],
        keypath1: KeyPaths<T, V1>,
        keypath2: KeyPaths<U, V2>,
        f: F,
    ) -> KeyPathResult<Vec<R>>
    where
        F: Fn(&V1, &V2) -> R;
    
    /// Window operations over keypath values
    fn window_by_keypath<V, F, R>(
        &self,
        keypath: KeyPaths<T, V>,
        window_size: usize,
        f: F,
    ) -> KeyPathResult<Vec<R>>
    where
        V: Clone,
        F: Fn(&[V]) -> R;
    
    /// Rolling operations over keypath values
    fn rolling_by_keypath<V, F, R>(
        &self,
        keypath: KeyPaths<T, V>,
        window_size: usize,
        f: F,
    ) -> KeyPathResult<Vec<R>>
    where
        V: Clone,
        F: Fn(&[V]) -> R;
}

impl<T: KeyPathsOperable> KeyPathsCollectionExt<T> for Vec<T> {
    fn collect_keypath<V>(&self, keypath: KeyPaths<T, V>) -> KeyPathResult<Vec<V>>
    where
        V: Clone,
    {
        let mut result = Vec::with_capacity(self.len());
            for item in self {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in collect_keypath")
                });
                result.push(value.clone());
            }
        Ok(result)
    }
    
    fn partition_by_keypath<V, F>(&self, keypath: KeyPaths<T, V>, predicate: F) -> KeyPathResult<(Vec<T>, Vec<T>)>
    where
        T: Clone,
        F: Fn(&V) -> bool,
    {
        let mut left = Vec::new();
        let mut right = Vec::new();
        
            for item in self {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in partition_by_keypath")
                });
                if predicate(value) {
                    left.push(item.clone());
                } else {
                    right.push(item.clone());
                }
            }
        
        Ok((left, right))
    }
    
    fn group_by_keypath<V, F>(&self, keypath: KeyPaths<T, V>, f: F) -> KeyPathResult<HashMap<V, Vec<T>>>
    where
        V: std::hash::Hash + Eq + Clone,
        T: Clone,
        F: Fn(&V) -> V,
    {
        let mut groups = HashMap::new();
        for item in self {
            let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                panic!("KeyPath access failed in group_by_keypath")
            });
            let key = f(value);
            groups.entry(key).or_insert_with(Vec::new).push(item.clone());
        }
        Ok(groups)
    }
    
    fn sort_by_keypath<V, F>(&mut self, keypath: KeyPaths<T, V>, compare: F) -> KeyPathResult<()>
    where
        F: Fn(&V, &V) -> std::cmp::Ordering,
    {
        self.sort_by(|a, b| {
            let a_val = a.get_at_keypath(&keypath).unwrap_or_else(|_| {
                panic!("KeyPath access failed in sort_by_keypath")
            });
            let b_val = b.get_at_keypath(&keypath).unwrap_or_else(|_| {
                panic!("KeyPath access failed in sort_by_keypath")
            });
            compare(a_val, b_val)
        });
        Ok(())
    }
    
    fn find_by_keypath<V, F>(&self, keypath: KeyPaths<T, V>, predicate: F) -> KeyPathResult<Option<&T>>
    where
        F: Fn(&V) -> bool,
    {
        for item in self {
            let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                panic!("KeyPath access failed in find_by_keypath")
            });
            if predicate(value) {
                return Ok(Some(item));
            }
        }
        Ok(None)
    }
    
    fn any_by_keypath<V, F>(&self, keypath: KeyPaths<T, V>, predicate: F) -> KeyPathResult<bool>
    where
        F: Fn(&V) -> bool,
    {
        for item in self {
            let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                panic!("KeyPath access failed in any_by_keypath")
            });
            if predicate(value) {
                return Ok(true);
            }
        }
        Ok(false)
    }
    
    fn all_by_keypath<V, F>(&self, keypath: KeyPaths<T, V>, predicate: F) -> KeyPathResult<bool>
    where
        F: Fn(&V) -> bool,
    {
        for item in self {
            let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                panic!("KeyPath access failed in all_by_keypath")
            });
            if !predicate(value) {
                return Ok(false);
            }
        }
        Ok(true)
    }
    
    fn count_by_keypath<V, F>(&self, keypath: KeyPaths<T, V>, predicate: F) -> KeyPathResult<usize>
    where
        F: Fn(&V) -> bool,
    {
        let mut count = 0;
        for item in self {
            let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                panic!("KeyPath access failed in count_by_keypath")
            });
            if predicate(value) {
                count += 1;
            }
        }
        Ok(count)
    }
    
    fn unique_by_keypath<V>(&self, keypath: KeyPaths<T, V>) -> KeyPathResult<HashSet<V>>
    where
        V: std::hash::Hash + Eq + Clone,
    {
        let mut unique = HashSet::new();
        for item in self {
            let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                panic!("KeyPath access failed in unique_by_keypath")
            });
            unique.insert(value.clone());
        }
        Ok(unique)
    }
    
    fn distinct_by_keypath<V>(&self, keypath: KeyPaths<T, V>) -> KeyPathResult<HashMap<V, usize>>
    where
        V: std::hash::Hash + Eq + Clone,
    {
        let mut counts = HashMap::new();
        for item in self {
            let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                panic!("KeyPath access failed in distinct_by_keypath")
            });
            *counts.entry(value.clone()).or_insert(0) += 1;
        }
        Ok(counts)
    }
    
    fn zip_with_keypath<U, V1, V2, F, R>(
        &self,
        other: &[U],
        keypath1: KeyPaths<T, V1>,
        keypath2: KeyPaths<U, V2>,
        f: F,
    ) -> KeyPathResult<Vec<R>>
    where
        U: KeyPathsOperable,
        F: Fn(&V1, &V2) -> R,
    {
        let min_len = self.len().min(other.len());
        let mut result = Vec::with_capacity(min_len);
        
        for i in 0..min_len {
            let value1 = self[i].get_at_keypath(&keypath1).unwrap_or_else(|_| {
                panic!("KeyPath access failed in zip_with_keypath")
            });
            let value2 = other[i].get_at_keypath(&keypath2).unwrap_or_else(|_| {
                panic!("KeyPath access failed in zip_with_keypath")
            });
            result.push(f(value1, value2));
        }
        
        Ok(result)
    }
    
    fn window_by_keypath<V, F, R>(
        &self,
        keypath: KeyPaths<T, V>,
        window_size: usize,
        f: F,
    ) -> KeyPathResult<Vec<R>>
    where
        V: Clone,
        F: Fn(&[V]) -> R,
    {
        if window_size == 0 || window_size > self.len() {
            return Err(KeyPathError::CollectionError {
                message: format!("Invalid window size: {}", window_size),
            });
        }
        
        let mut result = Vec::new();
        for i in 0..=self.len() - window_size {
            let window: Vec<V> = self[i..i + window_size]
                .iter()
                .map(|item| item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in window_by_keypath")
                }).clone())
                .collect();
            result.push(f(&window));
        }
        
        Ok(result)
    }
    
    fn rolling_by_keypath<V, F, R>(
        &self,
        keypath: KeyPaths<T, V>,
        window_size: usize,
        f: F,
    ) -> KeyPathResult<Vec<R>>
    where
        V: Clone,
        F: Fn(&[V]) -> R,
    {
        if window_size == 0 {
            return Err(KeyPathError::CollectionError {
                message: "Window size must be greater than 0".to_string(),
            });
        }
        
        let mut result = Vec::new();
        let mut window = Vec::with_capacity(window_size);
        
        for item in self {
            let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                panic!("KeyPath access failed in rolling_by_keypath")
            }).clone();
            window.push(value);
            
            if window.len() == window_size {
                result.push(f(&window));
                window.remove(0);
            }
        }
        
        Ok(result)
    }
}

/// Specialized collection operations for different data structures
pub mod specialized {
    use super::*;
    
    /// Operations for HashMap collections
    pub trait KeyPathsHashMapExt<K, V> {
        /// Transform values using keypath
        fn map_values_keypath<T, F, R>(&self, keypath: KeyPaths<V, T>, f: F) -> KeyPathResult<HashMap<K, R>>
        where
            K: Clone,
            F: Fn(&T) -> R;
        
        /// Filter by keypath predicate on values
        fn filter_values_keypath<T, F>(&self, keypath: KeyPaths<V, T>, predicate: F) -> KeyPathResult<HashMap<K, V>>
        where
            K: Clone,
            V: Clone,
            F: Fn(&T) -> bool;
    }
    
    impl<K: std::hash::Hash + std::cmp::Eq, V: KeyPathsOperable> KeyPathsHashMapExt<K, V> for HashMap<K, V> {
        fn map_values_keypath<T, F, R>(&self, keypath: KeyPaths<V, T>, f: F) -> KeyPathResult<HashMap<K, R>>
        where
            K: Clone,
            F: Fn(&T) -> R,
        {
            let mut result = HashMap::new();
            for (key, value) in self {
                let keypath_value = value.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in map_values_keypath")
                });
                result.insert(key.clone(), f(keypath_value));
            }
            Ok(result)
        }
        
        fn filter_values_keypath<T, F>(&self, keypath: KeyPaths<V, T>, predicate: F) -> KeyPathResult<HashMap<K, V>>
        where
            K: Clone,
            V: Clone,
            F: Fn(&T) -> bool,
        {
            let mut result = HashMap::new();
            for (key, value) in self {
                let keypath_value = keypath.get(value).unwrap_or_else(|| {
                    panic!("KeyPath access failed in filter_values_keypath")
                });
                if predicate(keypath_value) {
                    result.insert(key.clone(), value.clone());
                }
            }
            Ok(result)
        }
    }
    
    /// Operations for BTreeMap collections
    pub trait KeyPathsBTreeMapExt<K, V> {
        /// Transform values using keypath
        fn map_values_keypath<T, F, R>(&self, keypath: KeyPaths<V, T>, f: F) -> KeyPathResult<BTreeMap<K, R>>
        where
            K: Clone + Ord,
            F: Fn(&T) -> R;
        
        /// Filter by keypath predicate on values
        fn filter_values_keypath<T, F>(&self, keypath: KeyPaths<V, T>, predicate: F) -> KeyPathResult<BTreeMap<K, V>>
        where
            K: Clone + Ord,
            V: Clone,
            F: Fn(&T) -> bool;
    }
    
    impl<K: std::cmp::Ord, V: KeyPathsOperable> KeyPathsBTreeMapExt<K, V> for BTreeMap<K, V> {
        fn map_values_keypath<T, F, R>(&self, keypath: KeyPaths<V, T>, f: F) -> KeyPathResult<BTreeMap<K, R>>
        where
            K: Clone + Ord,
            F: Fn(&T) -> R,
        {
            let mut result = BTreeMap::new();
            for (key, value) in self {
                let keypath_value = value.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in map_values_keypath")
                });
                result.insert(key.clone(), f(keypath_value));
            }
            Ok(result)
        }
        
        fn filter_values_keypath<T, F>(&self, keypath: KeyPaths<V, T>, predicate: F) -> KeyPathResult<BTreeMap<K, V>>
        where
            K: Clone + Ord,
            V: Clone,
            F: Fn(&T) -> bool,
        {
            let mut result = BTreeMap::new();
            for (key, value) in self {
                let keypath_value = keypath.get(value).unwrap_or_else(|| {
                    panic!("KeyPath access failed in filter_values_keypath")
                });
                if predicate(keypath_value) {
                    result.insert(key.clone(), value.clone());
                }
            }
            Ok(result)
        }
    }
}

/// Utility functions for collection operations
pub mod utils {
    use super::*;
    
    /// Create a keypath-based comparator for sorting
    pub fn create_keypath_comparator<T: KeyPathsOperable, V, F>(
        keypath: KeyPaths<T, V>,
        compare: F,
    ) -> impl Fn(&T, &T) -> std::cmp::Ordering
    where
        F: Fn(&V, &V) -> std::cmp::Ordering,
    {
        move |a, b| {
            let a_val = a.get_at_keypath(&keypath).unwrap_or_else(|_| {
                panic!("KeyPath access failed in create_keypath_comparator")
            });
            let b_val = b.get_at_keypath(&keypath).unwrap_or_else(|_| {
                panic!("KeyPath access failed in create_keypath_comparator")
            });
            compare(a_val, b_val)
        }
    }
    
    /// Create a keypath-based hash function
    pub fn create_keypath_hasher<T: KeyPathsOperable, V, H>(
        keypath: KeyPaths<T, V>,
        hasher: H,
    ) -> impl Fn(&T) -> u64
    where
        H: Fn(&V) -> u64,
    {
        move |item| {
            let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                panic!("KeyPath access failed in create_keypath_hasher")
            });
            hasher(value)
        }
    }
    
    /// Create a keypath-based equality function
    pub fn create_keypath_equality<T: KeyPathsOperable, V, E>(
        keypath: KeyPaths<T, V>,
        equality: E,
    ) -> impl Fn(&T, &T) -> bool
    where
        E: Fn(&V, &V) -> bool,
    {
        move |a, b| {
            let a_val = a.get_at_keypath(&keypath).unwrap_or_else(|_| {
                panic!("KeyPath access failed in create_keypath_equality")
            });
            let b_val = b.get_at_keypath(&keypath).unwrap_or_else(|_| {
                panic!("KeyPath access failed in create_keypath_equality")
            });
            equality(a_val, b_val)
        }
    }
}
