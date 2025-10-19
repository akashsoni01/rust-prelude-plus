//! Examples demonstrating asynchronous keypath operations

use key_paths_derive::Keypath;
use rust_prelude_plus::async_ops::async_collections;
use std::sync::Arc;

/// Example data structure for demonstrating async keypath operations
#[derive(Keypath, Debug, Clone, PartialEq)]
pub struct User {
    pub name: String,
    pub email: String,
    pub age: u32,
    pub is_active: bool,
    pub subscription_tier: String,
}

#[derive(Keypath, Debug, Clone, PartialEq)]
pub struct Post {
    pub title: String,
    pub content: String,
    pub author_id: u32,
    pub likes: u32,
    pub published: bool,
}

#[tokio::main]
async fn main() {
    println!("=== Async KeyPath Examples ===");
    
    // Create sample data
    let users = create_sample_users();
    let posts = create_sample_posts();
    
    // Test async operations
    test_async_operations(&users).await;
    test_async_collections(&users, &posts).await;
    test_async_performance(&users).await;
    
    println!("\n=== All async examples completed successfully! ===");
}

fn create_sample_users() -> Vec<Arc<User>> {
    vec![
        Arc::new(User {
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
            age: 28,
            is_active: true,
            subscription_tier: "premium".to_string(),
        }),
        Arc::new(User {
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
            age: 35,
            is_active: true,
            subscription_tier: "basic".to_string(),
        }),
        Arc::new(User {
            name: "Charlie".to_string(),
            email: "charlie@example.com".to_string(),
            age: 22,
            is_active: false,
            subscription_tier: "free".to_string(),
        }),
        Arc::new(User {
            name: "Diana".to_string(),
            email: "diana@example.com".to_string(),
            age: 31,
            is_active: true,
            subscription_tier: "premium".to_string(),
        }),
    ]
}

fn create_sample_posts() -> Vec<Arc<Post>> {
    vec![
        Arc::new(Post {
            title: "Rust Async Programming".to_string(),
            content: "A comprehensive guide to async programming in Rust...".to_string(),
            author_id: 1,
            likes: 42,
            published: true,
        }),
        Arc::new(Post {
            title: "KeyPath Operations".to_string(),
            content: "Understanding keypath-based functional programming...".to_string(),
            author_id: 2,
            likes: 28,
            published: true,
        }),
        Arc::new(Post {
            title: "Draft Post".to_string(),
            content: "This is a draft post...".to_string(),
            author_id: 1,
            likes: 0,
            published: false,
        }),
    ]
}

async fn test_async_operations(users: &[Arc<User>]) {
    println!("\n=== Async Operations ===");
    
    // Convert Arc<User> to User for async operations
    let users_owned: Vec<User> = users.iter().map(|u| (**u).clone()).collect();
    
    // Async map - get user names
    match async_collections::map_keypath_async(
        users_owned.clone(),
        User::name(),
        |name: &String| name.clone(),
    ).await {
        Ok(names) => println!("User names: {:?}", names),
        Err(e) => println!("Error in async map: {}", e),
    }
    
    // Async filter - get active users
    match async_collections::filter_by_keypath_async(
        users_owned.clone(),
        User::is_active(),
        |&is_active| is_active,
    ).await {
        Ok(active_users) => println!("Active users count: {}", active_users.len()),
        Err(e) => println!("Error in async filter: {}", e),
    }
    
    // Async find - find premium user
    match async_collections::find_by_keypath_async(
        users_owned.clone(),
        User::subscription_tier(),
        |tier| tier == "premium",
    ).await {
        Ok(Some(user)) => println!("Found premium user: {}", user.name),
        Ok(None) => println!("No premium user found"),
        Err(e) => println!("Error in async find: {}", e),
    }
    
    // Async collect - get all emails
    match async_collections::collect_keypath_async(
        users_owned.clone(),
        User::email(),
    ).await {
        Ok(emails) => println!("All emails: {:?}", emails),
        Err(e) => println!("Error in async collect: {}", e),
    }
    
    // Async count - count premium users
    match async_collections::count_by_keypath_async(
        users_owned.clone(),
        User::subscription_tier(),
        |tier| tier == "premium",
    ).await {
        Ok(count) => println!("Premium users count: {}", count),
        Err(e) => println!("Error in async count: {}", e),
    }
    
    // Async any - check if any young users
    match async_collections::any_by_keypath_async(
        users_owned.clone(),
        User::age(),
        |&age| age < 25,
    ).await {
        Ok(has_young) => println!("Has young users: {}", has_young),
        Err(e) => println!("Error in async any: {}", e),
    }
    
    // Async all - check if all users have emails
    match async_collections::all_by_keypath_async(
        users_owned,
        User::email(),
        |email| !email.is_empty(),
    ).await {
        Ok(all_have_emails) => println!("All users have emails: {}", all_have_emails),
        Err(e) => println!("Error in async all: {}", e),
    }
}

