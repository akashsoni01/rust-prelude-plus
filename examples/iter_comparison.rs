use rust_prelude_plus::prelude::*;
use key_paths_derive::Keypath;
use std::collections::HashMap;

#[derive(Keypath, Debug, Clone, PartialEq)]
struct Person {
    name: String,
    age: u32,
    department: String,
    salary: f64,
    is_active: bool,
}

fn main() {
    let people = vec![
        Person { 
            name: "Alice".to_string(), 
            age: 30, 
            department: "Engineering".to_string(),
            salary: 75000.0,
            is_active: true,
        },
        Person { 
            name: "Bob".to_string(), 
            age: 25, 
            department: "Engineering".to_string(),
            salary: 65000.0,
            is_active: true,
        },
        Person { 
            name: "Charlie".to_string(), 
            age: 35, 
            department: "HR".to_string(),
            salary: 55000.0,
            is_active: false,
        },
        Person { 
            name: "Diana".to_string(), 
            age: 28, 
            department: "Engineering".to_string(),
            salary: 70000.0,
            is_active: true,
        },
    ];

    println!("=== Iterator vs KeyPath Functional Programming Comparison ===\n");

    // 1. Traditional iter() - Borrows references
    println!("1. Traditional iter() - Borrows references:");
    println!("   Original data is still available after iteration");
    let names_iter: Vec<String> = people
        .iter()  // Borrows &Person
        .map(|p| p.name.clone())  // Clone needed because we only have &Person
        .collect();
    println!("   Names: {:?}", names_iter);
    println!("   Original people count: {}\n", people.len());

    // 2. Traditional into_iter() - Takes ownership
    println!("2. Traditional into_iter() - Takes ownership:");
    println!("   Original data is consumed and no longer available");
    let names_into: Vec<String> = people
        .into_iter()  // Takes ownership of Person
        .map(|p| p.name)  // No clone needed, we own Person
        .collect();
    println!("   Names: {:?}", names_into);
    // println!("   Original people count: {}", people.len()); // This would fail!
    println!("   Original people vector is now consumed\n");

    // Recreate people for further examples
    let people = vec![
        Person { 
            name: "Alice".to_string(), 
            age: 30, 
            department: "Engineering".to_string(),
            salary: 75000.0,
            is_active: true,
        },
        Person { 
            name: "Bob".to_string(), 
            age: 25, 
            department: "Engineering".to_string(),
            salary: 65000.0,
            is_active: true,
        },
        Person { 
            name: "Charlie".to_string(), 
            age: 35, 
            department: "HR".to_string(),
            salary: 55000.0,
            is_active: false,
        },
        Person { 
            name: "Diana".to_string(), 
            age: 28, 
            department: "Engineering".to_string(),
            salary: 70000.0,
            is_active: true,
        },
    ];

    // 3. KeyPath Functional Programming - Type-safe field access
    println!("3. KeyPath Functional Programming - Type-safe field access:");
    println!("   Compile-time guarantees about field existence and types");
    let names_keypath = collect_keypath(people.clone(), Person::name()).unwrap();
    println!("   Names: {:?}", names_keypath);
    println!("   Original people count: {}\n", people.len());

    // 4. Comparison: Filter operations
    println!("4. Filter Operations Comparison:");
    
    // Traditional iter() filter
    let young_traditional: Vec<&Person> = people
        .iter()
        .filter(|p| p.age < 30)
        .collect();
    println!("   Traditional iter() filter (references): {} people", young_traditional.len());
    
    // Traditional into_iter() filter
    let young_into: Vec<Person> = people
        .clone()
        .into_iter()
        .filter(|p| p.age < 30)
        .collect();
    println!("   Traditional into_iter() filter (owned): {} people", young_into.len());
    
    // KeyPath filter
    let young_keypath = filter_by_keypath(people.clone(), Person::age(), |&age| age < 30).unwrap();
    println!("   KeyPath filter (owned): {} people\n", young_keypath.len());

    // 5. Comparison: Complex transformations
    println!("5. Complex Transformations Comparison:");
    
    // Traditional iter() - Multiple operations
    let high_earners_traditional: Vec<String> = people
        .iter()
        .filter(|p| p.salary > 60000.0 && p.is_active)
        .map(|p| p.name.to_uppercase())
        .collect();
    println!("   Traditional iter() chain: {:?}", high_earners_traditional);
    
    // KeyPath - Multiple operations with type safety
    let high_earners_keypath = filter_by_keypath(people.clone(), Person::salary(), |&salary| salary > 60000.0).unwrap();
    let active_high_earners = filter_by_keypath(high_earners_keypath, Person::is_active(), |&active| active).unwrap();
    let high_earner_names = map_keypath_collection(&active_high_earners, Person::name(), |name| name.to_uppercase()).unwrap();
    println!("   KeyPath chain: {:?}\n", high_earner_names);

    // 6. Comparison: Grouping operations
    println!("6. Grouping Operations Comparison:");
    
    // Traditional iter() grouping
    let mut groups_traditional: HashMap<String, Vec<&Person>> = HashMap::new();
    for person in people.iter() {
        let key = if person.age < 30 { "young" } else { "experienced" };
        groups_traditional.entry(key.to_string()).or_insert_with(Vec::new).push(person);
    }
    println!("   Traditional grouping (references):");
    for (key, group) in &groups_traditional {
        println!("     {}: {} people", key, group.len());
    }
    
    // KeyPath grouping
    let groups_keypath = group_by_keypath(&people, Person::age(), |&age| {
        if age < 30 { "young" } else { "experienced" }
    }).unwrap();
    println!("   KeyPath grouping (owned):");
    for (key, group) in &groups_keypath {
        println!("     {}: {} people", key, group.len());
    }
    println!();

    // 7. Comparison: Error handling
    println!("7. Error Handling Comparison:");
    
    // Traditional - No compile-time guarantees
    println!("   Traditional approach:");
    println!("     - No compile-time field validation");
    println!("     - Runtime errors if field doesn't exist");
    println!("     - Manual error handling required");
    
    // KeyPath - Compile-time guarantees
    println!("   KeyPath approach:");
    println!("     - Compile-time field validation");
    println!("     - Type-safe field access");
    println!("     - Built-in error handling");
    
    // Example of type safety
    let result = collect_keypath(people.clone(), Person::name()).unwrap();
    println!("     - Guaranteed to work: {:?}\n", result);

    // 8. Comparison: Performance and Memory
    println!("8. Performance and Memory Comparison:");
    
    // Traditional iter() - Memory efficient, borrows
    let start = std::time::Instant::now();
    let _names_iter = people.iter().map(|p| &p.name).collect::<Vec<_>>();
    let iter_time = start.elapsed();
    println!("   Traditional iter() (borrows): {:?}", iter_time);
    
    // Traditional into_iter() - Takes ownership
    let start = std::time::Instant::now();
    let _names_into = people.clone().into_iter().map(|p| p.name).collect::<Vec<_>>();
    let into_time = start.elapsed();
    println!("   Traditional into_iter() (owned): {:?}", into_time);
    
    // KeyPath - Type-safe with minimal overhead
    let start = std::time::Instant::now();
    let _names_keypath = collect_keypath(people.clone(), Person::name()).unwrap();
    let keypath_time = start.elapsed();
    println!("   KeyPath (owned): {:?}\n", keypath_time);

    // 9. Functional Programming Benefits
    println!("9. Functional Programming Benefits with KeyPaths:");
    println!("   ✅ Type Safety: Compile-time field validation");
    println!("   ✅ Reusability: KeyPaths defined once, used many times");
    println!("   ✅ Maintainability: Single point of change for field access");
    println!("   ✅ Error Prevention: Catches field access errors at compile time");
    println!("   ✅ Composability: Easy to chain operations");
    println!("   ✅ Readability: Clear intent with field-specific operations");
    println!("   ✅ Performance: Minimal overhead with safety guarantees");

    // 10. Real-world example: Employee analytics
    println!("\n10. Real-world Example: Employee Analytics");
    
    // Get active employees
    let active_employees = filter_by_keypath(people.clone(), Person::is_active(), |&active| active).unwrap();
    println!("   Active employees: {}", active_employees.len());
    
    // Calculate average salary of active employees
    let total_salary = fold_keypath(active_employees.clone(), Person::salary(), 0.0, |acc, &salary| acc + salary).unwrap();
    let avg_salary = total_salary / active_employees.len() as f64;
    println!("   Average salary of active employees: ${:.0}", avg_salary);
    
    // Group active employees by department
    let dept_groups = group_by_keypath(&active_employees, Person::department(), |dept| dept.clone()).unwrap();
    println!("   Active employees by department:");
    for (dept, employees) in &dept_groups {
        let dept_total = fold_keypath(employees.clone(), Person::salary(), 0.0, |acc, &salary| acc + salary).unwrap();
        let dept_avg = dept_total / employees.len() as f64;
        println!("     {}: {} employees, avg salary: ${:.0}", dept, employees.len(), dept_avg);
    }
}
