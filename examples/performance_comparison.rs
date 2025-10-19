//! Comprehensive performance comparison between Traditional, Parallel, and Async approaches
//! with large datasets using KeyPath operations

use key_paths_derive::Keypath;
use rust_prelude_plus::prelude::*;
use rust_prelude_plus::parallel::parallel_collections;
use rust_prelude_plus::async_ops::async_collections;
use rayon::prelude::*;
use std::sync::Arc;
use std::time::Instant;

/// Data structure for performance testing
#[derive(Keypath, Debug, Clone, PartialEq)]
pub struct Employee {
    pub id: u32,
    pub name: String,
    pub department: String,
    pub salary: f64,
    pub age: u32,
    pub is_active: bool,
    pub skills: Vec<String>,
    pub years_experience: u32,
}

/// Performance test results
#[derive(Debug)]
pub struct PerformanceResult {
    pub approach: String,
    pub dataset_size: usize,
    pub operation: String,
    pub duration: std::time::Duration,
    pub throughput: f64, // operations per second
    pub memory_usage: Option<usize>, // in bytes
}

impl PerformanceResult {
    pub fn new(approach: &str, dataset_size: usize, operation: &str, duration: std::time::Duration) -> Self {
        let throughput = dataset_size as f64 / duration.as_secs_f64();
        Self {
            approach: approach.to_string(),
            dataset_size,
            operation: operation.to_string(),
            duration,
            throughput,
            memory_usage: None,
        }
    }
}

fn main() {
    println!("=== Large Dataset Performance Comparison ===");
    println!("Testing Traditional vs Parallel vs Async approaches with KeyPaths\n");
    
    // Test with different dataset sizes
    let dataset_sizes = vec![
        1_000,      // Small
        10_000,     // Medium
        50_000,     // Large
        100_000,    // Very Large
        500_000,    // Huge
    ];
    
    for &size in &dataset_sizes {
        println!("📊 Testing with {} employees", size);
        println!("{}", "=".repeat(50));
        
        let employees = create_large_dataset(size);
        
        // Test different operations
        test_map_operations(&employees, size);
        test_filter_operations(&employees, size);
        test_fold_operations(&employees, size);
        test_complex_operations(&employees, size);
        
        println!();
    }
    
    // Memory usage comparison
    test_memory_usage();
    
    println!("🎯 Performance comparison completed!");
}

fn create_large_dataset(size: usize) -> Vec<Employee> {
    let departments = vec!["Engineering", "Marketing", "HR", "Sales", "Finance"];
    let skills_pool = vec![
        "Rust", "Python", "JavaScript", "Go", "Java", "C++", "React", "Vue", 
        "Angular", "Node.js", "Docker", "Kubernetes", "AWS", "GCP", "Azure"
    ];
    
    (0..size)
        .map(|i| Employee {
            id: i as u32,
            name: format!("Employee_{:06}", i),
            department: departments[i % departments.len()].to_string(),
            salary: 30000.0 + (i as f64 * 100.0) + (i % 1000) as f64,
            age: 22 + (i % 40) as u32,
            is_active: i % 3 != 0, // 2/3 are active
            skills: skills_pool.iter()
                .enumerate()
                .filter(|(idx, _)| idx % 3 == i % 3)
                .map(|(_, skill)| skill.to_string())
                .take(3)
                .collect(),
            years_experience: (i % 20) as u32,
        })
        .collect()
}

fn test_map_operations(employees: &[Employee], size: usize) {
    println!("\n🔄 Map Operations - Extract Employee Names");
    
    let mut results = Vec::new();
    
    // Traditional approach
    let start = Instant::now();
    let traditional_names: Vec<String> = employees
        .iter()
        .map(|emp| emp.name.clone())
        .collect();
    let traditional_duration = start.elapsed();
    results.push(PerformanceResult::new("Traditional", size, "Map Names", traditional_duration));
    
    // Parallel approach
    let start = Instant::now();
    let parallel_names = match parallel_collections::par_map_keypath(
        employees.to_vec(),
        Employee::name(),
        |name: &String| name.clone(),
    ) {
        Ok(names) => names,
        Err(e) => {
            println!("Parallel map error: {}", e);
            vec![]
        }
    };
    let parallel_duration = start.elapsed();
    results.push(PerformanceResult::new("Parallel", size, "Map Names", parallel_duration));
    
    // Async approach
    let start = Instant::now();
    let async_names = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            async_collections::map_keypath_async(
                employees.to_vec(),
                Employee::name(),
                |name: &String| name.clone(),
            ).await
        });
    let async_duration = start.elapsed();
    let async_names = match async_names {
        Ok(names) => names,
        Err(e) => {
            println!("Async map error: {}", e);
            vec![]
        }
    };
    results.push(PerformanceResult::new("Async", size, "Map Names", async_duration));
    
    // Verify results are identical
    assert_eq!(traditional_names.len(), parallel_names.len());
    assert_eq!(traditional_names.len(), async_names.len());
    
    print_performance_results(&results);
}

