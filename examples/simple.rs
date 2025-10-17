use rust_prelude_plus::prelude::*;
use key_paths_derive::Keypaths;

#[derive(Keypaths, Debug, Clone)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
        Person { name: "Charlie".to_string(), age: 35 },
    ];

    // Test map_keypath
    let person = people[0].clone();
    let result = map_keypath(person, Person::name_r(), |name: &String| name.to_uppercase()).unwrap();
    println!("Uppercase name: {}", result);

    // Test filter_by_keypath
    let young_people = filter_by_keypath(people.clone(), Person::age_r(), |age: &u32| *age < 30).unwrap();
    println!("Young people: {:?}", young_people);

    // Test fold_keypath
    let total_age = fold_keypath(people.clone(), Person::age_r(), 0, |acc, age: &u32| acc + *age).unwrap();
    println!("Total age: {}", total_age);

    // Test find_by_keypath
    let found = find_by_keypath(people.clone(), Person::age_r(), |age: &u32| *age == 30).unwrap();
    println!("Found person with age 30: {:?}", found);

    // Test collect_keypath
    let ages = collect_keypath(people.clone(), Person::age_r()).unwrap();
    println!("All ages: {:?}", ages);

    // Test partition_by_keypath
    let (young, old) = partition_by_keypath(people.clone(), Person::age_r(), |age: &u32| *age < 30).unwrap();
    println!("Young: {:?}, Old: {:?}", young, old);

    // Test map_keypath_collection
    let names = map_keypath_collection(&people, Person::name_r(), |name: &String| name.to_uppercase()).unwrap();
    println!("All names uppercase: {:?}", names);

    // Test group_by_keypath
    let grouped = group_by_keypath(&people, Person::age_r(), |&age| {
        if age < 30 { "young" } else { "adult" }
    }).unwrap();
    println!("Grouped by age: {:?}", grouped);

    // Test sort_by_keypath
    let mut sorted_people = people.clone();
    sort_by_keypath(&mut sorted_people, Person::age_r(), |a, b| a.cmp(b)).unwrap();
    println!("Sorted by age: {:?}", sorted_people);

    // Test zip_with_keypath
    let people2 = vec![
        Person { name: "David".to_string(), age: 22 },
        Person { name: "Eve".to_string(), age: 28 },
    ];
    let combined = zip_with_keypath(&people, &people2, Person::name_r(), Person::name_r(), |name1, name2| {
        format!("{} & {}", name1, name2)
    }).unwrap();
    println!("Combined names: {:?}", combined);
}