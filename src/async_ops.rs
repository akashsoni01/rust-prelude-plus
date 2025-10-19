//! Asynchronous operations for keypath-based functional programming

#[cfg(feature = "async")]
use {
    key_paths_core::KeyPaths,
    crate::error::{KeyPathResult, KeyPathError},
    crate::traits::KeyPathsOperable,
};

#[cfg(feature = "async")]
/// Asynchronous keypath operations for collections
pub mod async_collections {
    use super::*;
    
    /// Async map over collection with keypath
    pub async fn map_keypath_async<T, V, F, R>(
        collection: Vec<T>,
        keypath: KeyPaths<T, V>,
        f: F,
    ) -> KeyPathResult<Vec<R>>
    where
        T: Send + Sync + KeyPathsOperable,
        V: Send + Sync,
        KeyPaths<T, V>: Send + Sync,
        F: Fn(&V) -> R + Send + Sync + 'static,
        R: Send,
    {
        let result: Vec<R> = collection
            .into_iter()
            .map(|item| {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in map_keypath_async")
                });
                f(value)
            })
            .collect();
        Ok(result)
    }
    
    /// Async filter by keypath predicate
    pub async fn filter_by_keypath_async<T, V, F>(
        collection: Vec<T>,
        keypath: KeyPaths<T, V>,
        predicate: F,
    ) -> KeyPathResult<Vec<T>>
    where
        T: Send + Sync + KeyPathsOperable,
        V: Send + Sync,
        KeyPaths<T, V>: Send + Sync,
        F: Fn(&V) -> bool + Send + Sync + 'static,
    {
        let result: Vec<T> = collection
            .into_iter()
            .filter(|item| {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in filter_by_keypath_async")
                });
                predicate(value)
            })
            .collect();
        Ok(result)
    }
    
    /// Async find by keypath predicate
    pub async fn find_by_keypath_async<T, V, F>(
        collection: Vec<T>,
        keypath: KeyPaths<T, V>,
        predicate: F,
    ) -> KeyPathResult<Option<T>>
    where
        T: Send + Sync + KeyPathsOperable,
        V: Send + Sync,
        KeyPaths<T, V>: Send + Sync,
        F: Fn(&V) -> bool + Send + Sync + 'static,
    {
        let result = collection
            .into_iter()
            .find(|item| {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in find_by_keypath_async")
                });
                predicate(value)
            });
        Ok(result)
    }
    
    /// Async collect keypath values
    pub async fn collect_keypath_async<T, V>(
        collection: Vec<T>,
        keypath: KeyPaths<T, V>,
    ) -> KeyPathResult<Vec<V>>
    where
        T: Send + Sync + KeyPathsOperable,
        V: Send + Sync + Clone,
        KeyPaths<T, V>: Send + Sync,
    {
        let result: Vec<V> = collection
            .into_iter()
            .map(|item| {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in collect_keypath_async")
                });
                value.clone()
            })
            .collect();
        Ok(result)
    }
    
    /// Async count by keypath predicate
    pub async fn count_by_keypath_async<T, V, F>(
        collection: Vec<T>,
        keypath: KeyPaths<T, V>,
        predicate: F,
    ) -> KeyPathResult<usize>
    where
        T: Send + Sync + KeyPathsOperable,
        V: Send + Sync,
        KeyPaths<T, V>: Send + Sync,
        F: Fn(&V) -> bool + Send + Sync + 'static,
    {
        let count = collection
            .into_iter()
            .filter(|item| {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in count_by_keypath_async")
                });
                predicate(value)
            })
            .count();
        Ok(count)
    }
    
    /// Async any by keypath predicate
    pub async fn any_by_keypath_async<T, V, F>(
        collection: Vec<T>,
        keypath: KeyPaths<T, V>,
        predicate: F,
    ) -> KeyPathResult<bool>
    where
        T: Send + Sync + KeyPathsOperable,
        V: Send + Sync,
        KeyPaths<T, V>: Send + Sync,
        F: Fn(&V) -> bool + Send + Sync + 'static,
    {
        let result = collection
            .into_iter()
            .any(|item| {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in any_by_keypath_async")
                });
                predicate(value)
            });
        Ok(result)
    }
    
    /// Async all by keypath predicate
    pub async fn all_by_keypath_async<T, V, F>(
        collection: Vec<T>,
        keypath: KeyPaths<T, V>,
        predicate: F,
    ) -> KeyPathResult<bool>
    where
        T: Send + Sync + KeyPathsOperable,
        V: Send + Sync,
        KeyPaths<T, V>: Send + Sync,
        F: Fn(&V) -> bool + Send + Sync + 'static,
    {
        let result = collection
            .into_iter()
            .all(|item| {
                let value = item.get_at_keypath(&keypath).unwrap_or_else(|_| {
                    panic!("KeyPath access failed in all_by_keypath_async")
                });
                predicate(value)
            });
        Ok(result)
    }
}

