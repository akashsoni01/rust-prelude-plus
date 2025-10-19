//! Examples demonstrating parallel keypath operations

use key_paths_derive::Keypath;
use rust_prelude_plus::parallel::parallel_collections;
use rayon::prelude::*;
use std::sync::Arc;

/// Example data structure for demonstrating parallel keypath operations
#[derive(Keypath, Debug, Clone, PartialEq)]
pub struct Product {
    pub name: String,
    pub price: f64,
    pub category: String,
    pub rating: f64,
    pub in_stock: bool,
}

#[derive(Keypath, Debug, Clone, PartialEq)]
pub struct Order {
    pub id: u32,
    pub status: String,
    pub total_amount: f64,
    pub customer_id: u32,
}

fn main() {
    println!("=== Parallel KeyPath Examples ===");
    
    // Create sample data
    let products = create_sample_products();
    let orders = create_sample_orders();
    
    // Test parallel operations
    test_parallel_operations(&products);
    test_parallel_collections(&products);
    test_parallel_thread_pools(&products);
    test_performance_comparison(&products, &orders);
    
    println!("\n=== All parallel examples completed successfully! ===");
}

fn create_sample_products() -> Vec<Arc<Product>> {
    vec![
        Arc::new(Product {
            name: "Laptop".to_string(),
            price: 999.99,
            category: "Electronics".to_string(),
            rating: 4.5,
            in_stock: true,
        }),
        Arc::new(Product {
            name: "Mouse".to_string(),
            price: 29.99,
            category: "Electronics".to_string(),
            rating: 4.2,
            in_stock: true,
        }),
        Arc::new(Product {
            name: "Keyboard".to_string(),
            price: 79.99,
            category: "Electronics".to_string(),
            rating: 4.3,
            in_stock: false,
        }),
        Arc::new(Product {
            name: "Monitor".to_string(),
            price: 299.99,
            category: "Electronics".to_string(),
            rating: 4.7,
            in_stock: true,
        }),
        Arc::new(Product {
            name: "Desk".to_string(),
            price: 199.99,
            category: "Furniture".to_string(),
            rating: 4.1,
            in_stock: true,
        }),
    ]
}

fn create_sample_orders() -> Vec<Arc<Order>> {
    vec![
        Arc::new(Order {
            id: 1,
            status: "completed".to_string(),
            total_amount: 1029.98,
            customer_id: 1001,
        }),
        Arc::new(Order {
            id: 2,
            status: "pending".to_string(),
            total_amount: 79.99,
            customer_id: 1002,
        }),
        Arc::new(Order {
            id: 3,
            status: "completed".to_string(),
            total_amount: 299.99,
            customer_id: 1003,
        }),
        Arc::new(Order {
            id: 4,
            status: "cancelled".to_string(),
            total_amount: 199.99,
            customer_id: 1004,
        }),
    ]
}

fn test_parallel_operations(products: &[Arc<Product>]) {
    println!("\n=== Parallel Operations ===");
    
    // Convert Arc<Product> to Product for parallel operations
    let products_owned: Vec<Product> = products.iter().map(|p| (**p).clone()).collect();
    
    // Parallel map - get product names
    match parallel_collections::par_map_keypath(
        products_owned.clone(),
        Product::name(),
        |name| name.clone(),
    ) {
        Ok(names) => println!("Product names: {:?}", names),
        Err(e) => println!("Error in parallel map: {}", e),
    }
    
    // Parallel filter - get expensive products
    match parallel_collections::par_filter_by_keypath(
        products_owned.clone(),
        Product::price(),
        |&price| price > 100.0,
    ) {
        Ok(expensive) => println!("Expensive products count: {}", expensive.len()),
        Err(e) => println!("Error in parallel filter: {}", e),
    }
    
    // Parallel find - find high-rated product
    match parallel_collections::par_find_by_keypath(
        products_owned.clone(),
        Product::rating(),
        |&rating| rating > 4.5,
    ) {
        Ok(Some(product)) => println!("Found high-rated product: {}", product.name),
        Ok(None) => println!("No high-rated product found"),
        Err(e) => println!("Error in parallel find: {}", e),
    }
    
    // Parallel collect - get all prices
    match parallel_collections::par_collect_keypath(
        products_owned.clone(),
        Product::price(),
    ) {
        Ok(prices) => println!("All prices: {:?}", prices),
        Err(e) => println!("Error in parallel collect: {}", e),
    }
    
    // Parallel count - count electronics
    match parallel_collections::par_count_by_keypath(
        products_owned.clone(),
        Product::category(),
        |cat| cat == "Electronics",
    ) {
        Ok(count) => println!("Electronics count: {}", count),
        Err(e) => println!("Error in parallel count: {}", e),
    }
    
    // Parallel any - check if any expensive products
    match parallel_collections::par_any_by_keypath(
        products_owned.clone(),
        Product::price(),
        |&price| price > 500.0,
    ) {
        Ok(has_expensive) => println!("Has expensive products: {}", has_expensive),
        Err(e) => println!("Error in parallel any: {}", e),
    }
    
    // Parallel all - check if all have ratings
    match parallel_collections::par_all_by_keypath(
        products_owned,
        Product::rating(),
        |&rating| rating > 0.0,
    ) {
        Ok(all_rated) => println!("All products have ratings: {}", all_rated),
        Err(e) => println!("Error in parallel all: {}", e),
    }
}

