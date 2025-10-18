//! Parallel operations for keypath-based functional programming

#[cfg(feature = "parallel")]
use {
    rayon::prelude::*,
    key_paths_core::KeyPaths,
    crate::error::{KeyPathResult, KeyPathError},
    crate::traits::KeyPathsOperable,
    std::sync::Arc,
};

#[cfg(feature = "parallel")]
/// Parallel keypath operations for collections
pub mod parallel_collections {
    use super::*;
    
    /// Parallel map over collection with keypath
    pub fn par_map_keypath<T, V, F, R>(
        collection: &[T],
        keypath: KeyPaths<T, V>,
        f: F,
    ) -> KeyPathResult<Vec<R>>
    where
        T: Send + Sync + KeyPathsOperable,
        V: Send + Sync,
        KeyPaths<T, V>: Send + Sync,
        KeyPaths<T, V>: Send + Sync,
        F: Fn(&V) -> R + Send + Sync,
        R: Send,
    {
        let result: Vec<R> = collection
            .par_iter()
            .map(|item| {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in par_map_keypath")
                });
                f(value)
            })
            .collect();
        Ok(result)
    }
    
    /// Parallel filter collection by keypath
    pub fn par_filter_by_keypath<T, V, F>(
        collection: &[T],
        keypath: KeyPaths<T, V>,
        predicate: F,
    ) -> KeyPathResult<Vec<T>>
    where
        T: Send + Sync + Clone,
        V: Send + Sync,
        F: Fn(&V) -> bool + Send + Sync,
    {
        let result: Vec<T> = collection
            .par_iter()
            .filter(|item| {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in par_map_keypath")
                });
                predicate(value)
            })
            .cloned()
            .collect();
        Ok(result)
    }
    
    /// Parallel find in collection by keypath
    pub fn par_find_by_keypath<T, V, F>(
        collection: &[T],
        keypath: KeyPaths<T, V>,
        predicate: F,
    ) -> KeyPathResult<Option<T>>
    where
        T: Send + Sync + Clone,
        V: Send + Sync,
        F: Fn(&V) -> bool + Send + Sync,
    {
        let result = collection
            .par_iter()
            .find_any(|item| {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in par_map_keypath")
                });
                predicate(value)
            })
            .cloned();
        Ok(result)
    }
    
    /// Parallel fold over collection with keypath
    pub fn par_fold_keypath<T, V, F, B>(
        collection: &[T],
        keypath: KeyPaths<T, V>,
        init: B,
        f: F,
    ) -> KeyPathResult<B>
    where
        T: Send + Sync + KeyPathsOperable,
        V: Send + Sync,
        KeyPaths<T, V>: Send + Sync,
        F: Fn(B, &V) -> B + Send + Sync,
        B: Send + Sync + Clone,
    {
        let result = collection
            .par_iter()
            .map(|item| {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in par_map_keypath")
                });
                value
            })
            .fold(|| init.clone(), |acc, value| f(acc, value))
            .reduce(|| init, |acc, value| f(acc, &value));
        Ok(result)
    }
    
    /// Parallel group by keypath
    pub fn par_group_by_keypath<T, V, F>(
        collection: &[T],
        keypath: KeyPaths<T, V>,
        f: F,
    ) -> KeyPathResult<std::collections::HashMap<V, Vec<T>>>
    where
        T: Send + Sync + Clone,
        V: Send + Sync + std::hash::Hash + Eq + Clone,
        F: Fn(&V) -> V + Send + Sync,
    {
        let pairs: Vec<(V, T)> = collection
            .par_iter()
            .map(|item| {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in par_group_by_keypath")
                });
                let key = f(value);
                (key, item.clone())
            })
            .collect();
        
        let mut result = std::collections::HashMap::new();
        for (key, item) in pairs {
            result.entry(key).or_insert_with(Vec::new).push(item);
        }
        Ok(result)
    }
    
    /// Parallel partition by keypath
    pub fn par_partition_by_keypath<T, V, F>(
        collection: &[T],
        keypath: KeyPaths<T, V>,
        predicate: F,
    ) -> KeyPathResult<(Vec<T>, Vec<T>)>
    where
        T: Send + Sync + Clone + Copy,
        V: Send + Sync,
        KeyPaths<T, V>: Send + Sync,
        F: Fn(&V) -> bool + Send + Sync,
    {
        let (left, right): (Vec<T>, Vec<T>) = collection
            .par_iter()
            .partition(|item| {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in par_map_keypath")
                });
                predicate(value)
            });
        Ok((left, right))
    }
    
    /// Parallel sort by keypath
    pub fn par_sort_by_keypath<T, V, F>(
        collection: &mut [T],
        keypath: KeyPaths<T, V>,
        compare: F,
    ) -> KeyPathResult<()>
    where
        T: Send + Sync + KeyPathsOperable,
        V: Send + Sync,
        KeyPaths<T, V>: Send + Sync,
        F: Fn(&V, &V) -> std::cmp::Ordering + Send + Sync,
    {
        collection.par_sort_by(|a, b| {
            let a_val = a.get_at_keypath(&keypath).unwrap_or_else(|_| {
                panic!("KeyPath access failed in par_sort_by_keypath")
            });
            let b_val = b.get_at_keypath(&keypath).unwrap_or_else(|_| {
                panic!("KeyPath access failed in par_sort_by_keypath")
            });
            compare(a_val, b_val)
        });
        Ok(())
    }
    
    /// Parallel collect keypath values
    pub fn par_collect_keypath<T, V>(
        collection: &[T],
        keypath: KeyPaths<T, V>,
    ) -> KeyPathResult<Vec<V>>
    where
        T: Send + Sync,
        V: Send + Sync + Clone,
    {
        let result: Vec<V> = collection
            .par_iter()
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
        collection: &[T],
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
            .par_iter()
            .filter(|item| {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in par_map_keypath")
                });
                predicate(value)
            })
            .count();
        Ok(count)
    }
    
    /// Parallel any by keypath predicate
    pub fn par_any_by_keypath<T, V, F>(
        collection: &[T],
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
            .par_iter()
            .any(|item| {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in par_map_keypath")
                });
                predicate(value)
            });
        Ok(result)
    }
    
    /// Parallel all by keypath predicate
    pub fn par_all_by_keypath<T, V, F>(
        collection: &[T],
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
            .par_iter()
            .all(|item| {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in par_map_keypath")
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
        collection: &[T],
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
        let result = pool.install(|| {
            collection
                .par_iter()
                .map(|item| {
                    let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in par_map_keypath")
                });
                    operation(value)
                })
                .collect()
        });
        Ok(result)
    }
}

