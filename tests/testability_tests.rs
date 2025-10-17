use rust_prelude_plus::prelude::*;
use key_paths_derive::Keypath;
use std::collections::HashMap;

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

// Business logic functions for testing
fn get_products_by_category(products: &[Product], category: &str) -> KeyPathResult<Vec<Product>> {
    filter_by_keypath(products.to_vec(), Product::category(), |cat| cat == category)
}

fn get_products_in_stock(products: &[Product]) -> KeyPathResult<Vec<Product>> {
    filter_by_keypath(products.to_vec(), Product::in_stock(), |&in_stock| in_stock)
}

fn calculate_average_price(products: &[Product]) -> KeyPathResult<f64> {
    let total_price = fold_keypath(products.to_vec(), Product::price(), 0.0, |acc, &price| acc + price)?;
    Ok(total_price / products.len() as f64)
}

fn get_high_rated_products(products: &[Product], min_rating: f64) -> KeyPathResult<Vec<Product>> {
    filter_by_keypath(products.to_vec(), Product::rating(), |&rating| rating >= min_rating)
}

fn group_products_by_category(products: &[Product]) -> KeyPathResult<HashMap<String, Vec<Product>>> {
    group_by_keypath(products, Product::category(), |category| category.clone())
}

fn get_product_names(products: &[Product]) -> KeyPathResult<Vec<String>> {
    collect_keypath(products.to_vec(), Product::name())
}

fn get_orders_by_status(orders: &[Order], status: &str) -> KeyPathResult<Vec<Order>> {
    filter_by_keypath(orders.to_vec(), Order::status(), |order_status| order_status == status)
}

fn calculate_total_revenue(orders: &[Order]) -> KeyPathResult<f64> {
    fold_keypath(orders.to_vec(), Order::total_amount(), 0.0, |acc, &amount| acc + amount)
}

#[cfg(test)]
mod testability_tests {
    use super::*;

    // Test data factory functions
    fn create_test_product(id: u32, name: &str, price: f64, category: &str, in_stock: bool, rating: f64) -> Product {
        Product {
            id,
            name: name.to_string(),
            price,
            category: category.to_string(),
            in_stock,
            rating,
            tags: vec!["test".to_string()],
        }
    }

    fn create_test_order(id: u32, customer_id: u32, products: Vec<Product>, total_amount: f64, status: &str) -> Order {
        Order {
            id,
            customer_id,
            products,
            total_amount,
            status: status.to_string(),
            created_at: "2024-01-15".to_string(),
        }
    }

    #[test]
    fn test_pure_functions_predictability() {
        // Test that pure functions with KeyPaths are predictable
        let products = vec![
            create_test_product(1, "Laptop", 999.99, "Electronics", true, 4.5),
            create_test_product(2, "Book", 19.99, "Books", true, 4.2),
            create_test_product(3, "Phone", 699.99, "Electronics", false, 4.8),
        ];

        // Same input should always produce same output
        let result1 = get_products_by_category(&products, "Electronics").unwrap();
        let result2 = get_products_by_category(&products, "Electronics").unwrap();
        
        assert_eq!(result1, result2);
        assert_eq!(result1.len(), 2);
    }

    #[test]
    fn test_isolated_functions_no_side_effects() {
        // Test that functions don't modify input data
        let original_products = vec![
            create_test_product(1, "Laptop", 999.99, "Electronics", true, 4.5),
            create_test_product(2, "Book", 19.99, "Books", true, 4.2),
        ];

        let products_clone = original_products.clone();
        
        // Function should not modify original data
        let _result = get_products_in_stock(&products_clone).unwrap();
        
        // Original data should remain unchanged
        assert_eq!(original_products, products_clone);
    }

