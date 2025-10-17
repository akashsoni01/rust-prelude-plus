use rust_prelude_plus::prelude::*;
use key_paths_derive::Keypath;

#[derive(Keypath, Debug, Clone, PartialEq)]
struct Person {
    name: String,
    age: u32,
    department: String,
    salary: f64,
    skills: Vec<String>,
}

fn main() {
    let mut people = vec![
        Person { 
            name: "Alice".to_string(), 
            age: 30, 
            department: "Engineering".to_string(),
            salary: 75000.0,
            skills: vec!["Rust".to_string(), "Python".to_string()],
        },
        Person { 
            name: "Bob".to_string(), 
            age: 25, 
            department: "Engineering".to_string(),
            salary: 65000.0,
            skills: vec!["JavaScript".to_string(), "React".to_string()],
        },
        Person { 
            name: "Charlie".to_string(), 
            age: 35, 
            department: "HR".to_string(),
            salary: 55000.0,
            skills: vec!["Communication".to_string(), "Recruitment".to_string()],
        },
        Person { 
            name: "Diana".to_string(), 
            age: 28, 
            department: "Engineering".to_string(),
            salary: 70000.0,
            skills: vec!["Rust".to_string(), "Go".to_string()],
        },
    ];

    println!("=== Collection Operations Examples ===\n");

    // 1. Collect keypath values
    println!("1. Collect all names:");
    let names = collect_keypath(people.clone(), Person::name()).unwrap();
    println!("   Names: {:?}\n", names);

    // 2. Collect ages
    println!("2. Collect all ages:");
    let ages = collect_keypath(people.clone(), Person::age()).unwrap();
    println!("   Ages: {:?}\n", ages);

    // 3. Partition by age
    println!("3. Partition by age (young vs experienced):");
    let (young, experienced) = partition_by_keypath(people.clone(), Person::age(), |&age| age < 30).unwrap();
    println!("   Young (< 30): {} people", young.len());
    println!("   Experienced (>= 30): {} people\n", experienced.len());

    // 4. Group by department
    println!("4. Group by department:");
    let grouped_by_dept = group_by_keypath(&people, Person::department(), |dept| dept.clone()).unwrap();
    for (dept, employees) in &grouped_by_dept {
        println!("   {}: {} employees", dept, employees.len());
        for emp in employees {
            println!("     - {}", emp.name);
        }
    }
    println!();

    // 5. Sort by salary
    println!("5. Sort by salary (ascending):");
    sort_by_keypath(&mut people, Person::salary(), |a, b| a.partial_cmp(b).unwrap()).unwrap();
    for person in &people {
        println!("   {}: ${:.0}", person.name, person.salary);
    }
    println!();

    // 6. Find person with specific age
    println!("6. Find person with age 30:");
    let found = find_by_keypath(people.clone(), Person::age(), |&age| age == 30).unwrap();
    match found {
        Some(person) => println!("   Found: {}", person.name),
        None => println!("   No person found with age 30"),
    }
    println!();

    // 7. Map keypath collection - get all salaries
    println!("7. Get all salaries:");
    let salaries = map_keypath_collection(&people, Person::salary(), |&salary| salary).unwrap();
    println!("   Salaries: {:?}\n", salaries);

    // 8. Complex operation: Find highest paid person in each department
    println!("8. Highest paid person in each department:");
    let grouped = group_by_keypath(&people, Person::department(), |dept| dept.clone()).unwrap();
    for (dept, employees) in &grouped {
        let highest_paid = employees
            .iter()
            .max_by(|a, b| a.salary.partial_cmp(&b.salary).unwrap())
            .unwrap();
        println!("   {}: {} (${:.0})", dept, highest_paid.name, highest_paid.salary);
    }
    println!();

    // 9. Filter and collect - get names of high earners
    println!("9. Names of people earning > $60,000:");
    let high_earners = filter_by_keypath(people.clone(), Person::salary(), |&salary| salary > 60000.0).unwrap();
    let high_earner_names = map_keypath_collection(&high_earners, Person::name(), |name| name.clone()).unwrap();
    println!("   High earners: {:?}\n", high_earner_names);

    // 10. Fold operation - calculate total salary
    println!("10. Calculate total salary:");
    let total_salary = fold_keypath(people.clone(), Person::salary(), 0.0, |acc, &salary| acc + salary).unwrap();
    println!("   Total salary: ${:.0}\n", total_salary);

    // 11. Average salary calculation
    println!("11. Calculate average salary:");
    let avg_salary = total_salary / people.len() as f64;
    println!("   Average salary: ${:.0}\n", avg_salary);

    // 12. Skills analysis - show skills for each person
    println!("12. Skills analysis:");
    for person in &people {
        println!("   {}: {:?}", person.name, person.skills);
    }

    // 13. Chained operations - get names of young engineers
    println!("\n13. Chained operations - Young engineers:");
    let young_people = filter_by_keypath(people.clone(), Person::age(), |&age| age < 30).unwrap();
    let young_engineers = filter_by_keypath(young_people, Person::department(), |dept| dept == "Engineering").unwrap();
    let young_engineer_names = map_keypath_collection(&young_engineers, Person::name(), |name| name.clone()).unwrap();
    println!("   Young engineers: {:?}\n", young_engineer_names);

    // 14. Zip with keypath - combine names and salaries
    println!("14. Zip names with salaries:");
    let name_salary_pairs = zip_with_keypath(
        &people, 
        &people, 
        Person::name(), 
        Person::salary(), 
        |name, &salary| format!("{}: ${:.0}", name, salary)
    ).unwrap();
    for pair in name_salary_pairs {
        println!("   {}", pair);
    }
}
