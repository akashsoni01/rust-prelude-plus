use rust_prelude_plus::prelude::*;
use key_paths_derive::Keypath;
use std::sync::Arc;
use std::time::Instant;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[derive(Keypath, Debug, Clone, PartialEq)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    category: String,
    in_stock: bool,
    rating: f64,
    tags: Vec<String>,
}

#[derive(Keypath, Debug, Clone, PartialEq)]
struct Order {
    id: u32,
    customer_id: u32,
    products: Vec<Product>,
    total_amount: f64,
    status: String,
    created_at: String,
}

fn main() {
    println!("=== Parallel KeyPath Examples ===\n");

    #[cfg(feature = "parallel")]
    {
        // Create sample data
        let products = create_sample_products();
        let orders = create_sample_orders();

        // 1. Basic Parallel Operations
        demonstrate_basic_parallel_operations(&products);

        // 2. Parallel Collection Operations
        demonstrate_parallel_collection_operations(&products);

        // 3. Parallel Processing with Custom Thread Pools
        demonstrate_custom_thread_pools(&products);

        // 4. Parallel Work Stealing
        demonstrate_work_stealing(&products);

        // 5. Parallel Load Balancing
        demonstrate_load_balancing(&products);

        // 6. Performance Comparison
        demonstrate_performance_comparison(&products, &orders);
    }

    #[cfg(not(feature = "parallel"))]
    {
        println!("Parallel features are not enabled. Run with --features parallel to see examples.");
    }
}

#[cfg(feature = "parallel")]
fn create_sample_products() -> Vec<Arc<Product>> {
    (1..=1000)
        .map(|i| {
            Arc::new(Product {
                id: i,
                name: format!("Product {}", i),
                price: 10.0 + (i as f64 * 0.5),
                category: if i % 3 == 0 { "Electronics" } else if i % 3 == 1 { "Books" } else { "Clothing" }.to_string(),
                in_stock: i % 4 != 0,
                rating: 3.0 + (i as f64 % 5.0) * 0.4,
                tags: vec![format!("tag{}", i % 10), format!("category{}", i % 5)],
            })
        })
        .collect()
}

#[cfg(feature = "parallel")]
fn create_sample_orders() -> Vec<Arc<Order>> {
    (1..=100)
        .map(|i| {
            Arc::new(Order {
                id: i,
                customer_id: i * 10,
                products: (1..=5)
                    .map(|j| Product {
                        id: i * 10 + j,
                        name: format!("Product {}-{}", i, j),
                        price: 20.0 + (j as f64 * 2.0),
                        category: "Electronics".to_string(),
                        in_stock: true,
                        rating: 4.0,
                        tags: vec!["electronics".to_string(), "popular".to_string()],
                    })
                    .collect(),
                total_amount: 100.0 + (i as f64 * 10.0),
                status: if i % 3 == 0 { "completed" } else if i % 3 == 1 { "pending" } else { "shipped" }.to_string(),
                created_at: format!("2024-01-{:02}", (i % 30) + 1),
            })
        })
        .collect()
}

#[cfg(feature = "parallel")]
fn demonstrate_basic_parallel_operations(products: &[Arc<Product>]) {
    println!("1. BASIC PARALLEL OPERATIONS:");
    
    // Parallel map operation
    let start = Instant::now();
    let product_names: Vec<String> = products
        .par_iter()
        .map_keypath(Product::name(), |name| name.clone())
        .collect();
    let map_time = start.elapsed();
    println!("   Parallel map - {} products in {:?}", product_names.len(), map_time);

    // Parallel filter operation
    let start = Instant::now();
    let expensive_products: Vec<Arc<Product>> = products
        .par_iter()
        .filter_by_keypath(Product::price(), |&price| price > 100.0)
        .map(|product| product.clone())
        .collect();
    let filter_time = start.elapsed();
    println!("   Parallel filter - {} expensive products in {:?}", expensive_products.len(), filter_time);

    // Parallel find operation
    let start = Instant::now();
    let high_rated_product = products
        .par_iter()
        .find_by_keypath(Product::rating(), |&rating| rating > 4.5)
        .unwrap();
    let find_time = start.elapsed();
    println!("   Parallel find - found {} in {:?}", high_rated_product.name, find_time);

    // Parallel fold operation
    let start = Instant::now();
    let total_value: f64 = products
        .par_iter()
        .fold_keypath(Product::price(), 0.0, |acc, &price| acc + price)
        .unwrap();
    let fold_time = start.elapsed();
    println!("   Parallel fold - total value ${:.2} in {:?}", total_value, fold_time);

    println!();
}

