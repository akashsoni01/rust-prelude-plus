use rust_prelude_plus::prelude::*;

#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
}

struct NameKeyPath;
impl KeyPath<Person, String> for NameKeyPath {
    fn get<'a>(&self, data: &'a Person) -> &'a String { &data.name }
    fn get_mut<'a>(&self, data: &'a mut Person) -> &'a mut String { &mut data.name }
}

struct AgeKeyPath;
impl KeyPath<Person, u32> for AgeKeyPath {
    fn get<'a>(&self, data: &'a Person) -> &'a u32 { &data.age }
    fn get_mut<'a>(&self, data: &'a mut Person) -> &'a mut u32 { &mut data.age }
}

fn main() {
    let people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
        Person { name: "Charlie".to_string(), age: 35 },
    ];
    
    // Test map_keypath
    let person = people[0].clone();
    let result = map_keypath(person, NameKeyPath, |name: &String| name.to_uppercase()).unwrap();
    println!("Uppercase name: {}", result);
    
    // Test filter_by_keypath
    let young_people = filter_by_keypath(people.clone(), AgeKeyPath, |age: &u32| *age < 30).unwrap();
    println!("Young people: {:?}", young_people);
    
    // Test fold_keypath
    let total_age = fold_keypath(people.clone(), AgeKeyPath, 0, |acc, age: &u32| acc + *age).unwrap();
    println!("Total age: {}", total_age);
    
    // Test find_by_keypath
    let found = find_by_keypath(people.clone(), AgeKeyPath, |age: &u32| *age == 30).unwrap();
    println!("Found person with age 30: {:?}", found);
    
    // Test collect_keypath
    let ages = collect_keypath(people.clone(), AgeKeyPath).unwrap();
    println!("All ages: {:?}", ages);
    
    // Test partition_by_keypath
    let (young, old) = partition_by_keypath(people.clone(), AgeKeyPath, |age: &u32| *age < 30).unwrap();
    println!("Young: {:?}, Old: {:?}", young, old);
    
    // Test map_keypath_collection
    let names = map_keypath_collection(&people, NameKeyPath, |name: &String| name.to_uppercase()).unwrap();
    println!("All names uppercase: {:?}", names);
}