    #[test]
    fn test_composable_functions() {
        // Test that functions can be easily composed
        let products = vec![
            create_test_product(1, "Laptop", 999.99, "Electronics", true, 4.5),
            create_test_product(2, "Book", 19.99, "Books", true, 4.2),
            create_test_product(3, "Phone", 699.99, "Electronics", true, 4.8),
            create_test_product(4, "Tablet", 399.99, "Electronics", false, 4.0),
        ];

        // Compose multiple operations
        let electronics = get_products_by_category(&products, "Electronics").unwrap();
        let in_stock_electronics = get_products_in_stock(&electronics).unwrap();
        let high_rated_electronics = get_high_rated_products(&in_stock_electronics, 4.5).unwrap();
        let names = get_product_names(&high_rated_electronics).unwrap();

        // Should get both Laptop and Phone (both are electronics, in stock, and rated >= 4.5)
        assert_eq!(names, vec!["Laptop", "Phone"]);
    }

    #[test]
    fn test_type_safety_compile_time_validation() {
        // Test that KeyPaths provide compile-time type safety
        let products = vec![
            create_test_product(1, "Laptop", 999.99, "Electronics", true, 4.5),
        ];

        // These operations are guaranteed to work at compile time
        let names: Vec<String> = collect_keypath(products.clone(), Product::name()).unwrap();
        let prices: Vec<f64> = collect_keypath(products.clone(), Product::price()).unwrap();
        let categories: Vec<String> = collect_keypath(products.clone(), Product::category()).unwrap();

        assert_eq!(names, vec!["Laptop"]);
        assert_eq!(prices, vec![999.99]);
        assert_eq!(categories, vec!["Electronics"]);
    }