#[cfg(feature = "parallel")]
/// Parallel keypath operations with work stealing
pub mod parallel_work_stealing {
    use super::*;
    
    /// Parallel map with work stealing for large datasets
    pub fn par_map_keypath_work_stealing<T, V, F, R>(
        collection: &[T],
        keypath: KeyPaths<T, V>,
        f: F,
        chunk_size: usize,
    ) -> KeyPathResult<Vec<R>>
    where
        T: Send + Sync + KeyPathsOperable,
        V: Send + Sync,
        KeyPaths<T, V>: Send + Sync,
        F: Fn(&V) -> R + Send + Sync,
        R: Send,
    {
        let result: Vec<R> = collection
            .par_chunks(chunk_size)
            .flat_map(|chunk| {
                chunk
                    .par_iter()
                    .map(|item| {
                        let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in par_map_keypath")
                });
                        f(value)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        Ok(result)
    }
    
    /// Parallel reduce with work stealing
    pub fn par_reduce_keypath_work_stealing<T, V, F, B>(
        collection: &[T],
        keypath: KeyPaths<T, V>,
        init: B,
        f: F,
        chunk_size: usize,
    ) -> KeyPathResult<B>
    where
        T: Send + Sync + KeyPathsOperable,
        V: Send + Sync,
        KeyPaths<T, V>: Send + Sync,
        F: Fn(B, &V) -> B + Send + Sync,
        B: Send + Sync + Clone,
    {
        let result = collection
            .par_chunks(chunk_size)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|item| {
                        let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in par_map_keypath")
                });
                        value
                    })
                    .fold(init.clone(), |acc, value| f(acc, value))
            })
            .reduce(|| init, |acc, value| f(acc, &value));
        Ok(result)
    }
}

#[cfg(feature = "parallel")]
/// Parallel keypath operations with load balancing
pub mod parallel_load_balancing {
    use super::*;
    
    /// Parallel map with load balancing
    pub fn par_map_keypath_load_balanced<T, V, F, R>(
        collection: &[T],
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
            .par_iter()
            .with_min_len(1)
            .with_max_len(collection.len() / rayon::current_num_threads())
            .map(|item| {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in par_map_keypath")
                });
                f(value)
            })
            .collect();
        Ok(result)
    }
    
    /// Parallel filter with load balancing
    pub fn par_filter_keypath_load_balanced<T, V, F>(
        collection: &[T],
        keypath: KeyPaths<T, V>,
        predicate: F,
    ) -> KeyPathResult<Vec<T>>
    where
        T: Send + Sync + Clone,
        V: Send + Sync,
        F: Fn(&V) -> bool + Send + Sync,
    {
        let result: Vec<T> = collection
            .par_iter()
            .with_min_len(1)
            .with_max_len(collection.len() / rayon::current_num_threads())
            .filter(|item| {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in par_map_keypath")
                });
                predicate(value)
            })
            .cloned()
            .collect();
        Ok(result)
    }
}

#[cfg(not(feature = "parallel"))]
/// Placeholder module when parallel feature is not enabled
pub mod parallel_collections {
    use crate::error::KeyPathError;
    
    pub fn parallel_not_available() -> KeyPathError {
        KeyPathError::ParallelError {
            message: "Parallel operations require the 'parallel' feature to be enabled".to_string(),
        }
    }
}

#[cfg(not(feature = "parallel"))]
pub mod parallel_pools {
    use crate::error::KeyPathError;
    
    pub fn parallel_not_available() -> KeyPathError {
        KeyPathError::ParallelError {
            message: "Parallel pool operations require the 'parallel' feature to be enabled".to_string(),
        }
    }
}

#[cfg(not(feature = "parallel"))]
pub mod parallel_work_stealing {
    use crate::error::KeyPathError;
    
    pub fn parallel_not_available() -> KeyPathError {
        KeyPathError::ParallelError {
            message: "Parallel work stealing operations require the 'parallel' feature to be enabled".to_string(),
        }
    }
}

#[cfg(not(feature = "parallel"))]
pub mod parallel_load_balancing {
    use crate::error::KeyPathError;
    
    pub fn parallel_not_available() -> KeyPathError {
        KeyPathError::ParallelError {
            message: "Parallel load balancing operations require the 'parallel' feature to be enabled".to_string(),
        }
    }
}
