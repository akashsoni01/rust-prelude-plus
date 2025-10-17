use rust_prelude_plus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
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

#[test]
fn test_map_keypath() {
    let person = Person { name: "Alice".to_string(), age: 30 };
    let result = map_keypath(person, NameKeyPath, |name: &String| name.to_uppercase()).unwrap();
    assert_eq!(result, "ALICE");
}

#[test]
fn test_filter_by_keypath() {
    let people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
        Person { name: "Charlie".to_string(), age: 35 },
    ];
    
    let young_people = filter_by_keypath(people, AgeKeyPath, |age: &u32| *age < 30).unwrap();
    assert_eq!(young_people.len(), 1);
    assert_eq!(young_people[0].name, "Bob");
}

#[test]
fn test_fold_keypath() {
    let people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
        Person { name: "Charlie".to_string(), age: 35 },
    ];
    
    let total_age = fold_keypath(people, AgeKeyPath, 0, |acc, age: &u32| acc + *age).unwrap();
    assert_eq!(total_age, 90);
}

#[test]
fn test_find_by_keypath() {
    let people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
        Person { name: "Charlie".to_string(), age: 35 },
    ];
    
    let found = find_by_keypath(people, AgeKeyPath, |age: &u32| *age == 30).unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "Alice");
}

#[test]
fn test_collect_keypath() {
    let people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
        Person { name: "Charlie".to_string(), age: 35 },
    ];
    
    let ages = collect_keypath(people, AgeKeyPath).unwrap();
    assert_eq!(ages, vec![30, 25, 35]);
}

#[test]
fn test_partition_by_keypath() {
    let people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
        Person { name: "Charlie".to_string(), age: 35 },
    ];
    
    let (young, old) = partition_by_keypath(people, AgeKeyPath, |age: &u32| *age < 30).unwrap();
    assert_eq!(young.len(), 1);
    assert_eq!(old.len(), 2);
    assert_eq!(young[0].name, "Bob");
}

#[test]
fn test_map_keypath_collection() {
    let people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
        Person { name: "Charlie".to_string(), age: 35 },
    ];
    
    let names = map_keypath_collection(&people, NameKeyPath, |name: &String| name.to_uppercase()).unwrap();
    assert_eq!(names, vec!["ALICE", "BOB", "CHARLIE"]);
}