async fn test_async_collections(users: &[Arc<User>], posts: &[Arc<Post>]) {
    println!("\n=== Async Collections ===");
    
    // Convert Arc<User> to User for async operations
    let users_owned: Vec<User> = users.iter().map(|u| (**u).clone()).collect();
    let posts_owned: Vec<Post> = posts.iter().map(|p| (**p).clone()).collect();
    
    // Process users
    match async_collections::map_keypath_async(
        users_owned.clone(),
        User::name(),
        |name: &String| name.to_uppercase(),
    ).await {
        Ok(uppercase_names) => println!("Uppercase names: {:?}", uppercase_names),
        Err(e) => println!("Error processing users: {}", e),
    }
    
    // Process posts
    match async_collections::filter_by_keypath_async(
        posts_owned.clone(),
        Post::published(),
        |&published| published,
    ).await {
        Ok(published_posts) => println!("Published posts count: {}", published_posts.len()),
        Err(e) => println!("Error filtering posts: {}", e),
    }
    
    // Get popular posts
    match async_collections::filter_by_keypath_async(
        posts_owned,
        Post::likes(),
        |&likes| likes > 20,
    ).await {
        Ok(popular_posts) => {
            println!("Popular posts count: {}", popular_posts.len());
            for post in popular_posts {
                println!("  - {} ({} likes)", post.title, post.likes);
            }
        }
        Err(e) => println!("Error getting popular posts: {}", e),
    }
}

async fn test_async_performance(users: &[Arc<User>]) {
    println!("\n=== Async Performance ===");
    
    // Convert Arc<User> to User for async operations
    let users_owned: Vec<User> = users.iter().map(|u| (**u).clone()).collect();
    
    // Sequential processing
    let start = std::time::Instant::now();
    let sequential_result: Vec<String> = users_owned
        .iter()
        .filter(|u| u.is_active)
        .filter(|u| u.age >= 25)
        .map(|u| u.name.to_uppercase())
        .collect();
    let sequential_time = start.elapsed();
    
    // Async processing
    let start = std::time::Instant::now();
    let async_result = match async_collections::filter_by_keypath_async(
        users_owned.clone(),
        User::is_active(),
        |&is_active| is_active,
    ).await {
        Ok(active_users) => {
            match async_collections::filter_by_keypath_async(
                active_users,
                User::age(),
                |&age| age >= 25,
            ).await {
                Ok(adult_active_users) => {
                    match async_collections::map_keypath_async(
                        adult_active_users,
                        User::name(),
                        |name: &String| name.to_uppercase(),
                    ).await {
                        Ok(names) => names,
                        Err(_) => vec![],
                    }
                }
                Err(_) => vec![],
            }
        }
        Err(_) => vec![],
    };
    let async_time = start.elapsed();
    
    println!("Sequential time: {:?}", sequential_time);
    println!("Async time: {:?}", async_time);
    println!("Results match: {}", sequential_result == async_result);
    println!("Sequential result: {:?}", sequential_result);
    println!("Async result: {:?}", async_result);
}