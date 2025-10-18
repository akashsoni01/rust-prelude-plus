use rust_prelude_plus::prelude::*;
use key_paths_derive::Keypath;
use std::sync::Arc;
use std::time::Instant;

#[cfg(feature = "async")]
use {
    tokio::time::{sleep, Duration},
    futures::stream,
    std::collections::HashMap,
};

#[derive(Keypath, Debug, Clone, PartialEq)]
struct User {
    id: u32,
    name: String,
    email: String,
    age: u32,
    is_active: bool,
    subscription_tier: String,
    last_login: String,
}

#[derive(Keypath, Debug, Clone, PartialEq)]
struct Post {
    id: u32,
    user_id: u32,
    title: String,
    content: String,
    created_at: String,
    likes: u32,
    tags: Vec<String>,
}

#[derive(Keypath, Debug, Clone, PartialEq)]
struct Comment {
    id: u32,
    post_id: u32,
    user_id: u32,
    content: String,
    created_at: String,
    likes: u32,
}

fn main() {
    println!("=== Async KeyPath Examples ===\n");

    #[cfg(feature = "async")]
    {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            // Create sample data
            let users = create_sample_users();
            let posts = create_sample_posts();
            let comments = create_sample_comments();

            // 1. Basic Async Operations
            demonstrate_basic_async_operations(&users).await;

            // 2. Async Collection Operations
            demonstrate_async_collection_operations(&users, &posts).await;

            // 3. Async I/O Operations
            demonstrate_async_io_operations(&users).await;

            // 4. Async Network Operations
            demonstrate_async_network_operations(&posts).await;

            // 5. Async Database Operations
            demonstrate_async_database_operations(&users, &posts, &comments).await;

            // 6. Performance Comparison
            demonstrate_async_performance_comparison(&users, &posts).await;
        });
    }

    #[cfg(not(feature = "async"))]
    {
        println!("Async features are not enabled. Run with --features async to see examples.");
    }
}

#[cfg(feature = "async")]
fn create_sample_users() -> Vec<Arc<User>> {
    (1..=100)
        .map(|i| {
            Arc::new(User {
                id: i,
                name: format!("User {}", i),
                email: format!("user{}@example.com", i),
                age: 20 + (i % 50),
                is_active: i % 3 != 0,
                subscription_tier: if i % 4 == 0 { "premium" } else if i % 4 == 1 { "basic" } else { "free" }.to_string(),
                last_login: format!("2024-01-{:02}", (i % 30) + 1),
            })
        })
        .collect()
}

#[cfg(feature = "async")]
fn create_sample_posts() -> Vec<Arc<Post>> {
    (1..=200)
        .map(|i| {
            Arc::new(Post {
                id: i,
                user_id: (i % 100) + 1,
                title: format!("Post {}", i),
                content: format!("This is the content of post {}. It contains some interesting information.", i),
                created_at: format!("2024-01-{:02}", (i % 30) + 1),
                likes: i % 100,
                tags: vec![format!("tag{}", i % 10), format!("category{}", i % 5)],
            })
        })
        .collect()
}

#[cfg(feature = "async")]
fn create_sample_comments() -> Vec<Arc<Comment>> {
    (1..=500)
        .map(|i| {
            Arc::new(Comment {
                id: i,
                post_id: (i % 200) + 1,
                user_id: (i % 100) + 1,
                content: format!("This is comment {} on post {}.", i, (i % 200) + 1),
                created_at: format!("2024-01-{:02}", (i % 30) + 1),
                likes: i % 50,
            })
        })
        .collect()
}

#[cfg(feature = "async")]
async fn demonstrate_basic_async_operations(users: &[Arc<User>]) {
    println!("1. BASIC ASYNC OPERATIONS:");
    
    // Async map operation
    let start = Instant::now();
    let user_stream = stream::iter(users);
    let user_names: Vec<String> = user_stream
        .keypath_ops()
        .map_keypath(User::name(), |name| name.clone())
        .await
        .collect()
        .await;
    let map_time = start.elapsed();
    println!("   Async map - {} users in {:?}", user_names.len(), map_time);

    // Async filter operation
    let start = Instant::now();
    let active_user_stream = stream::iter(users);
    let active_users: Vec<Arc<User>> = active_user_stream
        .keypath_ops()
        .filter_by_keypath(User::is_active(), |&active| active)
        .await
        .collect()
        .await;
    let filter_time = start.elapsed();
    println!("   Async filter - {} active users in {:?}", active_users.len(), filter_time);

    // Async find operation
    let start = Instant::now();
    let find_stream = stream::iter(users);
    let premium_user = find_stream
        .keypath_ops()
        .find_by_keypath(User::subscription_tier(), |tier| tier == "premium")
        .await
        .unwrap();
    let find_time = start.elapsed();
    println!("   Async find - found {} in {:?}", premium_user.name, find_time);

    // Async fold operation
    let start = Instant::now();
    let fold_stream = stream::iter(users);
    let total_age: u32 = fold_stream
        .keypath_ops()
        .fold_keypath(User::age(), 0, |acc, &age| acc + age)
        .await
        .unwrap();
    let fold_time = start.elapsed();
    println!("   Async fold - total age {} in {:?}", total_age, fold_time);

    println!();
}