#[cfg(feature = "parallel")]
fn demonstrate_parallel_collection_operations(products: &[Arc<Product>]) {
    println!("2. PARALLEL COLLECTION OPERATIONS:");
    
    // Parallel group by category
    let start = Instant::now();
    let grouped_by_category = products
        .par_iter()
        .map(|product| (product.category.clone(), product.clone()))
        .collect::<Vec<_>>();
    let mut category_groups: std::collections::HashMap<String, Vec<Arc<Product>>> = std::collections::HashMap::new();
    for (category, product) in grouped_by_category {
        category_groups.entry(category).or_insert_with(Vec::new).push(product);
    }
    let group_time = start.elapsed();
    println!("   Parallel group by category - {} categories in {:?}", category_groups.len(), group_time);

    // Parallel partition
    let start = Instant::now();
    let (in_stock, out_of_stock): (Vec<Arc<Product>>, Vec<Arc<Product>>) = products
        .par_iter()
        .partition(|product| product.in_stock);
    let partition_time = start.elapsed();
    println!("   Parallel partition - {} in stock, {} out of stock in {:?}", 
             in_stock.len(), out_of_stock.len(), partition_time);

    // Parallel count
    let start = Instant::now();
    let electronics_count = products
        .par_iter()
        .filter_by_keypath(Product::category(), |cat| cat == "Electronics")
        .count();
    let count_time = start.elapsed();
    println!("   Parallel count - {} electronics in {:?}", electronics_count, count_time);

    // Parallel any/all
    let start = Instant::now();
    let has_expensive_products = products
        .par_iter()
        .any_by_keypath(Product::price(), |&price| price > 500.0)
        .unwrap();
    let any_time = start.elapsed();
    println!("   Parallel any - has expensive products: {} in {:?}", has_expensive_products, any_time);

    let start = Instant::now();
    let all_have_ratings = products
        .par_iter()
        .all_by_keypath(Product::rating(), |&rating| rating > 0.0)
        .unwrap();
    let all_time = start.elapsed();
    println!("   Parallel all - all have ratings: {} in {:?}", all_have_ratings, all_time);

    println!();
}

#[cfg(feature = "parallel")]
fn demonstrate_custom_thread_pools(products: &[Arc<Product>]) {
    println!("3. CUSTOM THREAD POOLS:");
    
    // Create custom thread pool
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    let start = Instant::now();
    let result = pool.install(|| {
        products
            .par_iter()
            .map_keypath(Product::price(), |&price| price * 1.1) // 10% price increase
            .collect::<Vec<_>>()
    });
    let pool_time = start.elapsed();
    println!("   Custom thread pool (4 threads) - processed {} prices in {:?}", result.len(), pool_time);

    // Compare with default thread pool
    let start = Instant::now();
    let default_result = products
        .par_iter()
        .map_keypath(Product::price(), |&price| price * 1.1)
        .collect::<Vec<_>>();
    let default_time = start.elapsed();
    println!("   Default thread pool - processed {} prices in {:?}", default_result.len(), default_time);

    println!();
}

#[cfg(feature = "parallel")]
fn demonstrate_work_stealing(products: &[Arc<Product>]) {
    println!("4. WORK STEALING:");
    
    // Parallel map with work stealing
    let start = Instant::now();
    let chunk_size = 100;
    let result: Vec<f64> = products
        .par_chunks(chunk_size)
        .flat_map(|chunk| {
            chunk
                .par_iter()
                .map_keypath(Product::price(), |&price| price * 0.9) // 10% discount
                .collect::<Vec<_>>()
        })
        .collect();
    let work_stealing_time = start.elapsed();
    println!("   Work stealing (chunk size {}) - processed {} prices in {:?}", 
             chunk_size, result.len(), work_stealing_time);

    // Parallel reduce with work stealing
    let start = Instant::now();
    let total_discounted_value = products
        .par_chunks(chunk_size)
        .map(|chunk| {
            chunk
                .iter()
                .map(|product| product.price * 0.9)
                .sum::<f64>()
        })
        .sum::<f64>();
    let reduce_time = start.elapsed();
    println!("   Work stealing reduce - total discounted value ${:.2} in {:?}", 
             total_discounted_value, reduce_time);

    println!();
}