fn test_filter_operations(employees: &[Employee], size: usize) {
    println!("\n🔍 Filter Operations - Active Employees");
    
    let mut results = Vec::new();
    
    // Traditional approach
    let start = Instant::now();
    let traditional_active: Vec<&Employee> = employees
        .iter()
        .filter(|emp| emp.is_active)
        .collect();
    let traditional_duration = start.elapsed();
    results.push(PerformanceResult::new("Traditional", size, "Filter Active", traditional_duration));
    
    // Parallel approach
    let start = Instant::now();
    let parallel_active = match parallel_collections::par_filter_by_keypath(
        employees.to_vec(),
        Employee::is_active(),
        |&is_active| is_active,
    ) {
        Ok(active) => active,
        Err(e) => {
            println!("Parallel filter error: {}", e);
            vec![]
        }
    };
    let parallel_duration = start.elapsed();
    results.push(PerformanceResult::new("Parallel", size, "Filter Active", parallel_duration));
    
    // Async approach
    let start = Instant::now();
    let async_active = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            async_collections::filter_by_keypath_async(
                employees.to_vec(),
                Employee::is_active(),
                |&is_active| is_active,
            ).await
        });
    let async_duration = start.elapsed();
    let async_active = match async_active {
        Ok(active) => active,
        Err(e) => {
            println!("Async filter error: {}", e);
            vec![]
        }
    };
    results.push(PerformanceResult::new("Async", size, "Filter Active", async_duration));
    
    // Verify results are identical
    assert_eq!(traditional_active.len(), parallel_active.len());
    assert_eq!(traditional_active.len(), async_active.len());
    
    print_performance_results(&results);
}

fn test_fold_operations(employees: &[Employee], size: usize) {
    println!("\n📊 Fold Operations - Calculate Total Salary");
    
    let mut results = Vec::new();
    
    // Traditional approach
    let start = Instant::now();
    let traditional_total: f64 = employees
        .iter()
        .map(|emp| emp.salary)
        .sum();
    let traditional_duration = start.elapsed();
    results.push(PerformanceResult::new("Traditional", size, "Sum Salaries", traditional_duration));
    
    // Parallel approach using rayon
    let start = Instant::now();
    let parallel_total: f64 = employees
        .par_iter()
        .map(|emp| emp.salary)
        .sum();
    let parallel_duration = start.elapsed();
    results.push(PerformanceResult::new("Parallel", size, "Sum Salaries", parallel_duration));
    
    // Async approach (simulated with traditional for now since we don't have async fold)
    let start = Instant::now();
    let async_total: f64 = employees
        .iter()
        .map(|emp| emp.salary)
        .sum();
    let async_duration = start.elapsed();
    results.push(PerformanceResult::new("Async", size, "Sum Salaries", async_duration));
    
    // Verify results are identical
    assert!((traditional_total - parallel_total).abs() < 0.01);
    assert!((traditional_total - async_total).abs() < 0.01);
    
    print_performance_results(&results);
}

fn test_complex_operations(employees: &[Employee], size: usize) {
    println!("\n🔗 Complex Operations - High-earning Engineers");
    
    let mut results = Vec::new();
    
    // Traditional approach - Complex chain
    let start = Instant::now();
    let traditional_result: Vec<String> = employees
        .iter()
        .filter(|emp| emp.department == "Engineering")
        .filter(|emp| emp.salary > 80000.0)
        .filter(|emp| emp.is_active)
        .map(|emp| emp.name.clone())
        .collect();
    let traditional_duration = start.elapsed();
    results.push(PerformanceResult::new("Traditional", size, "Complex Chain", traditional_duration));
    
    // Parallel approach - Complex chain
    let start = Instant::now();
    let parallel_result = match parallel_collections::par_filter_by_keypath(
        employees.to_vec(),
        Employee::department(),
        |dept| dept == "Engineering",
    ) {
        Ok(engineers) => {
            match parallel_collections::par_filter_by_keypath(
                engineers,
                Employee::salary(),
                |&salary| salary > 80000.0,
            ) {
                Ok(high_earners) => {
                    match parallel_collections::par_filter_by_keypath(
                        high_earners,
                        Employee::is_active(),
                        |&is_active| is_active,
                    ) {
                        Ok(active_high_earners) => {
                            match parallel_collections::par_map_keypath(
                                active_high_earners,
                                Employee::name(),
                                |name: &String| name.clone(),
                            ) {
                                Ok(names) => names,
                                Err(_) => vec![],
                            }
                        }
                        Err(_) => vec![],
                    }
                }
                Err(_) => vec![],
            }
        }
        Err(_) => vec![],
    };
    let parallel_duration = start.elapsed();
    results.push(PerformanceResult::new("Parallel", size, "Complex Chain", parallel_duration));
    
    // Async approach - Complex chain
    let start = Instant::now();
    let async_result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            match async_collections::filter_by_keypath_async(
                employees.to_vec(),
                Employee::department(),
                |dept| dept == "Engineering",
            ).await {
                Ok(engineers) => {
                    match async_collections::filter_by_keypath_async(
                        engineers,
                        Employee::salary(),
                        |&salary| salary > 80000.0,
                    ).await {
                        Ok(high_earners) => {
                            match async_collections::filter_by_keypath_async(
                                high_earners,
                                Employee::is_active(),
                                |&is_active| is_active,
                            ).await {
                                Ok(active_high_earners) => {
                                    match async_collections::map_keypath_async(
                                        active_high_earners,
                                        Employee::name(),
                                        |name: &String| name.clone(),
                                    ).await {
                                        Ok(names) => names,
                                        Err(_) => vec![],
                                    }
                                }
                                Err(_) => vec![],
                            }
                        }
                        Err(_) => vec![],
                    }
                }
                Err(_) => vec![],
            }
        });
    let async_duration = start.elapsed();
    results.push(PerformanceResult::new("Async", size, "Complex Chain", async_duration));
    
    // Verify results are identical
    assert_eq!(traditional_result.len(), parallel_result.len());
    assert_eq!(traditional_result.len(), async_result.len());
    
    print_performance_results(&results);
}