    #[test]
    fn test_error_handling_built_in() {
        // Test that KeyPaths provide built-in error handling
        let products = vec![
            create_test_product(1, "Laptop", 999.99, "Electronics", true, 4.5),
        ];

        // KeyPath operations return Result types
        let result: KeyPathResult<Vec<String>> = collect_keypath(products, Product::name());
        
        // Proper error handling
        match result {
            Ok(names) => assert_eq!(names, vec!["Laptop"]),
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn test_mock_data_generation_easy() {
        // Test that KeyPaths make mock data generation easy
        let mock_products = (1..=10)
            .map(|i| create_test_product(
                i,
                &format!("Product{}", i),
                10.0 * i as f64,
                if i % 2 == 0 { "Electronics" } else { "Books" },
                i % 3 != 0,
                3.0 + (i as f64 * 0.2)
            ))
            .collect::<Vec<_>>();

        // Test with generated data
        let electronics_count = get_products_by_category(&mock_products, "Electronics").unwrap().len();
        let in_stock_count = get_products_in_stock(&mock_products).unwrap().len();
        let avg_price = calculate_average_price(&mock_products).unwrap();

        assert_eq!(electronics_count, 5); // Products 2, 4, 6, 8, 10
        assert_eq!(in_stock_count, 7); // Products 1, 2, 4, 5, 7, 8, 10
        assert_eq!(avg_price, 55.0); // (10 + 20 + ... + 100) / 10
    }

    #[test]
    fn test_edge_cases_handling() {
        // Test edge cases with KeyPaths
        let empty_products: Vec<Product> = vec![];
        
        // Empty collection should be handled gracefully
        let result = get_products_by_category(&empty_products, "Electronics").unwrap();
        assert_eq!(result.len(), 0);

        let single_product = vec![create_test_product(1, "Laptop", 999.99, "Electronics", true, 4.5)];
        
        // Single item should work correctly
        let result = get_products_by_category(&single_product, "Electronics").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "Laptop");
    }

    #[test]
    fn test_complex_business_logic() {
        // Test complex business logic with KeyPaths
        let products = vec![
            create_test_product(1, "Laptop", 999.99, "Electronics", true, 4.5),
            create_test_product(2, "Book", 19.99, "Books", true, 4.2),
            create_test_product(3, "Phone", 699.99, "Electronics", true, 4.8),
            create_test_product(4, "Tablet", 399.99, "Electronics", false, 4.0),
        ];

        let orders = vec![
            create_test_order(1, 101, vec![products[0].clone()], 999.99, "completed"),
            create_test_order(2, 102, vec![products[1].clone()], 19.99, "pending"),
            create_test_order(3, 103, vec![products[2].clone()], 699.99, "completed"),
        ];

        // Complex business logic: Get completed orders with electronics
        let completed_orders = get_orders_by_status(&orders, "completed").unwrap();
        let mut electronics_orders = Vec::new();
        
        for order in completed_orders {
            let electronics_products = get_products_by_category(&order.products, "Electronics").unwrap();
            if !electronics_products.is_empty() {
                electronics_orders.push(order);
            }
        }

        assert_eq!(electronics_orders.len(), 2);
        
        let total_revenue = calculate_total_revenue(&electronics_orders).unwrap();
        assert_eq!(total_revenue, 1699.98); // 999.99 + 699.99
    }

    #[test]
    fn test_parameterized_tests() {
        // Test parameterized testing with KeyPaths
        let test_cases = vec![
            ("Electronics", 3),
            ("Books", 1),
            ("Clothing", 0),
        ];

        let products = vec![
            create_test_product(1, "Laptop", 999.99, "Electronics", true, 4.5),
            create_test_product(2, "Book", 19.99, "Books", true, 4.2),
            create_test_product(3, "Phone", 699.99, "Electronics", true, 4.8),
            create_test_product(4, "Tablet", 399.99, "Electronics", false, 4.0),
        ];

        for (category, expected_count) in test_cases {
            let result = get_products_by_category(&products, category).unwrap();
            assert_eq!(result.len(), expected_count, "Failed for category: {}", category);
        }
    }

    #[test]
    fn test_property_based_testing() {
        // Test property-based testing with KeyPaths
        let products = vec![
            create_test_product(1, "Laptop", 999.99, "Electronics", true, 4.5),
            create_test_product(2, "Book", 19.99, "Books", true, 4.2),
            create_test_product(3, "Phone", 699.99, "Electronics", true, 4.8),
        ];

        // Property: Filtering should never increase the number of items
        let all_products = get_products_by_category(&products, "Electronics").unwrap();
        let in_stock_products = get_products_in_stock(&all_products).unwrap();
        
        assert!(in_stock_products.len() <= all_products.len());
        assert!(all_products.len() <= products.len());

        // Property: Average price should be within bounds
        let avg_price = calculate_average_price(&products).unwrap();
        assert!(avg_price >= 0.0);
        assert!(avg_price <= 999.99);
    }

    #[test]
    fn test_integration_testing() {
        // Test integration testing with KeyPaths
        let products = vec![
            create_test_product(1, "Laptop", 999.99, "Electronics", true, 4.5),
            create_test_product(2, "Book", 19.99, "Books", true, 4.2),
            create_test_product(3, "Phone", 699.99, "Electronics", true, 4.8),
        ];

        let orders = vec![
            create_test_order(1, 101, vec![products[0].clone()], 999.99, "completed"),
            create_test_order(2, 102, vec![products[1].clone()], 19.99, "completed"),
            create_test_order(3, 103, vec![products[2].clone()], 699.99, "completed"),
        ];

        // Integration test: End-to-end business process
        let completed_orders = get_orders_by_status(&orders, "completed").unwrap();
        let total_revenue = calculate_total_revenue(&completed_orders).unwrap();
        
        // Verify business logic
        assert_eq!(completed_orders.len(), 3);
        assert_eq!(total_revenue, 1719.97); // 999.99 + 19.99 + 699.99
        
        // Verify data consistency
        let order_amounts: Vec<f64> = collect_keypath(completed_orders, Order::total_amount()).unwrap();
        let calculated_total: f64 = order_amounts.iter().sum();
        assert_eq!(total_revenue, calculated_total);
    }
}
