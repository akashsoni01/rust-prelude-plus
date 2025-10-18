use rust_prelude_plus::prelude::*;
use key_paths_derive::Keypath;
use std::rc::Rc;
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Keypath, Debug, Clone, PartialEq)]
struct Person {
    id: u32,
    name: String,
    age: u32,
    email: String,
    department: String,
    salary: f64,
    is_active: bool,
    address: Address,
    skills: Vec<String>,
}

#[derive(Keypath, Debug, Clone, PartialEq)]
struct Address {
    street: String,
    city: String,
    country: String,
    coordinates: Coordinates,
}

#[derive(Keypath, Debug, Clone, PartialEq)]
struct Coordinates {
    latitude: f64,
    longitude: f64,
}

fn main() {
    println!("=== Comprehensive KeyPath Examples ===\n");

    // Create sample data using Rc for memory efficiency
    let people = create_sample_data();

    // 1. Basic KeyPath Operations
    demonstrate_basic_operations(&people);

    // 2. Iterator and Lazy Evaluation
    demonstrate_iterator_operations(&people);

    // 3. Memory Efficient Operations with Rc
    demonstrate_memory_efficient_operations(&people);

    // 4. Composable Operations
    demonstrate_composable_operations(&people);

    // 5. Collection Operations
    demonstrate_collection_operations(&people);

    // 6. Error Handling
    demonstrate_error_handling(&people);

    // 7. Performance Comparison
    demonstrate_performance_comparison(&people);
}

fn create_sample_data() -> Vec<Rc<Person>> {
    vec![
        Rc::new(Person {
            id: 1,
            name: "Alice Johnson".to_string(),
            age: 30,
            email: "alice@company.com".to_string(),
            department: "Engineering".to_string(),
            salary: 95000.0,
            is_active: true,
            address: Address {
                street: "123 Main St".to_string(),
                city: "San Francisco".to_string(),
                country: "USA".to_string(),
                coordinates: Coordinates { latitude: 37.7749, longitude: -122.4194 },
            },
            skills: vec!["Rust".to_string(), "Python".to_string(), "JavaScript".to_string()],
        }),
        Rc::new(Person {
            id: 2,
            name: "Bob Smith".to_string(),
            age: 25,
            email: "bob@company.com".to_string(),
            department: "Marketing".to_string(),
            salary: 65000.0,
            is_active: true,
            address: Address {
                street: "456 Oak Ave".to_string(),
                city: "New York".to_string(),
                country: "USA".to_string(),
                coordinates: Coordinates { latitude: 40.7128, longitude: -74.0060 },
            },
            skills: vec!["Marketing".to_string(), "Analytics".to_string()],
        }),
        Rc::new(Person {
            id: 3,
            name: "Charlie Brown".to_string(),
            age: 35,
            email: "charlie@company.com".to_string(),
            department: "Engineering".to_string(),
            salary: 110000.0,
            is_active: false,
            address: Address {
                street: "789 Pine St".to_string(),
                city: "Seattle".to_string(),
                country: "USA".to_string(),
                coordinates: Coordinates { latitude: 47.6062, longitude: -122.3321 },
            },
            skills: vec!["Java".to_string(), "C++".to_string(), "System Design".to_string()],
        }),
        Rc::new(Person {
            id: 4,
            name: "Diana Prince".to_string(),
            age: 28,
            email: "diana@company.com".to_string(),
            department: "HR".to_string(),
            salary: 75000.0,
            is_active: true,
            address: Address {
                street: "321 Elm St".to_string(),
                city: "Boston".to_string(),
                country: "USA".to_string(),
                coordinates: Coordinates { latitude: 42.3601, longitude: -71.0589 },
            },
            skills: vec!["HR Management".to_string(), "Recruitment".to_string()],
        }),
    ]
}

fn demonstrate_basic_operations(people: &[Rc<Person>]) {
    println!("1. BASIC KEYPATH OPERATIONS:");
    
    // Map operation
    let names: Vec<String> = people
        .clone()
        .into_iter()
        .map_keypath(Person::name(), |name| name.clone())
        .collect();
    println!("   Names: {:?}", names);

    // Filter operation
    let active_people: Vec<Rc<Person>> = people
        .clone().into_iter()
        .filter_by_keypath(Person::is_active(), |&active| active)
        ;
    println!("   Active people count: {}", active_people.len());

    // Find operation
    let young_person = people
        .clone().into_iter()
        .find_by_keypath(Person::age(), |&age| age < 30)
        .unwrap()
        .unwrap();
    println!("   Young person: {}", young_person.name);

    // Fold operation
    let total_salary: f64 = people
        .clone().into_iter()
        .fold_keypath(Person::salary(), 0.0, |acc, &salary| acc + salary)
        .unwrap();
    println!("   Total salary: ${:.2}", total_salary);

    println!();
}