#[cfg(all(feature = "async", feature = "serde"))]
/// Async operations with JSON serialization/deserialization
pub mod async_json {
    use super::*;
    use serde::{Deserialize, Serialize};
    
    /// Read and process JSON data with keypath
    pub async fn read_and_process_keypath<T, V, F, R>(
        json_data: &str,
        keypath: KeyPaths<T, V>,
        processor: F,
    ) -> KeyPathResult<Vec<R>>
    where
        T: Deserialize + Send + Sync + KeyPathsOperable,
        V: Send + Sync,
        KeyPaths<T, V>: Send + Sync,
        F: Fn(&V) -> R + Send + Sync + 'static,
        R: Send,
    {
        let data: Vec<T> = serde_json::from_str(json_data)
            .map_err(|e| KeyPathError::SerializationError {
                message: format!("Failed to deserialize JSON: {}", e),
            })?;
        
        async_collections::map_keypath_async(data, keypath, processor).await
    }
    
    /// Process data and write as JSON with keypath
    pub async fn process_and_write_keypath<T, V, F, R>(
        collection: Vec<T>,
        keypath: KeyPaths<T, V>,
        processor: F,
    ) -> KeyPathResult<String>
    where
        T: Send + Sync + KeyPathsOperable,
        V: Send + Sync,
        KeyPaths<T, V>: Send + Sync,
        F: Fn(&V) -> R + Send + Sync + 'static,
        R: Send + Serialize,
    {
        let results = async_collections::map_keypath_async(collection, keypath, processor).await?;
        
        let json = serde_json::to_string(&results)
            .map_err(|e| KeyPathError::SerializationError {
                message: format!("Failed to serialize to JSON: {}", e),
            })?;
        
        Ok(json)
    }
}

#[cfg(feature = "async")]
/// Async operations with HTTP requests
pub mod async_http {
    use super::*;
    
    /// Fetch data from URL and process with keypath
    pub async fn fetch_and_process_keypath<T, V, F, R>(
        url: &str,
        keypath: KeyPaths<T, V>,
        processor: F,
    ) -> KeyPathResult<Vec<R>>
    where
        T: serde::de::DeserializeOwned + Send + Sync + KeyPathsOperable,
        V: Send + Sync,
        KeyPaths<T, V>: Send + Sync,
        F: Fn(&V) -> R + Send + Sync + 'static,
        R: Send,
    {
        let response = reqwest::get(url).await
            .map_err(|e| KeyPathError::NetworkError {
                message: format!("Failed to fetch data: {}", e),
            })?;
        
        let data: Vec<T> = response.json().await
            .map_err(|e| KeyPathError::SerializationError {
                message: format!("Failed to deserialize response: {}", e),
            })?;
        
        async_collections::map_keypath_async(data, keypath, processor).await
    }
    
    /// Process data and send HTTP POST request
    pub async fn process_and_send_keypath<T, V, F, R>(
        collection: Vec<T>,
        keypath: KeyPaths<T, V>,
        processor: F,
        url: &str,
    ) -> KeyPathResult<reqwest::Response>
    where
        T: Send + Sync + KeyPathsOperable,
        V: Send + Sync,
        KeyPaths<T, V>: Send + Sync,
        F: Fn(&V) -> R + Send + Sync + 'static,
        R: Send + serde::Serialize,
    {
        let results = async_collections::map_keypath_async(collection, keypath, processor).await?;
        
        let client = reqwest::Client::new();
        let response = client.post(url)
            .json(&results)
            .send()
            .await
            .map_err(|e| KeyPathError::NetworkError {
                message: format!("Failed to send data: {}", e),
            })?;
        
        Ok(response)
    }
}