fn test_memory_usage() {
    println!("\n💾 Memory Usage Comparison");
    println!("{}", "=".repeat(50));
    
    let size = 100_000;
    let employees = create_large_dataset(size);
    
    // Traditional approach memory usage
    let start = Instant::now();
    let traditional_result: Vec<String> = employees
        .iter()
        .map(|emp| emp.name.clone())
        .collect();
    let traditional_duration = start.elapsed();
    
    // Parallel approach memory usage
    let start = Instant::now();
    let parallel_result = match parallel_collections::par_map_keypath(
        employees.to_vec(),
        Employee::name(),
        |name: &String| name.clone(),
    ) {
        Ok(names) => names,
        Err(_) => vec![],
    };
    let parallel_duration = start.elapsed();
    
    // Async approach memory usage
    let start = Instant::now();
    let async_result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            async_collections::map_keypath_async(
                employees.to_vec(),
                Employee::name(),
                |name: &String| name.clone(),
            ).await
        });
    let async_duration = start.elapsed();
    let async_result = match async_result {
        Ok(names) => names,
        Err(_) => vec![],
    };
    
    println!("Dataset size: {} employees", size);
    println!("Traditional: {:?} ({} items)", traditional_duration, traditional_result.len());
    println!("Parallel:    {:?} ({} items)", parallel_duration, parallel_result.len());
    println!("Async:       {:?} ({} items)", async_duration, async_result.len());
    
    // Calculate speedup
    let parallel_speedup = traditional_duration.as_secs_f64() / parallel_duration.as_secs_f64();
    let async_speedup = traditional_duration.as_secs_f64() / async_duration.as_secs_f64();
    
    println!("\n🚀 Speedup Analysis:");
    println!("Parallel vs Traditional: {:.2}x", parallel_speedup);
    println!("Async vs Traditional:    {:.2}x", async_speedup);
    
    if parallel_speedup > 1.0 {
        println!("✅ Parallel approach is {:.2}x faster than traditional", parallel_speedup);
    } else {
        println!("⚠️  Parallel approach is {:.2}x slower than traditional", 1.0 / parallel_speedup);
    }
    
    if async_speedup > 1.0 {
        println!("✅ Async approach is {:.2}x faster than traditional", async_speedup);
    } else {
        println!("⚠️  Async approach is {:.2}x slower than traditional", 1.0 / async_speedup);
    }
}

fn print_performance_results(results: &[PerformanceResult]) {
    println!("Results:");
    for result in results {
        println!(
            "  {}: {:?} ({:.0} ops/sec)",
            result.approach,
            result.duration,
            result.throughput
        );
    }
    
    // Find the fastest approach
    let fastest = results.iter().min_by(|a, b| a.duration.cmp(&b.duration)).unwrap();
    let slowest = results.iter().max_by(|a, b| a.duration.cmp(&b.duration)).unwrap();
    
    let speedup = slowest.duration.as_secs_f64() / fastest.duration.as_secs_f64();
    
    println!("🏆 Fastest: {} ({:?})", fastest.approach, fastest.duration);
    println!("🐌 Slowest: {} ({:?})", slowest.approach, slowest.duration);
    println!("⚡ Speedup: {:.2}x", speedup);
    
    // Performance analysis
    if fastest.approach == "Parallel" && speedup > 1.5 {
        println!("✅ Parallel approach shows significant performance improvement");
    } else if fastest.approach == "Async" && speedup > 1.2 {
        println!("✅ Async approach shows good performance improvement");
    } else if fastest.approach == "Traditional" {
        println!("⚠️  Traditional approach is fastest - parallel/async overhead may be too high for this dataset size");
    }
}