fn demonstrate_iterator_operations(people: &[Rc<Person>]) {
    println!("2. ITERATOR AND LAZY EVALUATION:");
    
    // Lazy evaluation - no intermediate collections
    let engineering_seniors: Vec<String> = people
        .clone().into_iter()
        .filter_by_keypath(Person::department(), |dept| dept == "Engineering")
        .filter_by_keypath(Person::age(), |&age| age >= 30)
        .map_keypath(Person::name(), |name| name.clone())
        .collect();
    println!("   Engineering seniors: {:?}", engineering_seniors);

    // Chained operations with take
    let top_earners: Vec<String> = people
        .clone().into_iter()
        .filter_by_keypath(Person::is_active(), |&active| active)
        .map_keypath(Person::salary(), |&salary| salary)
        .enumerate()
        .filter(|(_, &salary)| salary > 80000.0)
        .map(|(i, _)| people[i].name.clone())
        .take(2)
        .collect();
    println!("   Top earners: {:?}", top_earners);

    // Nested keypath access
    let cities: Vec<String> = people
        .clone().into_iter()
        .map_keypath(Person::address().then(Address::city()), |city| city.clone())
        .collect();
    println!("   Cities: {:?}", cities);

    println!();
}

fn demonstrate_memory_efficient_operations(people: &[Rc<Person>]) {
    println!("3. MEMORY EFFICIENT OPERATIONS WITH RC:");
    
    // Using Rc to avoid cloning large structures
    let engineering_team: Vec<Rc<Person>> = people
        .clone().into_iter()
        .filter_by_keypath(Person::department(), |dept| dept == "Engineering")
        .map(|person| person) // Only clones the Rc, not the Person
        .collect();
    println!("   Engineering team size: {}", engineering_team.len());

    // Group by department using Rc
    let mut department_groups: HashMap<String, Vec<Rc<Person>>> = HashMap::new();
    for person in people {
        let dept = person.department.clone();
        department_groups.entry(dept).or_insert_with(Vec::new).push(person.clone());
    }
    println!("   Departments: {:?}", department_groups.keys().collect::<Vec<_>>());

    // Memory efficient filtering
    let high_salary_people: Vec<&Rc<Person>> = people
        .clone().into_iter()
        .filter_by_keypath(Person::salary(), |&salary| salary > 80000.0)
        .collect();
    println!("   High salary people count: {}", high_salary_people.len());

    println!();
}

fn demonstrate_composable_operations(people: &[Rc<Person>]) {
    println!("4. COMPOSABLE OPERATIONS:");
    
    // Using pipe for function composition
    let result = pipe(people, |people| {
        people.iter()
            .filter_by_keypath(Person::is_active(), |&active| active)
            .filter_by_keypath(Person::age(), |&age| age >= 25)
            .map_keypath(Person::name(), |name| name.to_uppercase())
            .collect::<Vec<_>>()
    });
    println!("   Active adults (uppercase): {:?}", result);

    // Chaining operations
    let senior_engineers: Vec<String> = people
        .clone().into_iter()
        .filter_by_keypath(Person::department(), |dept| dept == "Engineering")
        .filter_by_keypath(Person::age(), |&age| age >= 30)
        .filter_by_keypath(Person::is_active(), |&active| active)
        .map_keypath(Person::name(), |name| name.clone())
        .collect();
    println!("   Senior engineers: {:?}", senior_engineers);

    // Complex nested operations
    let us_employees: Vec<String> = people
        .clone().into_iter()
        .filter_by_keypath(Person::address().then(Address::country()), |country| country == "USA")
        .map_keypath(Person::name(), |name| name.clone())
        .collect();
    println!("   US employees: {:?}", us_employees);

    println!();
}

fn demonstrate_collection_operations(people: &[Rc<Person>]) {
    println!("5. COLLECTION OPERATIONS:");
    
    // Group by department
    let grouped_by_dept = group_by_keypath(people, Person::department(), |dept| dept.clone()).unwrap();
    println!("   Grouped by department:");
    for (dept, people_in_dept) in &grouped_by_dept {
        println!("     {}: {} people", dept, people_in_dept.len());
    }

    // Partition by active status
    let (active, inactive) = partition_by_keypath(people.to_vec(), Person::is_active(), |&active| active).unwrap();
    println!("   Active: {}, Inactive: {}", active.len(), inactive.len());

    // Sort by salary
    let mut sorted_people = people.to_vec();
    sort_by_keypath(&mut sorted_people, Person::salary(), |a, b| a.partial_cmp(b).unwrap()).unwrap();
    println!("   Sorted by salary (ascending):");
    for person in &sorted_people {
        println!("     {}: ${:.0}", person.name, person.salary);
    }

    // Collect specific values
    let emails = collect_keypath(people.to_vec(), Person::email()).unwrap();
    println!("   Emails: {:?}", emails);

    println!();
}

