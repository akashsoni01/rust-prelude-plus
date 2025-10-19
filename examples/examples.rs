//! Comprehensive examples for keypath operations

use key_paths_derive::Keypath;
use rust_prelude_plus::prelude::*;
use rust_prelude_plus::traits::{KeyPathsIterator, KeyPathsOperable};
use rust_prelude_plus::collections::KeyPathsCollectionExt;

/// Example data structure for demonstrating keypath operations
#[derive(Keypath, Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub email: String,
    pub department: String,
    pub salary: f64,
    pub is_active: bool,
}

fn create_sample_people() -> Vec<Person> {
    vec![
        Person {
            name: "Alice".to_string(),
            age: 30,
            email: "alice@example.com".to_string(),
            department: "Engineering".to_string(),
            salary: 90000.0,
            is_active: true,
        },
        Person {
            name: "Bob".to_string(),
            age: 25,
            email: "bob@example.com".to_string(),
            department: "Marketing".to_string(),
            salary: 60000.0,
            is_active: false,
        },
        Person {
            name: "Charlie".to_string(),
            age: 35,
            email: "charlie@example.com".to_string(),
            department: "Engineering".to_string(),
            salary: 135000.0,
            is_active: true,
        },
        Person {
            name: "Diana".to_string(),
            age: 28,
            email: "diana@example.com".to_string(),
            department: "HR".to_string(),
            salary: 70000.0,
            is_active: true,
        },
    ]
}

fn test_basic_operations(people: &[Person]) {
    println!("\n=== Basic Operations ===");
    
    // Map operation
    let names = map_keypath_collection(people, Person::name(), |name: &String| name.clone()).unwrap();
    println!("Names: {:?}", names);
    
    // Filter operation
    let young_people = filter_by_keypath(people.to_vec(), Person::age(), |&age| age < 30).unwrap();
    println!("Young people count: {}", young_people.len());
    
    // Find operation
    let young_person = find_by_keypath(people.to_vec(), Person::age(), |&age| age < 30).unwrap();
    if let Some(person) = young_person {
        println!("Young person: {}", person.name);
    }
    
    // Fold operation
    let total_salary = fold_keypath(people.to_vec(), Person::salary(), 0.0, |acc, &salary| acc + salary).unwrap();
    println!("Total salary: {}", total_salary);
}

fn test_iterator_extensions(people: &[Person]) {
    println!("\n=== Iterator Extensions ===");
    
    // Test iterator extensions
    let names_from_iter: Vec<String> = people
        .to_vec()
        .into_iter()
        .map_keypath(Person::name(), |name: &String| name.clone());
    println!("Names from iterator: {:?}", names_from_iter);
    
    let young_people_from_iter: Vec<Person> = people
        .to_vec()
            .into_iter()
        .filter_by_keypath(Person::age(), |&age| age < 30);
    println!("Young people from iterator: {}", young_people_from_iter.len());
}

fn test_collection_operations(people: &[Person]) {
    println!("\n=== Collection Operations ===");
    
    // Convert to Vec for collection operations
    let people_vec = people.to_vec();
    
    // Test collection extensions
    let names_from_collection = people_vec.collect_keypath(Person::name()).unwrap();
    println!("Names from collection: {:?}", names_from_collection);
    
    let (active, inactive) = KeyPathsCollectionExt::partition_by_keypath(&people_vec, Person::is_active(), |&is_active| is_active).unwrap();
    println!("Active: {}, Inactive: {}", active.len(), inactive.len());
    
    // Group by department
    let grouped = KeyPathsCollectionExt::group_by_keypath(&people_vec, Person::department(), |dept: &String| dept.clone()).unwrap();
    println!("Grouped by department:");
    for (dept, people_in_dept) in grouped {
        println!("  {}: {} people", dept, people_in_dept.len());
    }
}

fn test_composable_operations(people: &[Person]) {
    println!("\n=== Composable Operations ===");
    
    // Test pipe operation
    let result = pipe(people.to_vec(), |people| {
        let filtered = people.into_iter()
            .filter_by_keypath(Person::age(), |&age| age >= 30);
        filtered.into_iter()
            .map_keypath(Person::name(), |name: &String| name.clone())
    });
    println!("Pipe result: {:?}", result);
}

fn test_error_handling(people: &[Person]) {
    println!("\n=== Error Handling ===");
    
    // Test safe keypath access
        for person in people {
        match person.get_at_keypath(&Person::name()) {
            Ok(name) => {
                println!("Person name: {}", name);
                }
                Err(e) => {
                eprintln!("Error accessing name: {}", e);
            }
        }
    }
}

fn test_performance_comparison(people: &[Person]) {
    println!("\n=== Performance Comparison ===");
    
    use std::time::Instant;
    
    // Traditional approach
    let start = Instant::now();
    let traditional_names: Vec<String> = people.iter().map(|p| p.name.clone()).collect();
    let traditional_time = start.elapsed();
    
    // KeyPath approach
    let start = Instant::now();
    let people_vec = people.to_vec();
    let keypath_names = people_vec.collect_keypath(Person::name()).unwrap();
    let keypath_time = start.elapsed();
    
    println!("Traditional approach: {:?}", traditional_time);
    println!("KeyPath approach: {:?}", keypath_time);
    println!("Results match: {}", traditional_names == keypath_names);
}

fn main() {
    println!("=== Comprehensive KeyPath Examples ===");
    
    // Create sample data
    let people = create_sample_people();
    
    // Test all operations
    test_basic_operations(&people);
    test_iterator_extensions(&people);
    test_collection_operations(&people);
    test_composable_operations(&people);
    test_error_handling(&people);
    test_performance_comparison(&people);
    
    println!("\n=== All examples completed successfully! ===");
}