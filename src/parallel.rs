//! Parallel operations for keypath-based functional programming

#[cfg(feature = "parallel")]
use {
    rayon::prelude::*,
    key_paths_core::KeyPaths,
    crate::error::{KeyPathResult, KeyPathError},
    crate::traits::KeyPathsOperable,
};

#[cfg(feature = "parallel")]
/// Parallel keypath operations for collections
pub mod parallel_collections {
    use super::*;
    
    /// Parallel map over collection with keypath
    pub fn par_map_keypath<T, V, F, R>(
        collection: Vec<T>,
        keypath: KeyPaths<T, V>,
        f: F,
    ) -> KeyPathResult<Vec<R>>
    where
        T: Send + Sync + KeyPathsOperable,
        V: Send + Sync,
        KeyPaths<T, V>: Send + Sync,
        F: Fn(&V) -> R + Send + Sync,
        R: Send,
    {
        let result: Vec<R> = collection
            .into_par_iter()
            .map(|item| {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in par_map_keypath")
                });
                f(value)
            })
            .collect();
        Ok(result)
    }
    
    /// Parallel filter by keypath predicate
    pub fn par_filter_by_keypath<T, V, F>(
        collection: Vec<T>,
        keypath: KeyPaths<T, V>,
        predicate: F,
    ) -> KeyPathResult<Vec<T>>
    where
        T: Send + Sync + KeyPathsOperable,
        V: Send + Sync,
        KeyPaths<T, V>: Send + Sync,
        F: Fn(&V) -> bool + Send + Sync,
    {
        let result: Vec<T> = collection
            .into_par_iter()
            .filter(|item| {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in par_filter_by_keypath")
                });
                predicate(value)
            })
            .collect();
        Ok(result)
    }
    
    /// Parallel find by keypath predicate
    pub fn par_find_by_keypath<T, V, F>(
        collection: Vec<T>,
        keypath: KeyPaths<T, V>,
        predicate: F,
    ) -> KeyPathResult<Option<T>>
    where
        T: Send + Sync + KeyPathsOperable,
        V: Send + Sync,
        KeyPaths<T, V>: Send + Sync,
        F: Fn(&V) -> bool + Send + Sync,
    {
        let result = collection
            .into_par_iter()
            .find_any(|item| {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in par_find_by_keypath")
                });
                predicate(value)
            });
        Ok(result)
    }
    
    /// Parallel collect keypath values
    pub fn par_collect_keypath<T, V>(
        collection: Vec<T>,
        keypath: KeyPaths<T, V>,
    ) -> KeyPathResult<Vec<V>>
    where
        T: Send + Sync + KeyPathsOperable,
        V: Send + Sync + Clone,
        KeyPaths<T, V>: Send + Sync,
    {
        let result: Vec<V> = collection
            .into_par_iter()
            .map(|item| {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in par_collect_keypath")
                });
                value.clone()
            })
            .collect();
        Ok(result)
    }
    
    /// Parallel count by keypath predicate
    pub fn par_count_by_keypath<T, V, F>(
        collection: Vec<T>,
        keypath: KeyPaths<T, V>,
        predicate: F,
    ) -> KeyPathResult<usize>
    where
        T: Send + Sync + KeyPathsOperable,
        V: Send + Sync,
        KeyPaths<T, V>: Send + Sync,
        F: Fn(&V) -> bool + Send + Sync,
    {
        let count = collection
            .into_par_iter()
            .filter(|item| {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in par_count_by_keypath")
                });
                predicate(value)
            })
            .count();
        Ok(count)
    }
    
    /// Parallel any by keypath predicate
    pub fn par_any_by_keypath<T, V, F>(
        collection: Vec<T>,
        keypath: KeyPaths<T, V>,
        predicate: F,
    ) -> KeyPathResult<bool>
    where
        T: Send + Sync + KeyPathsOperable,
        V: Send + Sync,
        KeyPaths<T, V>: Send + Sync,
        F: Fn(&V) -> bool + Send + Sync,
    {
        let result = collection
            .into_par_iter()
            .any(|item| {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in par_any_by_keypath")
                });
                predicate(value)
            });
        Ok(result)
    }
    
    /// Parallel all by keypath predicate
    pub fn par_all_by_keypath<T, V, F>(
        collection: Vec<T>,
        keypath: KeyPaths<T, V>,
        predicate: F,
    ) -> KeyPathResult<bool>
    where
        T: Send + Sync + KeyPathsOperable,
        V: Send + Sync,
        KeyPaths<T, V>: Send + Sync,
        F: Fn(&V) -> bool + Send + Sync,
    {
        let result = collection
            .into_par_iter()
            .all(|item| {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in par_all_by_keypath")
                });
                predicate(value)
            });
        Ok(result)
    }
}

#[cfg(feature = "parallel")]
/// Parallel keypath operations with custom thread pools
pub mod parallel_pools {
    use super::*;
    use rayon::{ThreadPool, ThreadPoolBuilder};
    
    /// Create a custom thread pool for keypath operations
    pub fn create_keypath_thread_pool(num_threads: usize) -> Result<ThreadPool, KeyPathError> {
        ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .map_err(|e| KeyPathError::ParallelError {
                message: format!("Failed to create thread pool: {}", e),
            })
    }
    
    /// Execute keypath operations on custom thread pool
    pub fn execute_on_pool<T, V, F, R>(
        pool: &ThreadPool,
        collection: Vec<T>,
        keypath: KeyPaths<T, V>,
        operation: F,
    ) -> KeyPathResult<Vec<R>>
    where
        T: Send + Sync + KeyPathsOperable,
        V: Send + Sync,
        KeyPaths<T, V>: Send + Sync,
        F: Fn(&V) -> R + Send + Sync,
        R: Send,
    {
        pool.install(|| {
            parallel_collections::par_map_keypath(collection, keypath, operation)
        })
    }
    
    /// Execute keypath filter on custom thread pool
    pub fn filter_on_pool<T, V, F>(
        pool: &ThreadPool,
        collection: Vec<T>,
        keypath: KeyPaths<T, V>,
        predicate: F,
    ) -> KeyPathResult<Vec<T>>
    where
        T: Send + Sync + KeyPathsOperable,
        V: Send + Sync,
        KeyPaths<T, V>: Send + Sync,
        F: Fn(&V) -> bool + Send + Sync,
    {
        pool.install(|| {
            parallel_collections::par_filter_by_keypath(collection, keypath, predicate)
        })
    }
}