#[cfg(feature = "async")]
async fn demonstrate_async_collection_operations(users: &[Arc<User>], posts: &[Arc<Post>]) {
    println!("2. ASYNC COLLECTION OPERATIONS:");
    
    // Async map over collection
    let start = Instant::now();
    let user_emails = async_collections::map_keypath_async(
        users.to_vec(),
        User::email(),
        |email| email.clone()
    ).await.unwrap();
    let map_time = start.elapsed();
    println!("   Async map collection - {} emails in {:?}", user_emails.len(), map_time);

    // Async filter collection
    let start = Instant::now();
    let premium_users = async_collections::filter_by_keypath_async(
        users.to_vec(),
        User::subscription_tier(),
        |tier| tier == "premium"
    ).await.unwrap();
    let filter_time = start.elapsed();
    println!("   Async filter collection - {} premium users in {:?}", premium_users.len(), filter_time);

    // Async find in collection
    let start = Instant::now();
    let young_user = async_collections::find_by_keypath_async(
        users.to_vec(),
        User::age(),
        |&age| age < 25
    ).await.unwrap();
    let find_time = start.elapsed();
    if let Some(user) = young_user {
        println!("   Async find collection - found {} in {:?}", user.name, find_time);
    }

    // Async fold over collection
    let start = Instant::now();
    let total_posts = async_collections::fold_keypath_async(
        posts.to_vec(),
        Post::likes(),
        0,
        |acc, &likes| acc + likes
    ).await.unwrap();
    let fold_time = start.elapsed();
    println!("   Async fold collection - total likes {} in {:?}", total_posts, fold_time);

    println!();
}

#[cfg(feature = "async")]
async fn demonstrate_async_io_operations(users: &[Arc<User>]) {
    println!("3. ASYNC I/O OPERATIONS:");
    
    // Simulate file I/O with async operations
    let start = Instant::now();
    
    // Simulate reading from file
    sleep(Duration::from_millis(10)).await;
    
    // Process data with keypaths
    let processed_users = async_collections::map_keypath_async(
        users.to_vec(),
        User::name(),
        |name| name.to_uppercase()
    ).await.unwrap();
    
    // Simulate writing to file
    sleep(Duration::from_millis(10)).await;
    
    let io_time = start.elapsed();
    println!("   Async I/O - processed {} users in {:?}", processed_users.len(), io_time);

    // Batch processing
    let start = Instant::now();
    let batch_size = 20;
    let batches: Vec<Vec<Arc<User>>> = users.chunks(batch_size).map(|chunk| chunk.to_vec()).collect();
    
    let mut all_results = Vec::new();
    for batch in batches {
        let batch_result = async_collections::map_keypath_async(
            batch,
            User::email(),
            |email| email.clone()
        ).await.unwrap();
        all_results.extend(batch_result);
    }
    
    let batch_time = start.elapsed();
    println!("   Async batch processing - processed {} users in {:?}", all_results.len(), batch_time);

    println!();
}

#[cfg(feature = "async")]
async fn demonstrate_async_network_operations(posts: &[Arc<Post>]) {
    println!("4. ASYNC NETWORK OPERATIONS:");
    
    // Simulate network I/O with async operations
    let start = Instant::now();
    
    // Simulate API call delay
    sleep(Duration::from_millis(50)).await;
    
    // Process posts with keypaths
    let popular_posts = async_collections::filter_by_keypath_async(
        posts.to_vec(),
        Post::likes(),
        |&likes| likes > 50
    ).await.unwrap();
    
    // Simulate sending data to API
    sleep(Duration::from_millis(50)).await;
    
    let network_time = start.elapsed();
    println!("   Async network - processed {} popular posts in {:?}", popular_posts.len(), network_time);

    // Concurrent network operations
    let start = Instant::now();
    
    let futures: Vec<_> = posts.chunks(50).map(|chunk| {
        let chunk = chunk.to_vec();
        async move {
            sleep(Duration::from_millis(20)).await;
            async_collections::map_keypath_async(
                chunk,
                Post::title(),
                |title| title.clone()
            ).await.unwrap()
        }
    }).collect();
    
    let results: Vec<Vec<String>> = futures::future::join_all(futures).await;
    let total_titles: usize = results.iter().map(|r| r.len()).sum();
    
    let concurrent_time = start.elapsed();
    println!("   Async concurrent network - processed {} titles in {:?}", total_titles, concurrent_time);

    println!();
}