#[cfg(feature = "parallel")]
fn demonstrate_load_balancing(products: &[Arc<Product>]) {
    println!("5. LOAD BALANCING:");
    
    // Parallel map with load balancing
    let start = Instant::now();
    let balanced_result: Vec<String> = products
        .par_iter()
        .with_min_len(1)
        .with_max_len(products.len() / rayon::current_num_threads())
        .map_keypath(Product::name(), |name| name.to_uppercase())
        .collect();
    let balanced_time = start.elapsed();
    println!("   Load balanced map - processed {} names in {:?}", balanced_result.len(), balanced_time);

    // Parallel filter with load balancing
    let start = Instant::now();
    let balanced_filtered: Vec<Arc<Product>> = products
        .par_iter()
        .with_min_len(1)
        .with_max_len(products.len() / rayon::current_num_threads())
        .filter_by_keypath(Product::rating(), |&rating| rating > 4.0)
        .map(|product| product.clone())
        .collect();
    let balanced_filter_time = start.elapsed();
    println!("   Load balanced filter - {} high-rated products in {:?}", 
             balanced_filtered.len(), balanced_filter_time);

    println!();
}

#[cfg(feature = "parallel")]
fn demonstrate_performance_comparison(products: &[Arc<Product>], orders: &[Arc<Order>]) {
    println!("6. PERFORMANCE COMPARISON:");
    
    // Sequential vs Parallel processing
    let iterations = 10;
    
    // Sequential processing
    let start = Instant::now();
    for _ in 0..iterations {
        let _result: Vec<f64> = products
            .iter()
            .map_keypath(Product::price(), |&price| price * 1.1)
            .collect();
    }
    let sequential_time = start.elapsed();
    
    // Parallel processing
    let start = Instant::now();
    for _ in 0..iterations {
        let _result: Vec<f64> = products
            .par_iter()
            .map_keypath(Product::price(), |&price| price * 1.1)
            .collect();
    }
    let parallel_time = start.elapsed();
    
    println!("   Sequential processing: {:?} ({} iterations)", sequential_time, iterations);
    println!("   Parallel processing: {:?} ({} iterations)", parallel_time, iterations);
    println!("   Speedup: {:.2}x", sequential_time.as_secs_f64() / parallel_time.as_secs_f64());
    
    // Complex operation comparison
    let start = Instant::now();
    let sequential_complex: Vec<String> = products
        .iter()
        .filter_by_keypath(Product::category(), |cat| cat == "Electronics")
        .filter_by_keypath(Product::in_stock(), |&in_stock| in_stock)
        .filter_by_keypath(Product::rating(), |&rating| rating > 4.0)
        .map_keypath(Product::name(), |name| name.clone())
        .collect();
    let sequential_complex_time = start.elapsed();
    
    let start = Instant::now();
    let parallel_complex: Vec<String> = products
        .par_iter()
        .filter_by_keypath(Product::category(), |cat| cat == "Electronics")
        .filter_by_keypath(Product::in_stock(), |&in_stock| in_stock)
        .filter_by_keypath(Product::rating(), |&rating| rating > 4.0)
        .map_keypath(Product::name(), |name| name.clone())
        .collect();
    let parallel_complex_time = start.elapsed();
    
    println!("   Sequential complex operation: {:?} - {} results", 
             sequential_complex_time, sequential_complex.len());
    println!("   Parallel complex operation: {:?} - {} results", 
             parallel_complex_time, parallel_complex.len());
    println!("   Complex operation speedup: {:.2}x", 
             sequential_complex_time.as_secs_f64() / parallel_complex_time.as_secs_f64());
    
    // Order processing comparison
    let start = Instant::now();
    let sequential_orders: Vec<f64> = orders
        .iter()
        .filter_by_keypath(Order::status(), |status| status == "completed")
        .map_keypath(Order::total_amount(), |&amount| amount)
        .collect();
    let sequential_orders_time = start.elapsed();
    
    let start = Instant::now();
    let parallel_orders: Vec<f64> = orders
        .par_iter()
        .filter_by_keypath(Order::status(), |status| status == "completed")
        .map_keypath(Order::total_amount(), |&amount| amount)
        .collect();
    let parallel_orders_time = start.elapsed();
    
    println!("   Sequential order processing: {:?} - {} orders", 
             sequential_orders_time, sequential_orders.len());
    println!("   Parallel order processing: {:?} - {} orders", 
             parallel_orders_time, parallel_orders.len());
    println!("   Order processing speedup: {:.2}x", 
             sequential_orders_time.as_secs_f64() / parallel_orders_time.as_secs_f64());
    
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "parallel")]
    #[test]
    fn test_parallel_operations() {
        let products = create_sample_products();
        
        // Test parallel map
        let product_names: Vec<String> = products
            .par_iter()
            .map_keypath(Product::name(), |name| name.clone())
            .collect();
        assert_eq!(product_names.len(), 1000);
        
        // Test parallel filter
        let expensive_products: Vec<Arc<Product>> = products
            .par_iter()
            .filter_by_keypath(Product::price(), |&price| price > 100.0)
            .map(|product| product.clone())
            .collect();
        assert!(expensive_products.len() > 0);
        
        // Test parallel find
        let high_rated_product = products
            .par_iter()
            .find_by_keypath(Product::rating(), |&rating| rating > 4.5)
            .unwrap();
        assert!(high_rated_product.rating > 4.5);
        
        // Test parallel fold
        let total_value: f64 = products
            .par_iter()
            .fold_keypath(Product::price(), 0.0, |acc, &price| acc + price)
            .unwrap();
        assert!(total_value > 0.0);
    }

    #[cfg(feature = "parallel")]
    #[test]
    fn test_parallel_collection_operations() {
        let products = create_sample_products();
        
        // Test parallel count
        let electronics_count = products
            .par_iter()
            .filter_by_keypath(Product::category(), |cat| cat == "Electronics")
            .count();
        assert!(electronics_count > 0);
        
        // Test parallel any
        let has_expensive_products = products
            .par_iter()
            .any_by_keypath(Product::price(), |&price| price > 500.0)
            .unwrap();
        assert!(has_expensive_products);
        
        // Test parallel all
        let all_have_ratings = products
            .par_iter()
            .all_by_keypath(Product::rating(), |&rating| rating > 0.0)
            .unwrap();
        assert!(all_have_ratings);
    }

    #[cfg(feature = "parallel")]
    #[test]
    fn test_custom_thread_pools() {
        let products = create_sample_products();
        
        // Create custom thread pool
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build()
            .unwrap();
        
        let result = pool.install(|| {
            products
                .par_iter()
                .map_keypath(Product::price(), |&price| price * 1.1)
                .collect::<Vec<_>>()
        });
        
        assert_eq!(result.len(), 1000);
        assert!(result[0] > 0.0);
    }

    #[cfg(feature = "parallel")]
    #[test]
    fn test_work_stealing() {
        let products = create_sample_products();
        
        // Test work stealing with chunks
        let chunk_size = 100;
        let result: Vec<f64> = products
            .par_chunks(chunk_size)
            .flat_map(|chunk| {
                chunk
                    .par_iter()
                    .map_keypath(Product::price(), |&price| price * 0.9)
                    .collect::<Vec<_>>()
            })
            .collect();
        
        assert_eq!(result.len(), 1000);
        assert!(result[0] > 0.0);
    }

    #[cfg(feature = "parallel")]
    #[test]
    fn test_load_balancing() {
        let products = create_sample_products();
        
        // Test load balancing
        let balanced_result: Vec<String> = products
            .par_iter()
            .with_min_len(1)
            .with_max_len(products.len() / rayon::current_num_threads())
            .map_keypath(Product::name(), |name| name.to_uppercase())
            .collect();
        
        assert_eq!(balanced_result.len(), 1000);
        assert!(balanced_result[0].contains("PRODUCT"));
    }
}
