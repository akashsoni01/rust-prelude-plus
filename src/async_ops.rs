//! Async operations for keypath-based functional programming

#[cfg(feature = "async")]
use {
    futures::future::{Future, FutureExt},
    futures::stream::{Stream, StreamExt},
    key_paths_core::KeyPaths,
    crate::error::{KeyPathResult, KeyPathError},
    std::pin::Pin,
    std::task::{Context, Poll},
};

#[cfg(feature = "async")]
/// Async iterator for keypath operations
pub struct AsyncKeyPathIterator<T, S> {
    stream: S,
    _phantom: std::marker::PhantomData<T>,
}

#[cfg(feature = "async")]
impl<T, S> AsyncKeyPathIterator<T, S>
where
    S: Stream<Item = T> + Unpin,
{
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            _phantom: std::marker::PhantomData,
        }
    }
    
    /// Map over keypath values asynchronously
    pub async fn map_keypath<V, F, R>(
        self,
        keypath: impl KeyPaths<T, V>,
        f: F,
    ) -> AsyncKeyPathIterator<R, impl Stream<Item = R>>
    where
        F: Fn(&V) -> R + Send + Sync + 'static,
    {
        let mapped_stream = self.stream.map(move |item| {
            let value = keypath.get(&item);
            f(value)
        });
        AsyncKeyPathIterator::new(mapped_stream)
    }
    
    /// Filter by keypath predicate asynchronously
    pub async fn filter_by_keypath<V, F>(
        self,
        keypath: impl KeyPaths<T, V>,
        predicate: F,
    ) -> AsyncKeyPathIterator<T, impl Stream<Item = T>>
    where
        F: Fn(&V) -> bool + Send + Sync + 'static,
    {
        let filtered_stream = self.stream.filter(move |item| {
            let value = keypath.get(item);
            futures::future::ready(predicate(value))
        });
        AsyncKeyPathIterator::new(filtered_stream)
    }
    
    /// Find element by keypath predicate asynchronously
    pub async fn find_by_keypath<V, F>(
        self,
        keypath: impl KeyPaths<T, V>,
        predicate: F,
    ) -> KeyPathResult<Option<T>>
    where
        F: Fn(&V) -> bool + Send + Sync + 'static,
    {
        let mut stream = self.stream;
        while let Some(item) = stream.next().await {
            let value = keypath.get(&item);
            if predicate(value) {
                return Ok(Some(item));
            }
        }
        Ok(None)
    }
    
    /// Fold over keypath values asynchronously
    pub async fn fold_keypath<V, F, B>(
        self,
        keypath: impl KeyPaths<T, V>,
        init: B,
        f: F,
    ) -> KeyPathResult<B>
    where
        F: Fn(B, &V) -> B + Send + Sync + 'static,
        B: Send,
    {
        let mut acc = init;
        let mut stream = self.stream;
        while let Some(item) = stream.next().await {
            let value = keypath.get(&item);
            acc = f(acc, value);
        }
        Ok(acc)
    }
    
    /// Collect into a vector asynchronously
    pub async fn collect<B: FromIterator<T>>(self) -> B {
        self.stream.collect().await
    }
}

#[cfg(feature = "async")]
/// Extension trait for async streams with keypath operations
pub trait AsyncKeyPathStreamExt<T>: Stream<Item = T> {
    /// Convert to async keypath iterator
    fn keypath_ops(self) -> AsyncKeyPathIterator<T, Self>
    where
        Self: Sized,
    {
        AsyncKeyPathIterator::new(self)
    }
}

#[cfg(feature = "async")]
impl<T, S> AsyncKeyPathStreamExt<T> for S where S: Stream<Item = T> {}

#[cfg(feature = "async")]
/// Async keypath operations for collections
pub mod async_collections {
    use super::*;
    use futures::stream;
    