#[cfg(feature = "async")]
async fn demonstrate_async_database_operations(users: &[Arc<User>], posts: &[Arc<Post>], comments: &[Arc<Comment>]) {
    println!("5. ASYNC DATABASE OPERATIONS:");
    
    // Simulate database operations
    let start = Instant::now();
    
    // Simulate database query delay
    sleep(Duration::from_millis(30)).await;
    
    // Process database results with keypaths
    let active_users = async_collections::filter_by_keypath_async(
        users.to_vec(),
        User::is_active(),
        |&active| active
    ).await.unwrap();
    
    let db_time = start.elapsed();
    println!("   Async database - processed {} active users in {:?}", active_users.len(), db_time);

    // Batch database operations
    let start = Instant::now();
    
    let batch_size = 25;
    let user_batches: Vec<Vec<Arc<User>>> = users.chunks(batch_size).map(|chunk| chunk.to_vec()).collect();
    
    let mut all_active_users = Vec::new();
    for batch in user_batches {
        sleep(Duration::from_millis(10)).await; // Simulate DB query
        let batch_result = async_collections::filter_by_keypath_async(
            batch,
            User::is_active(),
            |&active| active
        ).await.unwrap();
        all_active_users.extend(batch_result);
    }
    
    let batch_db_time = start.elapsed();
    println!("   Async batch database - processed {} active users in {:?}", all_active_users.len(), batch_db_time);

    // Complex database operations
    let start = Instant::now();
    
    // Simulate complex query
    sleep(Duration::from_millis(40)).await;
    
    // Process multiple collections
    let user_emails = async_collections::map_keypath_async(
        users.to_vec(),
        User::email(),
        |email| email.clone()
    ).await.unwrap();
    
    let post_titles = async_collections::map_keypath_async(
        posts.to_vec(),
        Post::title(),
        |title| title.clone()
    ).await.unwrap();
    
    let comment_contents = async_collections::map_keypath_async(
        comments.to_vec(),
        Comment::content(),
        |content| content.clone()
    ).await.unwrap();
    
    let complex_time = start.elapsed();
    println!("   Async complex database - processed {} emails, {} titles, {} comments in {:?}", 
             user_emails.len(), post_titles.len(), comment_contents.len(), complex_time);

    println!();
}

