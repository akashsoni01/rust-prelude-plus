use rust_prelude_plus::prelude::*;
use key_paths_derive::Keypath;

#[derive(Keypath, Debug, Clone, PartialEq)]
struct Person {
    name: String,
    age: u32,
    salary: f64,
}

fn main() {
    println!("=== Simple Working KeyPath Example ===\n");

    // Create sample data
    let people = vec![
        Person { name: "Alice".to_string(), age: 30, salary: 75000.0 },
        Person { name: "Bob".to_string(), age: 25, salary: 65000.0 },
        Person { name: "Charlie".to_string(), age: 35, salary: 85000.0 },
    ];

    // Test basic higher-order functions
    println!("1. Basic Higher-Order Functions:");
    
    // Map operation
    let names = map_keypath_collection(&people, Person::name(), |name: &String| name.clone()).unwrap();
    println!("   Names: {:?}", names);
    
    // Filter operation
    let young_people = filter_by_keypath(people.clone(), Person::age(), |&age| age < 30).unwrap();
    println!("   Young people count: {}", young_people.len());
    
    // Find operation
    let high_earner = find_by_keypath(people.clone(), Person::salary(), |&salary| salary > 80000.0).unwrap();
    if let Some(person) = high_earner {
        println!("   High earner: {}", person.name);
    }
    
    // Fold operation
    let total_salary = fold_keypath(people.clone(), Person::salary(), 0.0, |acc, &salary| acc + salary).unwrap();
    println!("   Total salary: {}", total_salary);
    
    // Group operation
    let grouped_by_age = group_by_keypath(&people, Person::age(), |&age| age / 10 * 10).unwrap();
    println!("   Age groups: {:?}", grouped_by_age.keys().collect::<Vec<_>>());
    
    // Sort operation
    let mut sorted_people = people.clone();
    sort_by_keypath(&mut sorted_people, Person::salary(), |a, b| a.partial_cmp(b).unwrap()).unwrap();
    println!("   Sorted by salary: {:?}", sorted_people.iter().map(|p| &p.name).collect::<Vec<_>>());
    
    // Collect operation
    let ages = collect_keypath(people.clone(), Person::age()).unwrap();
    println!("   Ages: {:?}", ages);
    
    // Partition operation
    let (high_earners, low_earners) = partition_by_keypath(people.clone(), Person::salary(), |&salary| salary > 70000.0).unwrap();
    println!("   High earners: {}, Low earners: {}", high_earners.len(), low_earners.len());
    
    println!("\n2. Iterator Extensions:");
    
    // Test iterator extensions
    let names_from_iter: Vec<String> = people
        .clone()
        .into_iter()
        .map_keypath(Person::name(), |name: &String| name.clone());
    println!("   Names from iterator: {:?}", names_from_iter);
    
    let young_people_from_iter: Vec<Person> = people
        .clone()
        .into_iter()
        .filter_by_keypath(Person::age(), |&age| age < 30);
    println!("   Young people from iterator: {}", young_people_from_iter.len());
    
    println!("\n3. Collection Extensions:");
    
    // Test collection extensions
    let names_from_collection = people.collect_keypath(Person::name()).unwrap();
    println!("   Names from collection: {:?}", names_from_collection);
    
    let (active, inactive) = <Vec<Person> as KeyPathsCollectionExt<Person>>::partition_by_keypath(&people, Person::age(), |&age| age >= 30).unwrap();
    println!("   Active (30+): {}, Inactive (<30): {}", active.len(), inactive.len());
    
    println!("\n4. Composable Operations:");
    
    // Test pipe operation
    let result = pipe(people.clone(), |people| {
        let filtered = people.into_iter()
            .filter_by_keypath(Person::age(), |&age| age >= 30);
        filtered.into_iter()
            .map_keypath(Person::name(), |name: &String| name.clone())
    });
    println!("   Pipe result: {:?}", result);
    
    println!("\n=== Example completed successfully! ===");
}
