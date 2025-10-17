use rust_prelude_plus::prelude::*;
use key_paths_derive::Keypaths;

#[derive(Keypaths, Debug, Clone, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

#[test]
fn test_map_keypath() {
    let person = Person { name: "Alice".to_string(), age: 30 };
    let result = map_keypath(person, Person::name_r(), |name: &String| name.to_uppercase()).unwrap();
    assert_eq!(result, "ALICE");
}

#[test]
fn test_filter_by_keypath() {
    let people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
        Person { name: "Charlie".to_string(), age: 35 },
    ];
    
    let young_people = filter_by_keypath(people, Person::age_r(), |age: &u32| *age < 30).unwrap();
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
    
    let total_age = fold_keypath(people, Person::age_r(), 0, |acc, age: &u32| acc + *age).unwrap();
    assert_eq!(total_age, 90);
}

#[test]
fn test_find_by_keypath() {
    let people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
        Person { name: "Charlie".to_string(), age: 35 },
    ];
    
    let found = find_by_keypath(people, Person::age_r(), |age: &u32| *age == 30).unwrap();
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
    
    let ages = collect_keypath(people, Person::age_r()).unwrap();
    assert_eq!(ages, vec![30, 25, 35]);
}

#[test]
fn test_partition_by_keypath() {
    let people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
        Person { name: "Charlie".to_string(), age: 35 },
    ];
    
    let (young, old) = partition_by_keypath(people, Person::age_r(), |age: &u32| *age < 30).unwrap();
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
    
    let names = map_keypath_collection(&people, Person::name_r(), |name: &String| name.to_uppercase()).unwrap();
    assert_eq!(names, vec!["ALICE", "BOB", "CHARLIE"]);
}

#[test]
fn test_group_by_keypath() {
    let people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
        Person { name: "Charlie".to_string(), age: 35 },
    ];
    
    let grouped = group_by_keypath(&people, Person::age_r(), |&age| {
        if age < 30 { "young" } else { "adult" }
    }).unwrap();
    
    assert_eq!(grouped.len(), 2);
    assert_eq!(grouped["young"].len(), 1);
    assert_eq!(grouped["adult"].len(), 2);
    assert_eq!(grouped["young"][0].name, "Bob");
}

#[test]
fn test_sort_by_keypath() {
    let mut people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
        Person { name: "Charlie".to_string(), age: 35 },
    ];
    
    sort_by_keypath(&mut people, Person::age_r(), |a, b| a.cmp(b)).unwrap();
    assert_eq!(people[0].age, 25);
    assert_eq!(people[1].age, 30);
    assert_eq!(people[2].age, 35);
}

#[test]
fn test_zip_with_keypath() {
    let people1 = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
    ];
    
    let people2 = vec![
        Person { name: "Charlie".to_string(), age: 35 },
        Person { name: "David".to_string(), age: 28 },
    ];
    
    let combined = zip_with_keypath(&people1, &people2, Person::name_r(), Person::name_r(), |name1, name2| {
        format!("{} & {}", name1, name2)
    }).unwrap();
    
    assert_eq!(combined.len(), 2);
    assert_eq!(combined[0], "Alice & Charlie");
    assert_eq!(combined[1], "Bob & David");
}