#[cfg(feature = "async")]
async fn demonstrate_async_performance_comparison(users: &[Arc<User>], posts: &[Arc<Post>]) {
    println!("6. ASYNC PERFORMANCE COMPARISON:");
    
    let iterations = 5;
    
    // Sequential async processing
    let start = Instant::now();
    for _ in 0..iterations {
        let _result = async_collections::map_keypath_async(
            users.to_vec(),
            User::name(),
            |name| name.clone()
        ).await.unwrap();
    }
    let sequential_async_time = start.elapsed();
    
    // Concurrent async processing
    let start = Instant::now();
    let futures: Vec<_> = (0..iterations).map(|_| {
        async_collections::map_keypath_async(
            users.to_vec(),
            User::name(),
            |name| name.clone()
        )
    }).collect();
    
    let _results: Vec<Vec<String>> = futures::future::join_all(futures).await
        .into_iter()
        .map(|r| r.unwrap())
        .collect();
    let concurrent_async_time = start.elapsed();
    
    println!("   Sequential async processing: {:?} ({} iterations)", sequential_async_time, iterations);
    println!("   Concurrent async processing: {:?} ({} iterations)", concurrent_async_time, iterations);
    println!("   Async concurrency speedup: {:.2}x", 
             sequential_async_time.as_secs_f64() / concurrent_async_time.as_secs_f64());
    
    // Stream processing comparison
    let start = Instant::now();
    let stream_result: Vec<String> = stream::iter(users)
        .keypath_ops()
        .map_keypath(User::name(), |name| name.clone())
        .await
        .collect()
        .await;
    let stream_time = start.elapsed();
    
    let start = Instant::now();
    let collection_result = async_collections::map_keypath_async(
        users.to_vec(),
        User::name(),
        |name| name.clone()
    ).await.unwrap();
    let collection_time = start.elapsed();
    
    println!("   Stream processing: {:?} - {} results", stream_time, stream_result.len());
    println!("   Collection processing: {:?} - {} results", collection_time, collection_result.len());
    println!("   Stream vs Collection: {:.2}x", 
             collection_time.as_secs_f64() / stream_time.as_secs_f64());
    
    // Complex operation comparison
    let start = Instant::now();
    let complex_stream: Vec<String> = stream::iter(users)
        .keypath_ops()
        .filter_by_keypath(User::is_active(), |&active| active)
        .filter_by_keypath(User::age(), |&age| age >= 25)
        .map_keypath(User::name(), |name| name.to_uppercase())
        .await
        .collect()
        .await;
    let complex_stream_time = start.elapsed();
    
    let start = Instant::now();
    let active_users = async_collections::filter_by_keypath_async(
        users.to_vec(),
        User::is_active(),
        |&active| active
    ).await.unwrap();
    let adult_users = async_collections::filter_by_keypath_async(
        active_users,
        User::age(),
        |&age| age >= 25
    ).await.unwrap();
    let complex_collection = async_collections::map_keypath_async(
        adult_users,
        User::name(),
        |name| name.to_uppercase()
    ).await.unwrap();
    let complex_collection_time = start.elapsed();
    
    println!("   Complex stream processing: {:?} - {} results", complex_stream_time, complex_stream.len());
    println!("   Complex collection processing: {:?} - {} results", complex_collection_time, complex_collection.len());
    println!("   Complex operation speedup: {:.2}x", 
             complex_collection_time.as_secs_f64() / complex_stream_time.as_secs_f64());
    
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_basic_async_operations() {
        let users = create_sample_users();
        
        // Test async map
        let user_stream = stream::iter(&users);
        let user_names: Vec<String> = user_stream
            .keypath_ops()
            .map_keypath(User::name(), |name| name.clone())
            .await
            .collect()
            .await;
        assert_eq!(user_names.len(), 100);
        
        // Test async filter
        let active_user_stream = stream::iter(&users);
        let active_users: Vec<Arc<User>> = active_user_stream
            .keypath_ops()
            .filter_by_keypath(User::is_active(), |&active| active)
            .await
            .collect()
            .await;
        assert!(active_users.len() > 0);
        
        // Test async find
        let find_stream = stream::iter(&users);
        let premium_user = find_stream
            .keypath_ops()
            .find_by_keypath(User::subscription_tier(), |tier| tier == "premium")
            .await
            .unwrap();
        assert_eq!(premium_user.subscription_tier, "premium");
        
        // Test async fold
        let fold_stream = stream::iter(&users);
        let total_age: u32 = fold_stream
            .keypath_ops()
            .fold_keypath(User::age(), 0, |acc, &age| acc + age)
            .await
            .unwrap();
        assert!(total_age > 0);
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_async_collection_operations() {
        let users = create_sample_users();
        
        // Test async map collection
        let user_emails = async_collections::map_keypath_async(
            users.to_vec(),
            User::email(),
            |email| email.clone()
        ).await.unwrap();
        assert_eq!(user_emails.len(), 100);
        
        // Test async filter collection
        let premium_users = async_collections::filter_by_keypath_async(
            users.to_vec(),
            User::subscription_tier(),
            |tier| tier == "premium"
        ).await.unwrap();
        assert!(premium_users.len() > 0);
        
        // Test async find collection
        let young_user = async_collections::find_by_keypath_async(
            users.to_vec(),
            User::age(),
            |&age| age < 25
        ).await.unwrap();
        assert!(young_user.is_some());
        
        // Test async fold collection
        let total_age = async_collections::fold_keypath_async(
            users.to_vec(),
            User::age(),
            0,
            |acc, &age| acc + age
        ).await.unwrap();
        assert!(total_age > 0);
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_async_io_operations() {
        let users = create_sample_users();
        
        // Test async I/O simulation
        sleep(Duration::from_millis(1)).await;
        
        let processed_users = async_collections::map_keypath_async(
            users.to_vec(),
            User::name(),
            |name| name.to_uppercase()
        ).await.unwrap();
        
        assert_eq!(processed_users.len(), 100);
        assert!(processed_users[0].contains("USER"));
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_async_database_operations() {
        let users = create_sample_users();
        
        // Test async database simulation
        sleep(Duration::from_millis(1)).await;
        
        let active_users = async_collections::filter_by_keypath_async(
            users.to_vec(),
            User::is_active(),
            |&active| active
        ).await.unwrap();
        
        assert!(active_users.len() > 0);
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_async_performance() {
        let users = create_sample_users();
        
        // Test concurrent processing
        let futures: Vec<_> = (0..3).map(|_| {
            async_collections::map_keypath_async(
                users.to_vec(),
                User::name(),
                |name| name.clone()
            )
        }).collect();
        
        let results: Vec<Vec<String>> = futures::future::join_all(futures).await
            .into_iter()
            .map(|r| r.unwrap())
            .collect();
        
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].len(), 100);
    }
}