    /// Async map over collection with keypath
    pub async fn map_keypath_async<T, V, F, R>(
        collection: Vec<T>,
        keypath: impl KeyPaths<T, V>,
        f: F,
    ) -> KeyPathResult<Vec<R>>
    where
        F: Fn(&V) -> R + Send + Sync + 'static,
    {
        let stream = stream::iter(collection);
        let result: Vec<R> = stream
            .keypath_ops()
            .map_keypath(keypath, f)
            .await
            .collect()
            .await;
        Ok(result)
    }
    
    /// Async filter collection by keypath
    pub async fn filter_by_keypath_async<T, V, F>(
        collection: Vec<T>,
        keypath: impl KeyPaths<T, V>,
        predicate: F,
    ) -> KeyPathResult<Vec<T>>
    where
        F: Fn(&V) -> bool + Send + Sync + 'static,
    {
        let stream = stream::iter(collection);
        let result: Vec<T> = stream
            .keypath_ops()
            .filter_by_keypath(keypath, predicate)
            .await
            .collect()
            .await;
        Ok(result)
    }
    
    /// Async find in collection by keypath
    pub async fn find_by_keypath_async<T, V, F>(
        collection: Vec<T>,
        keypath: impl KeyPaths<T, V>,
        predicate: F,
    ) -> KeyPathResult<Option<T>>
    where
        F: Fn(&V) -> bool + Send + Sync + 'static,
    {
        let stream = stream::iter(collection);
        stream
            .keypath_ops()
            .find_by_keypath(keypath, predicate)
            .await
    }
    
    /// Async fold over collection with keypath
    pub async fn fold_keypath_async<T, V, F, B>(
        collection: Vec<T>,
        keypath: impl KeyPaths<T, V>,
        init: B,
        f: F,
    ) -> KeyPathResult<B>
    where
        F: Fn(B, &V) -> B + Send + Sync + 'static,
        B: Send,
    {
        let stream = stream::iter(collection);
        stream
            .keypath_ops()
            .fold_keypath(keypath, init, f)
            .await
    }
}

#[cfg(feature = "async")]
/// Async keypath operations with I/O
pub mod async_io {
    use super::*;
    use tokio::fs;
    use tokio::io::{AsyncRead, AsyncWrite};
    
    /// Read data from file and apply keypath operations
    pub async fn read_and_process_keypath<T, V, F, R>(
        file_path: &str,
        keypath: impl KeyPaths<T, V>,
        processor: F,
    ) -> KeyPathResult<Vec<R>>
    where
        T: serde::de::DeserializeOwned,
        F: Fn(&V) -> R + Send + Sync + 'static,
    {
        let content = fs::read_to_string(file_path).await
            .map_err(|e| KeyPathError::AsyncError {
                message: format!("Failed to read file: {}", e),
            })?;
        
        let data: Vec<T> = serde_json::from_str(&content)
            .map_err(|e| KeyPathError::AsyncError {
                message: format!("Failed to parse JSON: {}", e),
            })?;
        
        async_collections::map_keypath_async(data, keypath, processor).await
    }
    
    /// Write processed data to file
    pub async fn process_and_write_keypath<T, V, F, R>(
        data: Vec<T>,
        keypath: impl KeyPaths<T, V>,
        processor: F,
        file_path: &str,
    ) -> KeyPathResult<()>
    where
        R: serde::Serialize,
        F: Fn(&V) -> R + Send + Sync + 'static,
    {
        let processed = async_collections::map_keypath_async(data, keypath, processor).await?;
        
        let json = serde_json::to_string_pretty(&processed)
            .map_err(|e| KeyPathError::AsyncError {
                message: format!("Failed to serialize: {}", e),
            })?;
        
        fs::write(file_path, json).await
            .map_err(|e| KeyPathError::AsyncError {
                message: format!("Failed to write file: {}", e),
            })?;
        
        Ok(())
    }
}

#[cfg(feature = "async")]
/// Async keypath operations with network I/O
pub mod async_network {
    use super::*;
    use reqwest::Client;
    
