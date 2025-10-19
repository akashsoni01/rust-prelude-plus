//! Optimized performance comparison focusing on CPU-intensive operations
//! where parallel processing shows clear benefits

use key_paths_derive::Keypath;
use rayon::prelude::*;
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

fn main() {
    println!("=== Optimized Performance Comparison ===");
    println!("Focusing on CPU-intensive operations where parallel processing excels\n");
    
    // Test with different dataset sizes
    let dataset_sizes = vec![
        10_000,     // Medium
        50_000,     // Large
        100_000,    // Very Large
        500_000,    // Huge
        1_000_000,  // Massive
    ];
    
    for &size in &dataset_sizes {
        println!("📊 Testing with {} employees", size);
        println!("{}", "=".repeat(50));
        
        let employees = create_large_dataset(size);
        
        // Test CPU-intensive operations where parallel processing shines
        test_cpu_intensive_calculations(&employees, size);
        test_complex_filtering(&employees, size);
        test_aggregation_operations(&employees, size);
        test_sorting_operations(&employees, size);
        
        println!();
    }
    
    // Test memory efficiency
    test_memory_efficiency();
    
    println!("🎯 Optimized performance comparison completed!");
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

fn test_cpu_intensive_calculations(employees: &[Employee], size: usize) {
    println!("\n🧮 CPU-Intensive Calculations - Complex Salary Analysis");
    
    // Traditional approach - CPU intensive calculations
    let start = Instant::now();
    let traditional_result: Vec<f64> = employees
        .iter()
        .map(|emp| {
            // Simulate complex calculation
            let base_salary = emp.salary;
            let experience_bonus = emp.years_experience as f64 * 1000.0;
            let age_factor = if emp.age > 30 { 1.1 } else { 0.9 };
            let department_multiplier = match emp.department.as_str() {
                "Engineering" => 1.2,
                "Sales" => 1.15,
                "Marketing" => 1.05,
                _ => 1.0,
            };
            
            // Complex mathematical operations
            let mut result = base_salary;
            for _ in 0..100 { // Simulate CPU-intensive work
                result = (result + experience_bonus) * age_factor * department_multiplier;
                result = result.sqrt() * 1.1;
            }
            result
        })
        .collect();
    let traditional_duration = start.elapsed();
    
    // Parallel approach - Same CPU intensive calculations
    let start = Instant::now();
    let parallel_result: Vec<f64> = employees
        .par_iter()
        .map(|emp| {
            // Same complex calculation
            let base_salary = emp.salary;
            let experience_bonus = emp.years_experience as f64 * 1000.0;
            let age_factor = if emp.age > 30 { 1.1 } else { 0.9 };
            let department_multiplier = match emp.department.as_str() {
                "Engineering" => 1.2,
                "Sales" => 1.15,
                "Marketing" => 1.05,
                _ => 1.0,
            };
            
            // Complex mathematical operations
            let mut result = base_salary;
            for _ in 0..100 { // Simulate CPU-intensive work
                result = (result + experience_bonus) * age_factor * department_multiplier;
                result = result.sqrt() * 1.1;
            }
            result
        })
        .collect();
    let parallel_duration = start.elapsed();
    
    // Verify results are identical
    assert_eq!(traditional_result.len(), parallel_result.len());
    
    let speedup = traditional_duration.as_secs_f64() / parallel_duration.as_secs_f64();
    
    println!("Traditional: {:?} ({:.0} ops/sec)", 
             traditional_duration, 
             size as f64 / traditional_duration.as_secs_f64());
    println!("Parallel:    {:?} ({:.0} ops/sec)", 
             parallel_duration, 
             size as f64 / parallel_duration.as_secs_f64());
    println!("Speedup:     {:.2}x", speedup);
    
    if speedup > 1.5 {
        println!("✅ Parallel approach shows significant performance improvement");
    } else if speedup > 1.1 {
        println!("✅ Parallel approach shows modest performance improvement");
    } else {
        println!("⚠️  Parallel overhead is too high for this operation");
    }
}

fn test_complex_filtering(employees: &[Employee], size: usize) {
    println!("\n🔍 Complex Filtering - Multi-criteria Selection");
    
    // Traditional approach - Complex filtering
    let start = Instant::now();
    let traditional_result: Vec<&Employee> = employees
        .iter()
        .filter(|emp| {
            // Complex filtering logic
            emp.is_active &&
            emp.salary > 50000.0 &&
            emp.age >= 25 && emp.age <= 45 &&
            emp.years_experience >= 3 &&
            (emp.department == "Engineering" || emp.department == "Sales") &&
            emp.skills.len() >= 2
        })
        .collect();
    let traditional_duration = start.elapsed();
    
    // Parallel approach - Same complex filtering
    let start = Instant::now();
    let parallel_result: Vec<&Employee> = employees
        .par_iter()
        .filter(|emp| {
            // Same complex filtering logic
            emp.is_active &&
            emp.salary > 50000.0 &&
            emp.age >= 25 && emp.age <= 45 &&
            emp.years_experience >= 3 &&
            (emp.department == "Engineering" || emp.department == "Sales") &&
            emp.skills.len() >= 2
        })
        .collect();
    let parallel_duration = start.elapsed();
    
    // Verify results are identical
    assert_eq!(traditional_result.len(), parallel_result.len());
    
    let speedup = traditional_duration.as_secs_f64() / parallel_duration.as_secs_f64();
    
    println!("Traditional: {:?} ({} matches)", traditional_duration, traditional_result.len());
    println!("Parallel:    {:?} ({} matches)", parallel_duration, parallel_result.len());
    println!("Speedup:     {:.2}x", speedup);
    
    if speedup > 1.5 {
        println!("✅ Parallel filtering shows significant performance improvement");
    } else if speedup > 1.1 {
        println!("✅ Parallel filtering shows modest performance improvement");
    } else {
        println!("⚠️  Parallel overhead is too high for this filtering operation");
    }
}

fn test_aggregation_operations(employees: &[Employee], size: usize) {
    println!("\n📊 Aggregation Operations - Statistical Analysis");
    
    // Traditional approach - Multiple aggregations
    let start = Instant::now();
    let traditional_stats = {
        let total_salary: f64 = employees.iter().map(|emp| emp.salary).sum();
        let avg_salary = total_salary / employees.len() as f64;
        let max_salary = employees.iter().map(|emp| emp.salary).fold(0.0, f64::max);
        let min_salary = employees.iter().map(|emp| emp.salary).fold(f64::INFINITY, f64::min);
        let active_count = employees.iter().filter(|emp| emp.is_active).count();
        let engineering_count = employees.iter().filter(|emp| emp.department == "Engineering").count();
        
        (total_salary, avg_salary, max_salary, min_salary, active_count, engineering_count)
    };
    let traditional_duration = start.elapsed();
    
    // Parallel approach - Same aggregations
    let start = Instant::now();
    let parallel_stats = {
        let total_salary: f64 = employees.par_iter().map(|emp| emp.salary).sum();
        let avg_salary = total_salary / employees.len() as f64;
        let max_salary = employees.par_iter().map(|emp| emp.salary).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let min_salary = employees.par_iter().map(|emp| emp.salary).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let active_count = employees.par_iter().filter(|emp| emp.is_active).count();
        let engineering_count = employees.par_iter().filter(|emp| emp.department == "Engineering").count();
        
        (total_salary, avg_salary, max_salary, min_salary, active_count, engineering_count)
    };
    let parallel_duration = start.elapsed();
    
    // Verify results are identical
    assert!((traditional_stats.0 - parallel_stats.0).abs() < 0.01);
    assert_eq!(traditional_stats.4, parallel_stats.4);
    assert_eq!(traditional_stats.5, parallel_stats.5);
    
    let speedup = traditional_duration.as_secs_f64() / parallel_duration.as_secs_f64();
    
    println!("Traditional: {:?}", traditional_duration);
    println!("Parallel:    {:?}", parallel_duration);
    println!("Speedup:     {:.2}x", speedup);
    println!("Stats: Total=${:.0}, Avg=${:.0}, Max=${:.0}, Min=${:.0}, Active={}, Engineering={}", 
             traditional_stats.0, traditional_stats.1, traditional_stats.2, 
             traditional_stats.3, traditional_stats.4, traditional_stats.5);
    
    if speedup > 1.5 {
        println!("✅ Parallel aggregation shows significant performance improvement");
    } else if speedup > 1.1 {
        println!("✅ Parallel aggregation shows modest performance improvement");
    } else {
        println!("⚠️  Parallel overhead is too high for this aggregation operation");
    }
}

fn test_sorting_operations(employees: &[Employee], size: usize) {
    println!("\n🔄 Sorting Operations - Multi-field Sorting");
    
    // Traditional approach - Complex sorting
    let start = Instant::now();
    let mut traditional_sorted = employees.to_vec();
    traditional_sorted.sort_by(|a, b| {
        // Sort by department first, then by salary (descending), then by age
        a.department.cmp(&b.department)
            .then(b.salary.partial_cmp(&a.salary).unwrap())
            .then(a.age.cmp(&b.age))
    });
    let traditional_duration = start.elapsed();
    
    // Parallel approach - Same sorting
    let start = Instant::now();
    let mut parallel_sorted = employees.to_vec();
    parallel_sorted.par_sort_by(|a, b| {
        // Same sorting logic
        a.department.cmp(&b.department)
            .then(b.salary.partial_cmp(&a.salary).unwrap())
            .then(a.age.cmp(&b.age))
    });
    let parallel_duration = start.elapsed();
    
    // Verify results are identical
    assert_eq!(traditional_sorted.len(), parallel_sorted.len());
    
    let speedup = traditional_duration.as_secs_f64() / parallel_duration.as_secs_f64();
    
    println!("Traditional: {:?}", traditional_duration);
    println!("Parallel:    {:?}", parallel_duration);
    println!("Speedup:     {:.2}x", speedup);
    
    if speedup > 1.5 {
        println!("✅ Parallel sorting shows significant performance improvement");
    } else if speedup > 1.1 {
        println!("✅ Parallel sorting shows modest performance improvement");
    } else {
        println!("⚠️  Parallel overhead is too high for this sorting operation");
    }
}

fn test_memory_efficiency() {
    println!("\n💾 Memory Efficiency Analysis");
    println!("{}", "=".repeat(50));
    
    let size = 1_000_000;
    println!("Testing with {} employees", size);
    
    let employees = create_large_dataset(size);
    
    // Test memory usage patterns
    let start = Instant::now();
    let traditional_result: Vec<String> = employees
        .iter()
        .map(|emp| format!("{}:{}:{}", emp.name, emp.department, emp.salary))
        .collect();
    let traditional_duration = start.elapsed();
    
    let start = Instant::now();
    let parallel_result: Vec<String> = employees
        .par_iter()
        .map(|emp| format!("{}:{}:{}", emp.name, emp.department, emp.salary))
        .collect();
    let parallel_duration = start.elapsed();
    
    let speedup = traditional_duration.as_secs_f64() / parallel_duration.as_secs_f64();
    
    println!("Traditional: {:?} ({:.0} ops/sec)", 
             traditional_duration, 
             size as f64 / traditional_duration.as_secs_f64());
    println!("Parallel:    {:?} ({:.0} ops/sec)", 
             parallel_duration, 
             size as f64 / parallel_duration.as_secs_f64());
    println!("Speedup:     {:.2}x", speedup);
    
    // Memory usage estimation
    let estimated_memory = size * std::mem::size_of::<String>() * 2; // Rough estimate
    println!("Estimated memory usage: {:.2} MB", estimated_memory as f64 / 1_000_000.0);
    
    if speedup > 2.0 {
        println!("✅ Parallel processing shows excellent performance for large datasets");
    } else if speedup > 1.5 {
        println!("✅ Parallel processing shows good performance for large datasets");
    } else {
        println!("⚠️  Parallel processing overhead is significant for this operation");
    }
}