fn test_parallel_collections(products: &[Arc<Product>]) {
    println!("\n=== Parallel Collections ===");
    
    // Convert Arc<Product> to Product for parallel operations
    let products_owned: Vec<Product> = products.iter().map(|p| (**p).clone()).collect();
    
    // Parallel partition using standard rayon
    let (in_stock, out_of_stock): (Vec<Product>, Vec<Product>) = products_owned
        .into_par_iter()
        .partition(|product| product.in_stock);
    
    println!("In stock: {} products", in_stock.len());
    println!("Out of stock: {} products", out_of_stock.len());
}

fn test_parallel_thread_pools(products: &[Arc<Product>]) {
    println!("\n=== Parallel Thread Pools ===");
    
    // Convert Arc<Product> to Product for parallel operations
    let products_owned: Vec<Product> = products.iter().map(|p| (**p).clone()).collect();
    
    // Create custom thread pool
    match rust_prelude_plus::parallel::parallel_pools::create_keypath_thread_pool(2) {
        Ok(pool) => {
            println!("Created custom thread pool with 2 threads");
            
            // Execute on custom pool
            match rust_prelude_plus::parallel::parallel_pools::execute_on_pool(
                &pool,
                products_owned.clone(),
                Product::name(),
                |name| name.len(),
            ) {
                Ok(name_lengths) => println!("Name lengths: {:?}", name_lengths),
                Err(e) => println!("Error executing on pool: {}", e),
            }
            
            // Filter on custom pool
            match rust_prelude_plus::parallel::parallel_pools::filter_on_pool(
                &pool,
                products_owned,
                Product::price(),
                |&price| price > 50.0,
            ) {
                Ok(filtered) => println!("Products > $50: {} items", filtered.len()),
                Err(e) => println!("Error filtering on pool: {}", e),
            }
        }
        Err(e) => println!("Failed to create thread pool: {}", e),
    }
}

fn test_performance_comparison(products: &[Arc<Product>], orders: &[Arc<Order>]) {
    println!("\n=== Performance Comparison ===");
    
    // Convert Arc<Product> to Product for parallel operations
    let products_owned: Vec<Product> = products.iter().map(|p| (**p).clone()).collect();
    let orders_owned: Vec<Order> = orders.iter().map(|o| (**o).clone()).collect();
    
    // Sequential processing
    let start = std::time::Instant::now();
    let sequential_result: Vec<String> = products_owned
        .iter()
        .filter(|p| p.category == "Electronics")
        .filter(|p| p.in_stock)
        .map(|p| p.name.clone())
        .collect();
    let sequential_time = start.elapsed();
    
    // Parallel processing
    let start = std::time::Instant::now();
    let parallel_result = match parallel_collections::par_filter_by_keypath(
        products_owned.clone(),
        Product::category(),
        |cat| cat == "Electronics",
    ) {
        Ok(electronics) => {
            match parallel_collections::par_filter_by_keypath(
                electronics,
                Product::in_stock(),
                |&in_stock| in_stock,
            ) {
                Ok(in_stock_electronics) => {
                    match parallel_collections::par_map_keypath(
                        in_stock_electronics,
                        Product::name(),
                        |name| name.clone(),
                    ) {
                        Ok(names) => names,
                        Err(_) => vec![],
                    }
                }
                Err(_) => vec![],
            }
        }
        Err(_) => vec![],
    };
    let parallel_time = start.elapsed();
    
    println!("Sequential time: {:?}", sequential_time);
    println!("Parallel time: {:?}", parallel_time);
    println!("Results match: {}", sequential_result == parallel_result);
    println!("Sequential result: {:?}", sequential_result);
    println!("Parallel result: {:?}", parallel_result);
    
    // Orders processing comparison
    let start = std::time::Instant::now();
    let sequential_orders: Vec<f64> = orders_owned
        .iter()
        .filter(|o| o.status == "completed")
        .map(|o| o.total_amount)
        .collect();
    let sequential_orders_time = start.elapsed();
    
    let start = std::time::Instant::now();
    let parallel_orders = match parallel_collections::par_filter_by_keypath(
        orders_owned,
        Order::status(),
        |status| status == "completed",
    ) {
        Ok(completed_orders) => {
            match parallel_collections::par_collect_keypath(
                completed_orders,
                Order::total_amount(),
            ) {
                Ok(amounts) => amounts,
                Err(_) => vec![],
            }
        }
        Err(_) => vec![],
    };
    let parallel_orders_time = start.elapsed();
    
    println!("\nOrders processing:");
    println!("Sequential time: {:?}", sequential_orders_time);
    println!("Parallel time: {:?}", parallel_orders_time);
    println!("Results match: {}", sequential_orders == parallel_orders);
}