fn demonstrate_error_handling(people: &[Rc<Person>]) {
    println!("6. ERROR HANDLING:");
    
    // Safe keypath access
    let result: KeyPathResult<Vec<String>> = people
        .clone().into_iter()
        .map_keypath(Person::name(), |name| name.clone())
        .collect::<Vec<_>>()
        .into_iter()
        .map(|name| Ok(name))
        .collect();
    
    match result {
        Ok(names) => println!("   Successfully extracted names: {:?}", names),
        Err(e) => println!("   Error: {:?}", e),
    }

    // Handling empty collections
    let empty_people: Vec<Rc<Person>> = vec![];
    let empty_result = filter_by_keypath(empty_people, Person::age(), |&age| age > 30).unwrap();
    println!("   Empty collection result: {} items", empty_result.len());

    println!();
}

fn demonstrate_performance_comparison(people: &[Rc<Person>]) {
    println!("7. PERFORMANCE COMPARISON:");
    
    // Traditional approach
    let start = std::time::Instant::now();
    let traditional_result: Vec<String> = people
        .clone().into_iter()
        .filter(|p| p.department == "Engineering" && p.is_active)
        .map(|p| p.name.clone())
        .collect();
    let traditional_time = start.elapsed();
    
    // KeyPath approach
    let start = std::time::Instant::now();
    let keypath_result: Vec<String> = people
        .clone().into_iter()
        .filter_by_keypath(Person::department(), |dept| dept == "Engineering")
        .filter_by_keypath(Person::is_active(), |&active| active)
        .map_keypath(Person::name(), |name| name.clone())
        .collect();
    let keypath_time = start.elapsed();
    
    println!("   Traditional approach: {:?} - {:?}", traditional_time, traditional_result);
    println!("   KeyPath approach: {:?} - {:?}", keypath_time, keypath_result);
    println!("   Results match: {}", traditional_result == keypath_result);
    
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let people = create_sample_data();
        
        // Test map operation
        let names: Vec<String> = people
            .clone().into_iter()
            .map_keypath(Person::name(), |name| name.clone())
            .collect();
        assert_eq!(names.len(), 4);
        assert!(names.contains(&"Alice Johnson".to_string()));
        
        // Test filter operation
        let active_people: Vec<Rc<Person>> = people
            .clone().into_iter()
            .filter_by_keypath(Person::is_active(), |&active| active)
            .map(|person| person)
            .collect();
        assert_eq!(active_people.len(), 3); // 3 active people
        
        // Test find operation
        let young_person = people
            .clone().into_iter()
            .find_by_keypath(Person::age(), |&age| age < 30)
            .unwrap();
        assert_eq!(young_person.name, "Bob Smith");
    }

    #[test]
    fn test_memory_efficiency() {
        let people = create_sample_data();
        
        // Test that Rc cloning is efficient
        let cloned_people: Vec<Rc<Person>> = people
            .clone().into_iter()
            .map(|person| person)
            .collect();
        
        assert_eq!(people.len(), cloned_people.len());
        
        // Test that the same data is referenced
        for (original, cloned) in people.iter().zip(cloned_people.iter()) {
            assert!(Rc::ptr_eq(original, cloned));
        }
    }

    #[test]
    fn test_composable_operations() {
        let people = create_sample_data();
        
        // Test pipe operation
        let result = pipe(people, |people| {
            people.iter()
                .filter_by_keypath(Person::is_active(), |&active| active)
                .map_keypath(Person::name(), |name| name.clone())
                .collect::<Vec<_>>()
        });
        
        assert_eq!(result.len(), 3); // 3 active people
        assert!(result.contains(&"Alice Johnson".to_string()));
    }

    #[test]
    fn test_collection_operations() {
        let people = create_sample_data();
        
        // Test group by department
        let grouped = group_by_keypath(people, Person::department(), |dept| dept.clone()).unwrap();
        assert_eq!(grouped.len(), 3); // 3 departments
        assert_eq!(grouped["Engineering"].len(), 2);
        assert_eq!(grouped["Marketing"].len(), 1);
        assert_eq!(grouped["HR"].len(), 1);
        
        // Test partition
        let (active, inactive) = partition_by_keypath(people.to_vec(), Person::is_active(), |&active| active).unwrap();
        assert_eq!(active.len(), 3);
        assert_eq!(inactive.len(), 1);
    }

    #[test]
    fn test_error_handling() {
        let people = create_sample_data();
        
        // Test successful operation
        let result: KeyPathResult<Vec<String>> = people
            .clone().into_iter()
            .map_keypath(Person::name(), |name| name.clone())
            .collect::<Vec<_>>()
            .into_iter()
            .map(|name| Ok(name))
            .collect();
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 4);
        
        // Test empty collection
        let empty_people: Vec<Rc<Person>> = vec![];
        let empty_result = filter_by_keypath(empty_people, Person::age(), |&age| age > 30).unwrap();
        assert_eq!(empty_result.len(), 0);
    }
}