    /// Fetch data from API and apply keypath operations
    pub async fn fetch_and_process_keypath<T, V, F, R>(
        url: &str,
        keypath: impl KeyPaths<T, V>,
        processor: F,
    ) -> KeyPathResult<Vec<R>>
    where
        T: serde::de::DeserializeOwned,
        F: Fn(&V) -> R + Send + Sync + 'static,
    {
        let client = Client::new();
        let response = client.get(url).send().await
            .map_err(|e| KeyPathError::AsyncError {
                message: format!("Failed to fetch data: {}", e),
            })?;
        
        let data: Vec<T> = response.json().await
            .map_err(|e| KeyPathError::AsyncError {
                message: format!("Failed to parse JSON: {}", e),
            })?;
        
        async_collections::map_keypath_async(data, keypath, processor).await
    }
    
    /// Process data and send to API
    pub async fn process_and_send_keypath<T, V, F, R>(
        data: Vec<T>,
        keypath: impl KeyPaths<T, V>,
        processor: F,
        url: &str,
    ) -> KeyPathResult<()>
    where
        R: serde::Serialize,
        F: Fn(&V) -> R + Send + Sync + 'static,
    {
        let processed = async_collections::map_keypath_async(data, keypath, processor).await?;
        
        let client = Client::new();
        let response = client.post(url)
            .json(&processed)
            .send()
            .await
            .map_err(|e| KeyPathError::AsyncError {
                message: format!("Failed to send data: {}", e),
            })?;
        
        if !response.status().is_success() {
            return Err(KeyPathError::AsyncError {
                message: format!("API request failed with status: {}", response.status()),
            });
        }
        
        Ok(())
    }
}

#[cfg(feature = "async")]
/// Async keypath operations with database
pub mod async_database {
    use super::*;
    
    /// Process database results with keypath operations
    pub async fn process_db_results_keypath<T, V, F, R>(
        results: Vec<T>,
        keypath: impl KeyPaths<T, V>,
        processor: F,
    ) -> KeyPathResult<Vec<R>>
    where
        F: Fn(&V) -> R + Send + Sync + 'static,
    {
        async_collections::map_keypath_async(results, keypath, processor).await
    }
    
    /// Batch process database operations
    pub async fn batch_process_keypath<T, V, F, R>(
        data: Vec<T>,
        keypath: impl KeyPaths<T, V>,
        processor: F,
        batch_size: usize,
    ) -> KeyPathResult<Vec<R>>
    where
        F: Fn(&V) -> R + Send + Sync + 'static,
    {
        let mut results = Vec::new();
        let mut stream = futures::stream::iter(data);
        
        while let Some(batch) = stream.next().await {
            let batch_results = async_collections::map_keypath_async(
                vec![batch],
                keypath,
                &processor,
            ).await?;
            results.extend(batch_results);
        }
        
        Ok(results)
    }
}

#[cfg(not(feature = "async"))]
/// Placeholder module when async feature is not enabled
pub mod async_collections {
    use crate::error::KeyPathError;
    
    pub fn async_not_available() -> KeyPathError {
        KeyPathError::AsyncError {
            message: "Async operations require the 'async' feature to be enabled".to_string(),
        }
    }
}

#[cfg(not(feature = "async"))]
pub mod async_io {
    use crate::error::KeyPathError;
    
    pub fn async_not_available() -> KeyPathError {
        KeyPathError::AsyncError {
            message: "Async I/O operations require the 'async' feature to be enabled".to_string(),
        }
    }
}

#[cfg(not(feature = "async"))]
pub mod async_network {
    use crate::error::KeyPathError;
    
    pub fn async_not_available() -> KeyPathError {
        KeyPathError::AsyncError {
            message: "Async network operations require the 'async' feature to be enabled".to_string(),
        }
    }
}

#[cfg(not(feature = "async"))]
pub mod async_database {
    use crate::error::KeyPathError;
    
    pub fn async_not_available() -> KeyPathError {
        KeyPathError::AsyncError {
            message: "Async database operations require the 'async' feature to be enabled".to_string(),
        }
    }
}
